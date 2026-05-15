#!/usr/bin/env bash
set -euo pipefail

required_env() {
    local name="$1"
    if [[ -z "${!name:-}" ]]; then
        echo "${name} is required" >&2
        exit 1
    fi
}

required_env AWS_REGION
required_env AWS_SPOT_SUBNET_ID
required_env AWS_SPOT_SECURITY_GROUP_ID
required_env GH_RUNNER_PAT
required_env GITHUB_REPOSITORY
required_env GITHUB_RUN_ID
required_env GITHUB_RUN_ATTEMPT

instance_type="${AWS_SPOT_INSTANCE_TYPE:-c7i.8xlarge}"
root_volume_size="${AWS_SPOT_ROOT_VOLUME_SIZE:-200}"
runner_label="xyo-${GITHUB_RUN_ID}-${GITHUB_RUN_ATTEMPT}"
runner_name="xyo-spot-${GITHUB_RUN_ID}-${GITHUB_RUN_ATTEMPT}"
repo_url="https://github.com/${GITHUB_REPOSITORY}"

registration_token="$(
    curl -fsSL -X POST \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer ${GH_RUNNER_PAT}" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        "https://api.github.com/repos/${GITHUB_REPOSITORY}/actions/runners/registration-token" \
        | jq -r '.token'
)"

if [[ -z "${registration_token}" || "${registration_token}" == "null" ]]; then
    echo "failed to obtain a GitHub runner registration token" >&2
    exit 1
fi

ami_id="${AWS_SPOT_AMI_ID:-}"
if [[ -z "${ami_id}" ]]; then
    ami_id="$(
        aws ssm get-parameter \
            --region "${AWS_REGION}" \
            --name /aws/service/canonical/ubuntu/server/24.04/stable/current/amd64/hvm/ebs-gp3/ami-id \
            --query 'Parameter.Value' \
            --output text
    )"
fi

root_device_name="$(
    aws ec2 describe-images \
        --region "${AWS_REGION}" \
        --image-ids "${ami_id}" \
        --query 'Images[0].RootDeviceName' \
        --output text
)"

user_data_file="$(mktemp)"
cat >"${user_data_file}" <<EOF
#!/usr/bin/env bash
set -euo pipefail

export DEBIAN_FRONTEND=noninteractive
shutdown -h +360 &

apt-get update -y
apt-get install -y \
  build-essential \
  ca-certificates \
  clang \
  cmake \
  curl \
  git \
  gzip \
  jq \
  make \
  ninja-build \
  perl \
  pkg-config \
  python3 \
  tar \
  unzip \
  zstd

useradd --create-home --shell /bin/bash runner || true
mkdir -p /opt/actions-runner
chown runner:runner /opt/actions-runner
cd /opt/actions-runner

runner_version="\${RUNNER_VERSION:-}"
if [[ -z "\${runner_version}" ]]; then
  runner_version="\$(curl -fsSL https://api.github.com/repos/actions/runner/releases/latest | jq -r '.tag_name' | sed 's/^v//')"
fi

runner_archive="actions-runner-linux-x64-\${runner_version}.tar.gz"
curl -fsSL -o "\${runner_archive}" "https://github.com/actions/runner/releases/download/v\${runner_version}/\${runner_archive}"
tar -xzf "\${runner_archive}"
rm -f "\${runner_archive}"

if [[ -x ./bin/installdependencies.sh ]]; then
  ./bin/installdependencies.sh
fi

chown -R runner:runner /opt/actions-runner

sudo -u runner ./config.sh \
  --unattended \
  --url "${repo_url}" \
  --token "${registration_token}" \
  --name "${runner_name}" \
  --labels "xyo-ec2-spot,${runner_label}" \
  --ephemeral \
  --work _work

set +e
sudo -u runner ./run.sh
runner_status="\$?"
set -e

sudo -u runner ./config.sh remove --unattended --token "${registration_token}" || true
shutdown -h now
exit "\${runner_status}"
EOF

block_device_mappings="$(
    jq -nc \
        --arg device "${root_device_name}" \
        --argjson size "${root_volume_size}" \
        '[{DeviceName: $device, Ebs: {VolumeSize: $size, VolumeType: "gp3", DeleteOnTermination: true}}]'
)"

market_options='{"MarketType":"spot","SpotOptions":{"SpotInstanceType":"one-time","InstanceInterruptionBehavior":"terminate"}}'

run_args=(
    --region "${AWS_REGION}"
    --image-id "${ami_id}"
    --instance-type "${instance_type}"
    --subnet-id "${AWS_SPOT_SUBNET_ID}"
    --security-group-ids "${AWS_SPOT_SECURITY_GROUP_ID}"
    --instance-market-options "${market_options}"
    --instance-initiated-shutdown-behavior terminate
    --block-device-mappings "${block_device_mappings}"
    --user-data "file://${user_data_file}"
    --tag-specifications "ResourceType=instance,Tags=[{Key=Name,Value=${runner_name}},{Key=GitHubRepository,Value=${GITHUB_REPOSITORY}},{Key=GitHubRunId,Value=${GITHUB_RUN_ID}},{Key=GitHubRunnerLabel,Value=${runner_label}}]"
    --query 'Instances[0].InstanceId'
    --output text
)

if [[ -n "${AWS_SPOT_INSTANCE_PROFILE:-}" ]]; then
    run_args+=(--iam-instance-profile "Name=${AWS_SPOT_INSTANCE_PROFILE}")
fi

instance_id="$(aws ec2 run-instances "${run_args[@]}")"
rm -f "${user_data_file}"

echo "instance_id=${instance_id}" >>"${GITHUB_OUTPUT}"
echo "runner_label=${runner_label}" >>"${GITHUB_OUTPUT}"
echo "runner_name=${runner_name}" >>"${GITHUB_OUTPUT}"

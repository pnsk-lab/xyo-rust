# AWS EC2 Spot Runner

This repository can run the Linux x64 CI job on an ephemeral EC2 Spot instance.
The workflow starts one instance, waits for it to register as a self-hosted
runner, runs the job once, and then terminates the instance.

## Required GitHub Secrets

| Secret | Purpose |
| --- | --- |
| `AWS_ROLE_TO_ASSUME` | IAM role ARN used by GitHub Actions OIDC |
| `GH_RUNNER_PAT` | Fine-grained GitHub token with repository Administration read/write permission |

## Required GitHub Variables

| Variable | Example |
| --- | --- |
| `AWS_REGION` | `ap-northeast-1` |
| `AWS_SPOT_SUBNET_ID` | `subnet-...` |
| `AWS_SPOT_SECURITY_GROUP_ID` | `sg-...` |

## IAM Permissions

The OIDC role needs enough EC2 and SSM permission to discover an Ubuntu AMI,
launch the Spot instance, inspect it, tag it, and terminate it during cleanup.

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "ec2:DescribeImages",
        "ec2:DescribeInstances",
        "ec2:RunInstances",
        "ec2:CreateTags",
        "ec2:TerminateInstances",
        "ssm:GetParameter"
      ],
      "Resource": "*"
    },
    {
      "Effect": "Allow",
      "Action": "iam:PassRole",
      "Resource": "*",
      "Condition": {
        "StringEqualsIfExists": {
          "iam:PassedToService": "ec2.amazonaws.com"
        }
      }
    }
  ]
}
```

If `AWS_SPOT_INSTANCE_PROFILE` is unset, `iam:PassRole` is not needed.

## Optional Variables

| Variable | Default |
| --- | --- |
| `AWS_SPOT_INSTANCE_TYPE` | `c7i.8xlarge` |
| `AWS_SPOT_ROOT_VOLUME_SIZE` | `200` |
| `AWS_SPOT_AMI_ID` | Latest Ubuntu 24.04 amd64 gp3 AMI from SSM |
| `AWS_SPOT_INSTANCE_PROFILE` | unset |

The security group does not need inbound rules. The runner only needs outbound
HTTPS access to GitHub, AWS APIs, package repositories, and crates.io.

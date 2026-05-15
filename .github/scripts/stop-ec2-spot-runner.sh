#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${AWS_REGION:-}" || -z "${INSTANCE_ID:-}" ]]; then
    echo "AWS_REGION and INSTANCE_ID are required" >&2
    exit 1
fi

aws ec2 terminate-instances \
    --region "${AWS_REGION}" \
    --instance-ids "${INSTANCE_ID}" \
    >/dev/null

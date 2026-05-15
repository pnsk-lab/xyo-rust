#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${GH_RUNNER_PAT:-}" || -z "${GITHUB_REPOSITORY:-}" || -z "${RUNNER_LABEL:-}" ]]; then
    echo "GH_RUNNER_PAT, GITHUB_REPOSITORY, and RUNNER_LABEL are required" >&2
    exit 1
fi

deadline="$((SECONDS + ${RUNNER_WAIT_SECONDS:-900}))"

while (( SECONDS < deadline )); do
    runner="$(
        curl -fsSL \
            -H "Accept: application/vnd.github+json" \
            -H "Authorization: Bearer ${GH_RUNNER_PAT}" \
            -H "X-GitHub-Api-Version: 2022-11-28" \
            "https://api.github.com/repos/${GITHUB_REPOSITORY}/actions/runners?per_page=100" \
            | jq -r --arg label "${RUNNER_LABEL}" '
                .runners[]
                | select(.status == "online")
                | select(any(.labels[]; .name == $label))
                | .name
            ' \
            | head -n 1
    )"

    if [[ -n "${runner}" ]]; then
        echo "runner ${runner} is online with label ${RUNNER_LABEL}"
        exit 0
    fi

    sleep 10
done

echo "timed out waiting for EC2 Spot runner with label ${RUNNER_LABEL}" >&2
exit 1

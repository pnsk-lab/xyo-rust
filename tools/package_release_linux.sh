#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RELEASE_TARGET="${XYO_RELEASE_TARGET:-linux}"
BIN_PATH="${XYO_RELEASE_BIN:-${ROOT_DIR}/target/release/xyo}"
ICU_RUNTIME_DIR="${XYO_ICU_RUNTIME_DIR:-${ROOT_DIR}/target/icu-runtime}"
STAGE_DIR="${XYO_RELEASE_STAGE_DIR:-${ROOT_DIR}/target/release/xyo-${RELEASE_TARGET}}"
ARCHIVE_PATH="${XYO_RELEASE_ARCHIVE_PATH:-${ROOT_DIR}/target/release/xyo-${RELEASE_TARGET}.tar.gz}"

usage() {
    cat <<EOF
Usage: $0

Package the Linux release artifact with bundled ICU runtime libraries.

Environment:
  XYO_RELEASE_BIN     Path to the built xyo binary
  XYO_RELEASE_TARGET  Target suffix for the staged directory/archive
  XYO_RELEASE_STAGE_DIR     Staging directory override
  XYO_RELEASE_ARCHIVE_PATH  Archive path override
  XYO_ICU_RUNTIME_DIR ICU runtime prefix (default: target/icu-runtime)
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    usage
    exit 0
fi

if [[ ! -f "${BIN_PATH}" ]]; then
    echo "release binary not found: ${BIN_PATH}" >&2
    exit 1
fi

if [[ ! -d "${ICU_RUNTIME_DIR}/lib" ]]; then
    echo "ICU runtime lib directory not found: ${ICU_RUNTIME_DIR}/lib" >&2
    exit 1
fi

copy_runtime_lib() {
    local requested_name="$1"
    local source=""

    source="$(find "${ICU_RUNTIME_DIR}/lib" -maxdepth 1 \( -type f -o -type l \) -name "${requested_name}*" | sort | head -n 1)"
    if [[ -z "${source}" ]]; then
        echo "required runtime library not found: ${requested_name}" >&2
        exit 1
    fi

    cp -L "${source}" "${STAGE_DIR}/lib/${requested_name}"
}

rm -rf "${STAGE_DIR}"
mkdir -p "${STAGE_DIR}/lib"

install -m 0755 "${BIN_PATH}" "${STAGE_DIR}/xyo"
copy_runtime_lib "libicudata.so"
copy_runtime_lib "libicuuc.so"

tar -C "$(dirname "${STAGE_DIR}")" -czf "${ARCHIVE_PATH}" "$(basename "${STAGE_DIR}")"

echo "packaged Linux release artifact at ${ARCHIVE_PATH}"

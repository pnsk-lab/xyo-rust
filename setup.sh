#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

pick_tool() {
    local preferred="${1:-}"
    shift
    if [[ -n "${preferred}" ]] && command -v "${preferred}" >/dev/null 2>&1; then
        printf '%s\n' "${preferred}"
        return 0
    fi

    local candidate
    for candidate in "$@"; do
        if command -v "${candidate}" >/dev/null 2>&1; then
            printf '%s\n' "${candidate}"
            return 0
        fi
    done

    return 1
}

usage() {
    cat <<EOF
Usage: $0 [--clean-icu] [--skip-icu] [--skip-check]

Default flow:
  1. Build vendored ICU static archives into bitcodes/c/lib/icu-prebuilt
  2. Run cargo build --release
  3. Run tools/check_to_lower_native.sh

Options:
  --clean-icu   Rebuild prebuilt ICU from scratch
  --skip-icu    Skip prebuilt ICU build
  --skip-check  Skip native to_lower.ll check

Environment:
  CLANG         Preferred C compiler
  CLANGXX       Preferred C++ compiler
  LLVM_CONFIG_PATH  Optional LLVM config path for build.rs
  XYO_ICU_ROOT  ICU source root (default: bitcodes/c/lib/icu)
  XYO_ICU_PREBUILT_DIR  Prebuilt ICU install root (default: bitcodes/c/lib/icu-prebuilt)
  XYO_EMBED_ICU_BITCODE=1  Opt in to the heavy self-contained to_lower.bc build
EOF
}

CLEAN_ICU=0
SKIP_ICU=0
SKIP_CHECK=0

while [[ $# -gt 0 ]]; do
    case "$1" in
    --clean-icu)
        CLEAN_ICU=1
        shift
        ;;
    --skip-icu)
        SKIP_ICU=1
        shift
        ;;
    --skip-check)
        SKIP_CHECK=1
        shift
        ;;
    --help|-h)
        usage
        exit 0
        ;;
    *)
        echo "Unknown argument: $1" >&2
        usage >&2
        exit 1
        ;;
    esac
done

CLANG="${CLANG:-$(pick_tool "" clang-23 clang-22 clang-21 clang)}"
CLANGXX="${CLANGXX:-$(pick_tool "" clang++-23 clang++-22 clang++-21 clang++)}"
DEFAULT_XYO_ICU_ROOT="${ROOT_DIR}/bitcodes/c/lib/icu"
XYO_ICU_PREBUILT_DIR="${XYO_ICU_PREBUILT_DIR:-${ROOT_DIR}/bitcodes/c/lib/icu-prebuilt}"
export CLANG
export CLANGXX
export XYO_ICU_PREBUILT_DIR

if [[ -n "${XYO_ICU_ROOT:-}" ]]; then
    export XYO_ICU_ROOT
elif [[ -d "${DEFAULT_XYO_ICU_ROOT}" ]]; then
    XYO_ICU_ROOT="${DEFAULT_XYO_ICU_ROOT}"
    export XYO_ICU_ROOT
fi

echo "using CLANG=${CLANG}"
echo "using CLANGXX=${CLANGXX}"
if [[ -n "${XYO_ICU_ROOT:-}" ]]; then
    echo "using XYO_ICU_ROOT=${XYO_ICU_ROOT}"
else
    echo "using XYO_ICU_ROOT=<unset>"
fi
echo "using XYO_ICU_PREBUILT_DIR=${XYO_ICU_PREBUILT_DIR}"

if [[ ${SKIP_ICU} -eq 0 ]]; then
    ICU_ARGS=()
    if [[ ${CLEAN_ICU} -eq 1 ]]; then
        ICU_ARGS+=(--clean)
    fi

    "${ROOT_DIR}/tools/build_icu_prebuilt.sh" "${ICU_ARGS[@]}"
fi

(
    cd "${ROOT_DIR}"
    cargo build --release
)

if [[ ${SKIP_CHECK} -eq 0 ]]; then
    (
        cd "${ROOT_DIR}"
        "${ROOT_DIR}/tools/check_to_lower_native.sh"
    )
fi

echo "setup completed"

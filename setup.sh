#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

pick_tool() {
    local preferred="${1:-}"
    shift
    local candidate resolved

    if [[ -n "${preferred}" ]]; then
        resolved="$(command -v "${preferred}" 2>/dev/null || true)"
        if [[ -n "${resolved}" && "${resolved}" == */* ]]; then
            printf '%s\n' "${resolved}"
            return 0
        fi
        return 1
    fi

    for candidate in "$@"; do
        resolved="$(command -v "${candidate}" 2>/dev/null || true)"
        if [[ -n "${resolved}" && "${resolved}" == */* ]]; then
            printf '%s\n' "${resolved}"
            return 0
        fi
    done

    return 1
}

configure_macos_sdkroot() {
    if [[ "$(uname -s)" != "Darwin" ]]; then
        return 0
    fi

    if [[ -n "${SDKROOT:-}" ]]; then
        return 0
    fi

    if ! command -v xcrun >/dev/null 2>&1; then
        echo "xcrun is required on macOS to locate the active SDK" >&2
        return 1
    fi

    local sdkroot
    sdkroot="$(xcrun --sdk macosx --show-sdk-path 2>/dev/null || true)"
    if [[ -z "${sdkroot}" || ! -d "${sdkroot}" ]]; then
        echo "failed to determine macOS SDKROOT; install Xcode Command Line Tools or set SDKROOT" >&2
        return 1
    fi

    SDKROOT="${sdkroot}"
    export SDKROOT
    return 0
}

usage() {
    cat <<EOF
Usage: $0 [--clean-icu] [--skip-icu] [--force-fetch-icu]

Default flow:
  1. Fetch vendored ICU into bitcodes/c/lib/icu if missing
  2. Build vendored ICU static archives into bitcodes/c/lib/icu-prebuilt
  3. Run cargo build --release

Options:
  --clean-icu       Rebuild prebuilt ICU from scratch
  --skip-icu        Skip ICU fetch + prebuilt build
  --force-fetch-icu Re-fetch vendored ICU source archive

Environment:
  CLANG             Preferred C compiler
  CLANGXX           Preferred C++ compiler
  SDKROOT           macOS SDK root (optional; auto-detected if unset)
  LLVM_CONFIG_PATH  Optional LLVM config path for build.rs
  XYO_ICU_ROOT      ICU source root (default: bitcodes/c/lib/icu)
  XYO_ICU_PREBUILT_DIR  Prebuilt ICU install root (default: bitcodes/c/lib/icu-prebuilt)
    XYO_ICU_VERSION   ICU version to fetch when vendoring (default: 78.3)
  XYO_ICU_URL       Explicit ICU source archive URL override
  XYO_ICU_ARCHIVE   Cache path for downloaded ICU archive
EOF
}

CLEAN_ICU=0
SKIP_ICU=0
SKIP_CHECK=0
FORCE_FETCH_ICU=0

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
    --force-fetch-icu)
        FORCE_FETCH_ICU=1
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

fetch_icu_if_needed() {
    local icu_root="$1"
    local force_fetch="$2"

    if [[ -f "${icu_root}/source/common/unicode/ucasemap.h" ]]; then
        if [[ "${force_fetch}" -eq 0 ]]; then
            return 0
        fi
        rm -rf "${icu_root}"
    fi

    local icu_version="${XYO_ICU_VERSION:-78.3}"
    local icu_version_tag="${icu_version}"
    local icu_url="${XYO_ICU_URL:-https://github.com/unicode-org/icu/releases/download/release-${icu_version_tag}/icu4c-${icu_version_tag}-sources.tgz}"
    local archive_name
    archive_name="$(basename "${icu_url}")"
    local archive_path="${XYO_ICU_ARCHIVE:-${ROOT_DIR}/target/${archive_name}}"
    local parent_dir
    parent_dir="$(dirname "${icu_root}")"
    local tmp_extract_dir="${ROOT_DIR}/target/icu-fetch"
    local downloader=""

    if ! downloader="$(pick_tool "" curl wget)"; then
        echo "curl or wget is required to fetch ICU source archive" >&2
        exit 1
    fi

    mkdir -p "${parent_dir}" "$(dirname "${archive_path}")"
    rm -rf "${tmp_extract_dir}"
    mkdir -p "${tmp_extract_dir}"

    echo "fetching ICU ${icu_version} from ${icu_url}"
    if [[ "${downloader##*/}" == "curl" ]]; then
        curl -L --fail --output "${archive_path}" "${icu_url}"
    else
        wget -O "${archive_path}" "${icu_url}"
    fi

    tar -xzf "${archive_path}" -C "${tmp_extract_dir}"

    local extracted_root=""
    local candidate
    for candidate in "${tmp_extract_dir}"/*; do
        if [[ -d "${candidate}" && -f "${candidate}/source/common/unicode/ucasemap.h" ]]; then
            extracted_root="${candidate}"
            break
        fi
    done

    if [[ -z "${extracted_root}" ]]; then
        echo "failed to locate extracted ICU source tree in ${tmp_extract_dir}" >&2
        exit 1
    fi

    rm -rf "${icu_root}"
    mv "${extracted_root}" "${icu_root}"
    rm -rf "${tmp_extract_dir}"
}

CLANG="${CLANG:-}"
CLANGXX="${CLANGXX:-}"
if ! CLANG="$(pick_tool "${CLANG}" clang-23 clang-22 clang-21 clang)"; then
    echo "unable to find a usable C compiler; set CLANG explicitly" >&2
    exit 1
fi
if ! CLANGXX="$(pick_tool "${CLANGXX}" clang++-23 clang++-22 clang++-21 clang++)"; then
    echo "unable to find a usable C++ compiler; set CLANGXX explicitly" >&2
    exit 1
fi
DEFAULT_XYO_ICU_ROOT="${ROOT_DIR}/bitcodes/c/lib/icu"
XYO_ICU_PREBUILT_DIR="${XYO_ICU_PREBUILT_DIR:-${ROOT_DIR}/bitcodes/c/lib/icu-prebuilt}"
export CLANG
export CLANGXX
export XYO_ICU_PREBUILT_DIR
if ! configure_macos_sdkroot; then
    exit 1
fi

if [[ -n "${XYO_ICU_ROOT:-}" ]]; then
    export XYO_ICU_ROOT
elif [[ -d "${DEFAULT_XYO_ICU_ROOT}" ]]; then
    XYO_ICU_ROOT="${DEFAULT_XYO_ICU_ROOT}"
    export XYO_ICU_ROOT
fi

if [[ ${SKIP_ICU} -eq 0 ]]; then
    fetch_icu_if_needed "${XYO_ICU_ROOT:-${DEFAULT_XYO_ICU_ROOT}}" "${FORCE_FETCH_ICU}"

    if [[ -z "${XYO_ICU_ROOT:-}" ]]; then
        XYO_ICU_ROOT="${DEFAULT_XYO_ICU_ROOT}"
        export XYO_ICU_ROOT
    fi

    if [[ ${CLEAN_ICU} -eq 1 ]]; then
        "${ROOT_DIR}/tools/build_icu_prebuilt.sh" --clean
    else
        "${ROOT_DIR}/tools/build_icu_prebuilt.sh"
    fi
fi

echo "using CLANG=${CLANG}"
echo "using CLANGXX=${CLANGXX}"
if [[ -n "${SDKROOT:-}" ]]; then
    echo "using SDKROOT=${SDKROOT}"
fi
if [[ -n "${XYO_ICU_ROOT:-}" ]]; then
    echo "using XYO_ICU_ROOT=${XYO_ICU_ROOT}"
else
    echo "using XYO_ICU_ROOT=<unset>"
fi
echo "using XYO_ICU_PREBUILT_DIR=${XYO_ICU_PREBUILT_DIR}"

(
    cd "${ROOT_DIR}"
    cargo build --release
)

echo "setup completed"

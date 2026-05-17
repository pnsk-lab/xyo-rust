#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ICU_ROOT="${XYO_ICU_ROOT:-${ROOT_DIR}/bitcodes/c/lib/icu}"
ICU_SOURCE_DIR="${ICU_ROOT}/source"
BUILD_DIR="${XYO_ICU_RUNTIME_BUILD_DIR:-${ROOT_DIR}/target/icu-runtime-build}"
PREFIX_DIR="${XYO_ICU_RUNTIME_DIR:-${ROOT_DIR}/target/icu-runtime}"

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
Usage: $0 [--clean] [--jobs N]

Build vendored ICU as shared runtime libraries under:
  ${PREFIX_DIR}

Environment:
  XYO_ICU_ROOT         ICU source root      (default: bitcodes/c/lib/icu)
  XYO_ICU_RUNTIME_DIR  Install prefix       (default: target/icu-runtime)
  XYO_ICU_RUNTIME_BUILD_DIR  Build directory (default: target/icu-runtime-build)
  CLANG                Preferred C compiler (default: clang-23/22/21/clang)
  CLANGXX              Preferred C++ compiler
  SDKROOT              macOS SDK root (optional; auto-detected if unset)
EOF
}

normalize_shell_helpers() {
    python3 - <<PY
from pathlib import Path

root = Path(r"${ICU_SOURCE_DIR}")
names = {
    "configure",
    "config.sub",
    "config.guess",
    "install-sh",
    "mkinstalldirs",
    "runConfigureICU",
}

for path in root.rglob("*"):
    if path.is_file() and path.name in names:
        data = path.read_bytes()
        normalized = data.replace(b"\r\n", b"\n")
        if normalized != data:
            path.write_bytes(normalized)
PY
}

JOBS=""
CLEAN=0

while [[ $# -gt 0 ]]; do
    case "$1" in
    --clean)
        CLEAN=1
        shift
        ;;
    --jobs|-j)
        JOBS="${2:-}"
        if [[ -z "${JOBS}" ]]; then
            echo "--jobs requires a value" >&2
            exit 1
        fi
        shift 2
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

if [[ ! -d "${ICU_SOURCE_DIR}" ]]; then
    echo "ICU source directory not found: ${ICU_SOURCE_DIR}" >&2
    exit 1
fi

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
if ! MAKE_TOOL="$(pick_tool "" make gmake)"; then
    echo "unable to find make or gmake" >&2
    exit 1
fi

if ! configure_macos_sdkroot; then
    exit 1
fi

if [[ -z "${JOBS}" ]]; then
    JOBS="$(getconf _NPROCESSORS_ONLN 2>/dev/null || printf '4\n')"
fi

if [[ ${CLEAN} -eq 1 ]]; then
    rm -rf "${BUILD_DIR}" "${PREFIX_DIR}"
fi

mkdir -p "${BUILD_DIR}"

echo "using CLANG=${CLANG}"
echo "using CLANGXX=${CLANGXX}"
echo "using MAKE=${MAKE_TOOL}"
echo "using JOBS=${JOBS}"
echo "using ICU_ROOT=${ICU_ROOT}"
echo "using XYO_ICU_RUNTIME_BUILD_DIR=${BUILD_DIR}"
echo "using XYO_ICU_RUNTIME_DIR=${PREFIX_DIR}"
if [[ -n "${SDKROOT:-}" ]]; then
    echo "using SDKROOT=${SDKROOT}"
fi

if [[ ! -f "${BUILD_DIR}/Makefile" ]]; then
    normalize_shell_helpers

    NORMALIZED_CONFIGURE="${BUILD_DIR}/configure.lf.sh"
    python3 - <<PY
from pathlib import Path
src = Path(r"${ICU_SOURCE_DIR}/configure")
dst = Path(r"${NORMALIZED_CONFIGURE}")
dst.write_bytes(src.read_bytes().replace(b"\r\n", b"\n"))
PY
    chmod +x "${NORMALIZED_CONFIGURE}"

    if ! (
        cd "${BUILD_DIR}"
        CC="${CLANG}" \
        CXX="${CLANGXX}" \
        "${NORMALIZED_CONFIGURE}" \
            --srcdir="${ICU_SOURCE_DIR}" \
            --prefix="${PREFIX_DIR}" \
            --enable-shared \
            --disable-static \
            --disable-tests \
            --disable-samples \
            --disable-extras \
            --disable-tools \
            --with-data-packaging=archive
    ); then
        if [[ -f "${BUILD_DIR}/config.log" ]]; then
            echo "ICU configure failed; tail of ${BUILD_DIR}/config.log:" >&2
            tail -n 120 "${BUILD_DIR}/config.log" >&2 || true
        fi
        exit 1
    fi
fi

"${MAKE_TOOL}" -C "${BUILD_DIR}" -j "${JOBS}"
"${MAKE_TOOL}" -C "${BUILD_DIR}" install

echo "installed ICU runtime libraries into ${PREFIX_DIR}"
find "${PREFIX_DIR}/lib" -maxdepth 1 \( -type f -o -type l \) \( -name 'libicuuc.so*' -o -name 'libicudata.so*' \) | sort

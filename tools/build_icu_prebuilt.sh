#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ICU_ROOT="${XYO_ICU_ROOT:-${ROOT_DIR}/bitcodes/c/lib/icu}"
ICU_SOURCE_DIR="${ICU_ROOT}/source"
BUILD_DIR="${XYO_ICU_PREBUILT_BUILD_DIR:-${ROOT_DIR}/target/icu-prebuilt-build}"
PREFIX_DIR="${XYO_ICU_PREBUILT_DIR:-${ROOT_DIR}/bitcodes/c/lib/icu-prebuilt}"

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
Usage: $0 [--clean] [--jobs N]

Build vendored ICU as static archives under:
  ${PREFIX_DIR}

Environment:
  XYO_ICU_ROOT      ICU source root      (default: bitcodes/c/lib/icu)
  XYO_ICU_PREBUILT_DIR  Install prefix   (default: bitcodes/c/lib/icu-prebuilt)
  XYO_ICU_PREBUILT_BUILD_DIR  Build directory (default: target/icu-prebuilt-build)
  CLANG      Preferred C compiler    (default: clang-23/22/21/clang)
  CLANGXX    Preferred C++ compiler  (default: clang++-23/22/21/clang++)
  AR         Archiver                (default: ar)
  RANLIB     Ranlib                  (default: ranlib)
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

CLANG="${CLANG:-$(pick_tool "" clang-23 clang-22 clang-21 clang)}"
CLANGXX="${CLANGXX:-$(pick_tool "" clang++-23 clang++-22 clang++-21 clang++)}"
AR_TOOL="${AR:-$(pick_tool "" ar llvm-ar-23 llvm-ar-22 llvm-ar-21 llvm-ar)}"
RANLIB_TOOL="${RANLIB:-$(pick_tool "" ranlib llvm-ranlib-23 llvm-ranlib-22 llvm-ranlib-21 llvm-ranlib)}"
MAKE_TOOL="$(pick_tool "" make gmake)"

if [[ -z "${JOBS}" ]]; then
    JOBS="$(getconf _NPROCESSORS_ONLN 2>/dev/null || printf '4\n')"
fi

if [[ ${CLEAN} -eq 1 ]]; then
    rm -rf "${BUILD_DIR}" "${PREFIX_DIR}"
fi

mkdir -p "${BUILD_DIR}"

echo "using CLANG=${CLANG}"
echo "using CLANGXX=${CLANGXX}"
echo "using AR=${AR_TOOL}"
echo "using RANLIB=${RANLIB_TOOL}"
echo "using MAKE=${MAKE_TOOL}"
echo "using JOBS=${JOBS}"
echo "using ICU_ROOT=${ICU_ROOT}"
echo "using ICU_PREBUILT_BUILD_DIR=${BUILD_DIR}"
echo "using ICU_PREBUILT_DIR=${PREFIX_DIR}"

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

    (
        cd "${BUILD_DIR}"
        CC="${CLANG}" \
        CXX="${CLANGXX}" \
        AR="${AR_TOOL}" \
        RANLIB="${RANLIB_TOOL}" \
        "${NORMALIZED_CONFIGURE}" \
            --srcdir="${ICU_SOURCE_DIR}" \
            --prefix="${PREFIX_DIR}" \
            --disable-shared \
            --enable-static \
            --disable-tests \
            --disable-samples \
            --disable-extras \
            --disable-tools \
            --with-data-packaging=archive
    )
fi

"${MAKE_TOOL}" -C "${BUILD_DIR}" -j "${JOBS}"
"${MAKE_TOOL}" -C "${BUILD_DIR}" install

echo "installed ICU static archives into ${PREFIX_DIR}"
find "${PREFIX_DIR}" -maxdepth 2 -type f -name 'libicu*.a' | sort

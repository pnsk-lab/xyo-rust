#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="${ROOT_DIR}/target/to-lower-native-check"
IR_INPUT="${1:-${ROOT_DIR}/bitcodes/ll/to_lower.ll}"
HARNESS_C="${ROOT_DIR}/tools/to_lower_harness.c"

mkdir -p "${BUILD_DIR}"

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

CLANG="${CLANG:-$(pick_tool "" clang-23 clang-22 clang-21 clang)}"
CLANGXX="${CLANGXX:-$(pick_tool "" clang++-23 clang++-22 clang++-21 clang++)}"

if [[ ! -f "${IR_INPUT}" ]]; then
    echo "IR not found: ${IR_INPUT}" >&2
    exit 1
fi

if [[ ! -f "${HARNESS_C}" ]]; then
    echo "Harness not found: ${HARNESS_C}" >&2
    exit 1
fi

IR_OBJ="${BUILD_DIR}/to_lower_ir.o"
HARNESS_OBJ="${BUILD_DIR}/to_lower_harness.o"
OUT_BIN="${BUILD_DIR}/to_lower_harness"

"${CLANGXX}" -O0 -c -x ir "${IR_INPUT}" -o "${IR_OBJ}"
"${CLANG}" -O0 -c "${HARNESS_C}" -o "${HARNESS_OBJ}"

link_args=(
    "${IR_OBJ}"
    "${HARNESS_OBJ}"
    -o "${OUT_BIN}"
    -ldl
    -lm
    -pthread
)

DEFAULT_ICU_NATIVE_LIB_DIR="${XYO_ICU_PREBUILT_DIR:-${ROOT_DIR}/bitcodes/c/lib/icu-prebuilt}"
ICU_NATIVE_LIB_DIR="${XYO_ICU_NATIVE_LIB_DIR:-${DEFAULT_ICU_NATIVE_LIB_DIR}}"
if [[ -z "${ICU_NATIVE_LIB_DIR}" ]] && [[ -d "${DEFAULT_ICU_NATIVE_LIB_DIR}" ]]; then
    ICU_NATIVE_LIB_DIR="${DEFAULT_ICU_NATIVE_LIB_DIR}"
fi
if [[ -n "${ICU_NATIVE_LIB_DIR}" ]]; then
    if [[ ! -d "${ICU_NATIVE_LIB_DIR}" ]]; then
        echo "XYO_ICU_NATIVE_LIB_DIR does not exist: ${ICU_NATIVE_LIB_DIR}" >&2
        exit 1
    fi

    ICU_ARCHIVE_DIR="${ICU_NATIVE_LIB_DIR}"
    if [[ -d "${ICU_NATIVE_LIB_DIR}/lib" ]]; then
        ICU_ARCHIVE_DIR="${ICU_NATIVE_LIB_DIR}/lib"
    fi

    shopt -s nullglob
    archives=(
        "${ICU_ARCHIVE_DIR}"/libicu*.a
        "${ICU_ARCHIVE_DIR}"/libicudata.a
        "${ICU_ARCHIVE_DIR}"/libicuuc.a
        "${ICU_ARCHIVE_DIR}"/libicui18n.a
    )
    shopt -u nullglob

    unique_archives=()
    declare -A seen=()
    for archive in "${archives[@]}"; do
        if [[ -f "${archive}" ]] && [[ -z "${seen[${archive}]:-}" ]]; then
            unique_archives+=("${archive}")
            seen["${archive}"]=1
        fi
    done

    if [[ ${#unique_archives[@]} -eq 0 ]]; then
        echo "No ICU static archives found in ${ICU_ARCHIVE_DIR}" >&2
        exit 1
    fi

    link_args+=(-Wl,--start-group)
    link_args+=("${unique_archives[@]}")
    link_args+=(-Wl,--end-group)
fi

echo "using CLANG=${CLANG}"
echo "using CLANGXX=${CLANGXX}"
if [[ -n "${ICU_NATIVE_LIB_DIR}" ]]; then
    echo "using ICU_NATIVE_LIB_DIR=${ICU_NATIVE_LIB_DIR}"
fi

"${CLANGXX}" "${link_args[@]}"
"${OUT_BIN}"

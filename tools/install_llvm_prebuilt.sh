#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LLVM_VERSION="${LLVM_VERSION:-21.1.8}"
LLVM_INSTALL_DIR="${LLVM_INSTALL_DIR:-${ROOT_DIR}/.llvm/$(uname -s)-$(uname -m)}"

llvm_library_path() {
    if [[ -n "${LD_LIBRARY_PATH:-}" ]]; then
        printf '%s:%s\n' "${LLVM_INSTALL_DIR}/lib" "${LD_LIBRARY_PATH}"
    else
        printf '%s\n' "${LLVM_INSTALL_DIR}/lib"
    fi
}

installed_llvm_config() {
    if [[ -x "${LLVM_INSTALL_DIR}/bin/llvm-config" ]]; then
        printf '%s\n' "${LLVM_INSTALL_DIR}/bin/llvm-config"
        return 0
    fi

    if [[ -x "${LLVM_INSTALL_DIR}/bin/llvm-config.exe" ]]; then
        printf '%s\n' "${LLVM_INSTALL_DIR}/bin/llvm-config.exe"
        return 0
    fi

    return 1
}

installed_clang() {
    local candidate
    for candidate in clang clang-21 clang.exe; do
        if [[ -x "${LLVM_INSTALL_DIR}/bin/${candidate}" ]]; then
            printf '%s\n' "${LLVM_INSTALL_DIR}/bin/${candidate}"
            return 0
        fi
    done

    return 1
}

verify_existing_install() {
    local llvm_config="$1"
    local clang="$2"
    local lib_path
    lib_path="$(llvm_library_path)"

    env LD_LIBRARY_PATH="${lib_path}" "${llvm_config}" --version >/dev/null 2>&1 || return 1

    if [[ -n "${clang}" ]]; then
        env LD_LIBRARY_PATH="${lib_path}" "${clang}" --version >/dev/null 2>&1 || return 1
    fi
}

existing_llvm_config="$(installed_llvm_config || true)"
if [[ -n "${existing_llvm_config}" ]]; then
    existing_clang="$(installed_clang || true)"
    if verify_existing_install "${existing_llvm_config}" "${existing_clang}"; then
        echo "LLVM already installed at ${LLVM_INSTALL_DIR}"
        exit 0
    fi

    echo "LLVM install at ${LLVM_INSTALL_DIR} is incomplete; reinstalling" >&2
    rm -rf "${LLVM_INSTALL_DIR}"
fi

tmp_dir="$(mktemp -d)"
cleanup() {
    rm -rf "${tmp_dir}"
}
trap cleanup EXIT

run_as_root() {
    if [[ "$(id -u)" -eq 0 ]]; then
        "$@"
        return 0
    fi

    if command -v sudo >/dev/null 2>&1; then
        sudo "$@"
        return 0
    fi

    "$@"
}

copy_tree() {
    local source_root="$1"
    local destination_root="$2"

    rm -rf "${destination_root}"
    mkdir -p "${destination_root}"
    cp -a "${source_root}/." "${destination_root}/"

    if [[ ! -x "${destination_root}/bin/llvm-config" && ! -x "${destination_root}/bin/llvm-config.exe" ]]; then
        echo "failed to install LLVM into ${destination_root}" >&2
        exit 1
    fi
}

copy_resolved_include_dir() {
    local source_dir="$1"
    local include_name="$2"

    if [[ -z "${source_dir}" || ! -d "${source_dir}" ]]; then
        return 0
    fi

    rm -rf "${LLVM_INSTALL_DIR}/include/${include_name}"
    mkdir -p "${LLVM_INSTALL_DIR}/include/${include_name}"
    cp -a "${source_dir}/." "${LLVM_INSTALL_DIR}/include/${include_name}/"
}

copy_runtime_dependencies() {
    local binary="$1"
    local dependency

    if ! command -v ldd >/dev/null 2>&1 || [[ ! -x "${binary}" ]]; then
        return 0
    fi

    while IFS= read -r dependency; do
        case "${dependency}" in
        */libLLVM*.so*|*/libclang*.so*)
            cp -L "${dependency}" "${LLVM_INSTALL_DIR}/lib/"
            ;;
        esac
    done < <(
        ldd "${binary}" 2>/dev/null |
            awk '/=> \// { print $(NF - 1); next } /^[[:space:]]*\// { print $1 }'
    )
}

copy_tool_runtime_dependencies() {
    local tool

    mkdir -p "${LLVM_INSTALL_DIR}/lib"
    for tool in clang clang-21 clang++ clang++-21 llvm-config llvm-config-21; do
        copy_runtime_dependencies "${LLVM_INSTALL_DIR}/bin/${tool}"
    done
}

install_from_archive() {
    local asset_name="$1"
    local archive_url="https://github.com/llvm/llvm-project/releases/download/llvmorg-${LLVM_VERSION}/${asset_name}"
    local archive_path="${tmp_dir}/${asset_name}"
    local extract_dir="${tmp_dir}/extract"

    mkdir -p "${extract_dir}"
    echo "downloading ${asset_name}"
    curl --proto '=https' --tlsv1.2 -fsSL --output "${archive_path}" "${archive_url}"

    case "${asset_name}" in
    *.tar.xz)
        tar -xJf "${archive_path}" -C "${extract_dir}"
        ;;
    *.tar.gz)
        tar -xzf "${archive_path}" -C "${extract_dir}"
        ;;
    *)
        tar -xf "${archive_path}" -C "${extract_dir}"
        ;;
    esac

    local llvm_config_path
    llvm_config_path="$(find "${extract_dir}" -type f \( -name llvm-config -o -name llvm-config.exe \) | head -n 1)"
    if [[ -z "${llvm_config_path}" ]]; then
        echo "failed to locate llvm-config in ${asset_name}" >&2
        exit 1
    fi

    local source_root
    source_root="$(dirname "$(dirname "${llvm_config_path}")")"
    copy_tree "${source_root}" "${LLVM_INSTALL_DIR}"
}

install_from_brew() {
    if ! command -v brew >/dev/null 2>&1; then
        echo "brew is required to install LLVM on macOS" >&2
        exit 1
    fi

    brew install llvm@21
    copy_tree "$(brew --prefix llvm@21)" "${LLVM_INSTALL_DIR}"
}

install_from_apt_llvm() {
    local os_release="/etc/os-release"
    local codename="noble"
    if [[ -f "${os_release}" ]]; then
        # shellcheck disable=SC1091
        . "${os_release}"
        codename="${VERSION_CODENAME:-${codename}}"
    fi

    run_as_root env DEBIAN_FRONTEND=noninteractive apt-get update -q -y
    run_as_root env DEBIAN_FRONTEND=noninteractive apt-get install -q -y ca-certificates curl gnupg
    run_as_root install -d -m 0755 /usr/share/keyrings

    local keyring="/usr/share/keyrings/apt.llvm.org.gpg"
    if [[ ! -f "${keyring}" ]]; then
        curl -fsSL https://apt.llvm.org/llvm-snapshot.gpg.key | run_as_root gpg --dearmor -o "${keyring}"
    fi

    local sources_list="/etc/apt/sources.list.d/apt.llvm.org.list"
    if [[ ! -f "${sources_list}" ]]; then
        echo "deb [signed-by=${keyring}] http://apt.llvm.org/${codename}/ llvm-toolchain-${codename}-21 main" | run_as_root tee "${sources_list}" >/dev/null
    fi

    run_as_root env DEBIAN_FRONTEND=noninteractive apt-get update -q -y
    run_as_root env DEBIAN_FRONTEND=noninteractive apt-get install -q -y \
        clang-21 \
        libffi-dev \
        libpolly-21-dev \
        libxml2-dev \
        libzstd-dev \
        llvm-21 \
        llvm-21-dev \
        zlib1g-dev

    local llvm_include_dir
    local llvm_c_include_dir
    llvm_include_dir="$(readlink -f "$(llvm-config-21 --includedir)/llvm" 2>/dev/null || true)"
    llvm_c_include_dir="$(readlink -f "$(llvm-config-21 --includedir)/llvm-c" 2>/dev/null || true)"

    local source_root
    source_root="$(llvm-config-21 --prefix)"
    copy_tree "${source_root}" "${LLVM_INSTALL_DIR}"
    copy_tool_runtime_dependencies
    rm -rf "${LLVM_INSTALL_DIR}/build"

    # Debian's LLVM headers are usually symlinked out of the LLVM prefix.
    # Make the copied install self-contained so llvm-sys can find them.
    copy_resolved_include_dir "${llvm_include_dir}" "llvm"
    copy_resolved_include_dir "${llvm_c_include_dir}" "llvm-c"
}

case "$(uname -s)-$(uname -m)" in
Linux-x86_64|Linux-amd64)
    install_from_archive "LLVM-${LLVM_VERSION}-Linux-X64.tar.xz"
    ;;
Linux-aarch64|Linux-arm64)
    install_from_apt_llvm
    ;;
Linux-armv7l|Linux-armv7*)
    install_from_archive "clang+llvm-${LLVM_VERSION}-armv7a-linux-gnueabihf.tar.gz"
    ;;
Linux-s390x)
    install_from_apt_llvm
    ;;
Darwin-arm64|Darwin-aarch64|Darwin-x86_64|Darwin-amd64)
    install_from_brew
    ;;
*)
    echo "unsupported host for prebuilt LLVM: $(uname -s)-$(uname -m)" >&2
    exit 1
    ;;
esac

installed_llvm_config_path="$(installed_llvm_config || true)"
installed_clang_path="$(installed_clang || true)"
if [[ -z "${installed_llvm_config_path}" ]] ||
    ! verify_existing_install "${installed_llvm_config_path}" "${installed_clang_path}"; then
    echo "installed LLVM at ${LLVM_INSTALL_DIR} is not runnable" >&2
    exit 1
fi

echo "installed LLVM ${LLVM_VERSION} into ${LLVM_INSTALL_DIR}"

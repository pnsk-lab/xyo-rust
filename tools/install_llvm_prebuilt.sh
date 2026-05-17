#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LLVM_VERSION="${LLVM_VERSION:-21.1.8}"
LLVM_INSTALL_DIR="${LLVM_INSTALL_DIR:-${ROOT_DIR}/.llvm/$(uname -s)-$(uname -m)}"

if [[ -x "${LLVM_INSTALL_DIR}/bin/llvm-config" || -x "${LLVM_INSTALL_DIR}/bin/llvm-config.exe" ]]; then
    echo "LLVM already installed at ${LLVM_INSTALL_DIR}"
    exit 0
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

install_s390x() {
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
    run_as_root env DEBIAN_FRONTEND=noninteractive apt-get install -q -y llvm-21 llvm-21-dev clang-21

    local source_root
    source_root="$(llvm-config-21 --prefix)"
    copy_tree "${source_root}" "${LLVM_INSTALL_DIR}"
}

case "$(uname -s)-$(uname -m)" in
Linux-x86_64|Linux-amd64)
    install_from_archive "LLVM-${LLVM_VERSION}-Linux-X64.tar.xz"
    ;;
Linux-aarch64|Linux-arm64)
    install_from_archive "LLVM-${LLVM_VERSION}-Linux-ARM64.tar.xz"
    ;;
Linux-armv7l|Linux-armv7*)
    install_from_archive "clang+llvm-${LLVM_VERSION}-armv7a-linux-gnueabihf.tar.gz"
    ;;
Linux-s390x)
    install_s390x
    ;;
Darwin-arm64|Darwin-aarch64|Darwin-x86_64|Darwin-amd64)
    install_from_brew
    ;;
*)
    echo "unsupported host for prebuilt LLVM: $(uname -s)-$(uname -m)" >&2
    exit 1
    ;;
esac

echo "installed LLVM ${LLVM_VERSION} into ${LLVM_INSTALL_DIR}"

param(
    [string]$InstallDir,
    [string]$TargetArch
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$rootDir = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$version = if (-not [string]::IsNullOrWhiteSpace($env:LLVM_VERSION)) { $env:LLVM_VERSION } else { '21.1.8' }

function Get-RequestedArch {
    param(
        [string]$ExplicitArch
    )

    $candidates = @(
        $ExplicitArch,
        $env:LLVM_TARGET_ARCH,
        $env:CARGO_BUILD_TARGET,
        $env:RUNNER_ARCH,
        $env:PROCESSOR_ARCHITECTURE
    )

    foreach ($candidate in $candidates) {
        if ([string]::IsNullOrWhiteSpace($candidate)) {
            continue
        }

        $normalized = $candidate.ToLowerInvariant()
        if ($normalized -match '^(aarch64|arm64)(-|$)' -or $normalized -eq 'arm64') {
            return 'ARM64'
        }

        if ($normalized -match '^(x86_64|amd64|x64)(-|$)' -or $normalized -in @('amd64', 'x64')) {
            return 'AMD64'
        }
    }

    throw 'unable to determine Windows LLVM target architecture'
}

function Get-ExpectedHostTarget {
    param(
        [string]$Arch
    )

    switch ($Arch) {
        'AMD64' { return 'x86_64-pc-windows-msvc' }
        'ARM64' { return 'aarch64-pc-windows-msvc' }
        default { throw "unsupported Windows architecture for LLVM source build: $Arch" }
    }
}

function Get-CmakeArch {
    param(
        [string]$Arch
    )

    switch ($Arch) {
        'AMD64' { return 'x64' }
        'ARM64' { return 'ARM64' }
        default { throw "unsupported Windows architecture for LLVM source build: $Arch" }
    }
}

function Get-LlvmTargets {
    param(
        [string]$Arch
    )

    switch ($Arch) {
        'AMD64' { return 'X86;WebAssembly' }
        'ARM64' { return 'AArch64;WebAssembly' }
        default { throw "unsupported Windows architecture for LLVM source build: $Arch" }
    }
}

function Invoke-Checked {
    param(
        [string]$Command,
        [string[]]$Arguments
    )

    & $Command @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Command failed with exit code $LASTEXITCODE"
    }
}

function Test-LlvmInstallArch {
    param(
        [string]$LlvmConfigPath,
        [string]$Arch
    )

    $expectedHostTarget = Get-ExpectedHostTarget -Arch $Arch
    try {
        $actualHostTarget = (& $LlvmConfigPath --host-target).Trim()
    }
    catch {
        return $false
    }

    if ($actualHostTarget -ne $expectedHostTarget) {
        Write-Host "LLVM host target mismatch: expected $expectedHostTarget, found $actualHostTarget"
        return $false
    }

    return $true
}

$arch = Get-RequestedArch -ExplicitArch $TargetArch
$cmakeArch = Get-CmakeArch -Arch $arch
$llvmTargets = Get-LlvmTargets -Arch $arch
Write-Host "using LLVM_TARGET_ARCH=$arch"

if ([string]::IsNullOrWhiteSpace($InstallDir)) {
    $InstallDir = $env:LLVM_INSTALL_DIR
}

if ([string]::IsNullOrWhiteSpace($InstallDir)) {
    $InstallDir = Join-Path (Join-Path $rootDir '.llvm') $arch
}

$llvmConfigExe = Join-Path (Join-Path $InstallDir 'bin') 'llvm-config.exe'
if (Test-Path $llvmConfigExe) {
    if (Test-LlvmInstallArch -LlvmConfigPath $llvmConfigExe -Arch $arch) {
        Write-Host "LLVM already installed at $InstallDir"
        exit 0
    }

    Write-Host "removing LLVM install with the wrong host target: $InstallDir"
    Remove-Item $InstallDir -Recurse -Force
}

$tmpRoot = if (-not [string]::IsNullOrWhiteSpace($env:RUNNER_TEMP)) { $env:RUNNER_TEMP } else { [System.IO.Path]::GetTempPath() }
$llvmSourceDir = Join-Path $tmpRoot "llvm-project-$arch"
$llvmBuildDir = Join-Path $tmpRoot "llvm-build-$arch"

foreach ($path in @($InstallDir, $llvmSourceDir, $llvmBuildDir)) {
    if (Test-Path $path) {
        Remove-Item $path -Recurse -Force
    }
}

Invoke-Checked -Command 'git' -Arguments @(
    'clone',
    '--depth', '1',
    '--branch', "llvmorg-$version",
    'https://github.com/llvm/llvm-project.git',
    $llvmSourceDir
)

$cmakeArgs = @(
    '-S', (Join-Path $llvmSourceDir 'llvm'),
    '-B', $llvmBuildDir,
    '-G', 'Visual Studio 17 2022',
    '-A', $cmakeArch,
    '-DCMAKE_BUILD_TYPE=Release',
    "-DCMAKE_INSTALL_PREFIX=$InstallDir",
    "-DLLVM_TARGETS_TO_BUILD=$llvmTargets",
    '-DLLVM_INCLUDE_TESTS=OFF',
    '-DLLVM_INCLUDE_BENCHMARKS=OFF',
    '-DLLVM_INCLUDE_EXAMPLES=OFF',
    '-DLLVM_BUILD_LLVM_DYLIB=OFF',
    '-DBUILD_SHARED_LIBS=OFF',
    '-DLLVM_ENABLE_FFI=OFF',
    '-DLLVM_ENABLE_LIBXML2=OFF',
    '-DLLVM_ENABLE_RPMALLOC=OFF',
    '-DLLVM_ENABLE_TERMINFO=OFF',
    '-DLLVM_ENABLE_ZLIB=OFF',
    '-DLLVM_ENABLE_ZSTD=OFF'
)
Invoke-Checked -Command 'cmake' -Arguments $cmakeArgs

Invoke-Checked -Command 'cmake' -Arguments @(
    '--build', $llvmBuildDir,
    '--config', 'Release',
    '--target', 'INSTALL',
    '--', '/m'
)

if (-not (Test-Path $llvmConfigExe)) {
    throw "failed to install LLVM into $InstallDir"
}

if (-not (Test-LlvmInstallArch -LlvmConfigPath $llvmConfigExe -Arch $arch)) {
    throw "LLVM source build architecture does not match requested $arch"
}

Write-Host "installed LLVM $version into $InstallDir"

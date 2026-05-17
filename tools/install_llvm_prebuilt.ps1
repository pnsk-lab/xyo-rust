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

$arch = Get-RequestedArch -ExplicitArch $TargetArch
Write-Host "using LLVM_TARGET_ARCH=$arch"

function Get-ExpectedHostTarget {
    param(
        [string]$Arch
    )

    switch ($Arch) {
        'AMD64' { return 'x86_64-pc-windows-msvc' }
        'ARM64' { return 'aarch64-pc-windows-msvc' }
        default { throw "unsupported Windows architecture for prebuilt LLVM: $Arch" }
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

switch ($arch) {
    'AMD64' {
        $assetName = "clang+llvm-$version-x86_64-pc-windows-msvc.tar.xz"
        break
    }
    'ARM64' {
        $assetName = "clang+llvm-$version-aarch64-pc-windows-msvc.tar.xz"
        break
    }
    default {
        throw "unsupported Windows architecture for prebuilt LLVM: $arch"
    }
}

$tmpRoot = if (-not [string]::IsNullOrWhiteSpace($env:RUNNER_TEMP)) { $env:RUNNER_TEMP } else { [System.IO.Path]::GetTempPath() }
$tmpDir = Join-Path $tmpRoot "llvm-prebuilt-$arch"
if (Test-Path $tmpDir) {
    Remove-Item $tmpDir -Recurse -Force
}
New-Item -ItemType Directory -Path $tmpDir | Out-Null

$archiveUrl = "https://github.com/llvm/llvm-project/releases/download/llvmorg-$version/$assetName"
$archivePath = Join-Path $tmpDir $assetName
$extractDir = Join-Path $tmpDir 'extract'
New-Item -ItemType Directory -Path $extractDir | Out-Null

Write-Host "downloading $assetName"
Invoke-WebRequest -Uri $archiveUrl -OutFile $archivePath

& tar -xf $archivePath -C $extractDir
if ($LASTEXITCODE -ne 0) {
    throw "tar failed while extracting $assetName"
}

$llvmConfig = Get-ChildItem -Path $extractDir -Recurse -Filter llvm-config.exe | Select-Object -First 1
if (-not $llvmConfig) {
    throw "failed to locate llvm-config.exe in $assetName"
}

$sourceRoot = Split-Path -Parent (Split-Path -Parent $llvmConfig.FullName)
if (Test-Path $InstallDir) {
    Remove-Item $InstallDir -Recurse -Force
}
New-Item -ItemType Directory -Path $InstallDir | Out-Null

Get-ChildItem -Force $sourceRoot | ForEach-Object {
    Copy-Item -LiteralPath $_.FullName -Destination $InstallDir -Recurse -Force
}

if (-not (Test-Path $llvmConfigExe)) {
    throw "failed to install LLVM into $InstallDir"
}

if (-not (Test-LlvmInstallArch -LlvmConfigPath $llvmConfigExe -Arch $arch)) {
    throw "LLVM archive architecture does not match requested $arch"
}

Write-Host "installed LLVM $version into $InstallDir"

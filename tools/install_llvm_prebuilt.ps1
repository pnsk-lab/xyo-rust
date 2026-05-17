param(
    [string]$InstallDir
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$rootDir = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$version = if (-not [string]::IsNullOrWhiteSpace($env:LLVM_VERSION)) { $env:LLVM_VERSION } else { '21.1.8' }

if ([string]::IsNullOrWhiteSpace($InstallDir)) {
    $InstallDir = $env:LLVM_INSTALL_DIR
}

if ([string]::IsNullOrWhiteSpace($InstallDir)) {
    $InstallDir = Join-Path (Join-Path $rootDir '.llvm') $env:PROCESSOR_ARCHITECTURE
}

$llvmConfigExe = Join-Path (Join-Path $InstallDir 'bin') 'llvm-config.exe'
if (Test-Path $llvmConfigExe) {
    Write-Host "LLVM already installed at $InstallDir"
    exit 0
}

$arch = ($env:PROCESSOR_ARCHITECTURE ?? '').ToUpperInvariant()
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

$tmpDir = Join-Path $env:RUNNER_TEMP "llvm-prebuilt-$arch"
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

Write-Host "installed LLVM $version into $InstallDir"

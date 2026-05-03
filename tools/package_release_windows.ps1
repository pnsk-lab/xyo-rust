param(
    [string]$ArchivePath,
    [string]$StageDir
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$rootDir = Split-Path -Parent $PSScriptRoot
$releaseTarget = $env:XYO_RELEASE_TARGET
if ([string]::IsNullOrWhiteSpace($releaseTarget)) {
    $releaseTarget = 'windows'
}

if ([string]::IsNullOrWhiteSpace($ArchivePath)) {
    $ArchivePath = Join-Path $rootDir "target/release/xyo-$releaseTarget.zip"
}

if ([string]::IsNullOrWhiteSpace($StageDir)) {
    $StageDir = Join-Path $rootDir "target/release/xyo-$releaseTarget"
}

$binPath = $env:XYO_RELEASE_BIN
if ([string]::IsNullOrWhiteSpace($binPath)) {
    $binPath = Join-Path $rootDir 'target/release/xyo.exe'
}

$icuRuntimeDir = $env:XYO_ICU_RUNTIME_DIR
if ([string]::IsNullOrWhiteSpace($icuRuntimeDir)) {
    $icuRuntimeDir = Join-Path $rootDir 'target/vcpkg/installed/x64-windows'
}

$icuBinDir = Join-Path $icuRuntimeDir 'bin'

if (-not (Test-Path $binPath)) {
    throw "release binary not found: $binPath"
}

if (-not (Test-Path $icuBinDir)) {
    throw "ICU runtime bin directory not found: $icuBinDir"
}

$dlls = Get-ChildItem -Path $icuBinDir -Filter 'icu*.dll' | Sort-Object Name
if ($dlls.Count -eq 0) {
    throw "no ICU runtime DLLs found in $icuBinDir"
}

if (Test-Path $StageDir) {
    Remove-Item $StageDir -Recurse -Force
}
if (Test-Path $ArchivePath) {
    Remove-Item $ArchivePath -Force
}

$archiveParent = Split-Path -Parent $ArchivePath
if (-not [string]::IsNullOrWhiteSpace($archiveParent)) {
    New-Item -ItemType Directory -Path $archiveParent -Force | Out-Null
}
New-Item -ItemType Directory -Path $StageDir -Force | Out-Null

Copy-Item $binPath (Join-Path $StageDir 'xyo.exe')
foreach ($dll in $dlls) {
    Copy-Item $dll.FullName (Join-Path $StageDir $dll.Name)
}

Compress-Archive -Path (Join-Path $StageDir '*') -DestinationPath $ArchivePath

Write-Host "packaged Windows release artifact at $ArchivePath"

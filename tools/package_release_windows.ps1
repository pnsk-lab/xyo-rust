param(
    [string]$ArchivePath = (Join-Path (Join-Path $PSScriptRoot '..') 'target/release/xyo-windows.zip'),
    [string]$StageDir = (Join-Path (Join-Path $PSScriptRoot '..') 'target/release/xyo-windows')
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$rootDir = Split-Path -Parent $PSScriptRoot
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

New-Item -ItemType Directory -Path $StageDir | Out-Null

Copy-Item $binPath (Join-Path $StageDir 'xyo.exe')
foreach ($dll in $dlls) {
    Copy-Item $dll.FullName (Join-Path $StageDir $dll.Name)
}

Compress-Archive -Path (Join-Path $StageDir '*') -DestinationPath $ArchivePath

Write-Host "packaged Windows release artifact at $ArchivePath"

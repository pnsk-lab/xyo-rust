param(
    [switch]$CleanIcu,
    [switch]$SkipIcu,
    [switch]$SkipCheck,
    [switch]$ForceFetchIcu,
    [string]$VcpkgTriplet
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$rootDir = Split-Path -Parent $MyInvocation.MyCommand.Path

function Get-DefaultTriplet {
    param(
        [string]$RunnerArch,
        [string]$ExplicitTriplet
    )

    if (-not [string]::IsNullOrWhiteSpace($ExplicitTriplet)) {
        return $ExplicitTriplet
    }

    if (-not [string]::IsNullOrWhiteSpace($env:XYO_VCPKG_TRIPLET)) {
        return $env:XYO_VCPKG_TRIPLET
    }

    if (-not [string]::IsNullOrWhiteSpace($env:VCPKG_DEFAULT_TRIPLET)) {
        return $env:VCPKG_DEFAULT_TRIPLET
    }

    switch (($RunnerArch ?? '').ToUpperInvariant()) {
        'ARM64' { return 'arm64-windows' }
        default { return 'x64-windows' }
    }
}

function Get-VcpkgRoot {
    if (-not [string]::IsNullOrWhiteSpace($env:VCPKG_ROOT)) {
        return $env:VCPKG_ROOT
    }

    if ($env:GITHUB_ACTIONS -eq 'true' -and -not [string]::IsNullOrWhiteSpace($env:RUNNER_TEMP)) {
        return (Join-Path $env:RUNNER_TEMP 'vcpkg')
    }

    return (Join-Path $rootDir 'target/vcpkg')
}

function Ensure-Vcpkg {
    param(
        [string]$Root
    )

    $vcpkgExe = Join-Path $Root 'vcpkg.exe'
    if (Test-Path $vcpkgExe) {
        return $vcpkgExe
    }

    if (-not (Test-Path $Root)) {
        git clone --depth 1 https://github.com/microsoft/vcpkg $Root | Out-Host
    }

    $bootstrap = Join-Path $Root 'bootstrap-vcpkg.bat'
    if (-not (Test-Path $bootstrap)) {
        throw "vcpkg bootstrap script not found: $bootstrap"
    }

    & $bootstrap -disableMetrics | Out-Host

    if (-not (Test-Path $vcpkgExe)) {
        throw "vcpkg executable not found after bootstrap: $vcpkgExe"
    }

    return [string]$vcpkgExe
}

$triplet = Get-DefaultTriplet -RunnerArch $env:RUNNER_ARCH -ExplicitTriplet $VcpkgTriplet
$vcpkgRoot = Get-VcpkgRoot
$vcpkgExe = Join-Path $vcpkgRoot 'vcpkg.exe'
$icuInstallRoot = Join-Path $vcpkgRoot "installed/$triplet"

if ($ForceFetchIcu) {
    Write-Host 'ForceFetchIcu is not used on Windows; proceeding with vcpkg install.'
}

if (-not $SkipIcu) {
    $vcpkgExe = Ensure-Vcpkg -Root $vcpkgRoot

    if ($CleanIcu -and (Test-Path $icuInstallRoot)) {
        Remove-Item $icuInstallRoot -Recurse -Force
    }

    & $vcpkgExe install "icu:$triplet"
}

if (-not (Test-Path $icuInstallRoot)) {
    throw "ICU prebuilt directory not found: $icuInstallRoot"
}

$env:XYO_ICU_PREBUILT_DIR = $icuInstallRoot
$env:XYO_ICU_NATIVE_LIB_DIR = Join-Path $icuInstallRoot 'lib'

Write-Host "using VCPKG_ROOT=$vcpkgRoot"
Write-Host "using VCPKG_TRIPLET=$triplet"
Write-Host "using XYO_ICU_PREBUILT_DIR=$env:XYO_ICU_PREBUILT_DIR"
Write-Host "using XYO_ICU_NATIVE_LIB_DIR=$env:XYO_ICU_NATIVE_LIB_DIR"

Push-Location $rootDir
try {
    cargo build --release
}
finally {
    Pop-Location
}

Write-Host 'setup completed'

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

function Export-GitHubEnv {
    param(
        [string]$Name,
        [string]$Value
    )

    if ([string]::IsNullOrWhiteSpace($env:GITHUB_ENV)) {
        return
    }

    "$Name=$Value" | Out-File -FilePath $env:GITHUB_ENV -Encoding utf8 -Append
}

$triplet = Get-DefaultTriplet -RunnerArch $env:RUNNER_ARCH -ExplicitTriplet $VcpkgTriplet
if ($triplet.EndsWith('-static')) {
    $staticTriplet = $triplet
} else {
    $staticTriplet = "$triplet-static"
}
$vcpkgRoot = Get-VcpkgRoot
$vcpkgExe = Join-Path $vcpkgRoot 'vcpkg.exe'
$icuInstallRoot = Join-Path $vcpkgRoot "installed/$triplet"
# LLVM's Windows release binaries still reference libxml2 at link time, so we
# keep a static copy available alongside the ICU packages.
$libxml2InstallRoot = Join-Path $vcpkgRoot "installed/$staticTriplet"

function Ensure-LibXml2LibraryAliases {
    param(
        [string]$LibDir
    )

    if (-not (Test-Path $LibDir)) {
        return
    }

    $libXml2s = Join-Path $LibDir 'libxml2s.lib'
    $xml2s = Join-Path $LibDir 'xml2s.lib'

    if ((Test-Path $libXml2s) -and (-not (Test-Path $xml2s))) {
        Copy-Item -LiteralPath $libXml2s -Destination $xml2s -Force
        return
    }

    if ((Test-Path $xml2s) -and (-not (Test-Path $libXml2s))) {
        Copy-Item -LiteralPath $xml2s -Destination $libXml2s -Force
    }
}

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

if (-not (Test-Path $libxml2InstallRoot)) {
    $vcpkgExe = Ensure-Vcpkg -Root $vcpkgRoot
    & $vcpkgExe install "libxml2:$staticTriplet"
}

if (-not (Test-Path $icuInstallRoot)) {
    throw "ICU prebuilt directory not found: $icuInstallRoot"
}

Ensure-LibXml2LibraryAliases -LibDir (Join-Path $libxml2InstallRoot 'lib')

$libxml2LibDir = Join-Path $libxml2InstallRoot 'lib'
$env:XYO_ICU_PREBUILT_DIR = $icuInstallRoot
$env:XYO_ICU_NATIVE_LIB_DIR = Join-Path $icuInstallRoot 'lib'
$env:XYO_ICU_RUNTIME_DIR = $icuInstallRoot
if ([string]::IsNullOrWhiteSpace($env:LIB)) {
    $env:LIB = $libxml2LibDir
} else {
    $env:LIB = "$libxml2LibDir;$env:LIB"
}

Export-GitHubEnv -Name 'XYO_ICU_PREBUILT_DIR' -Value $env:XYO_ICU_PREBUILT_DIR
Export-GitHubEnv -Name 'XYO_ICU_NATIVE_LIB_DIR' -Value $env:XYO_ICU_NATIVE_LIB_DIR
Export-GitHubEnv -Name 'XYO_ICU_RUNTIME_DIR' -Value $env:XYO_ICU_RUNTIME_DIR

Write-Host "using VCPKG_ROOT=$vcpkgRoot"
Write-Host "using VCPKG_TRIPLET=$triplet"
Write-Host "using XYO_ICU_PREBUILT_DIR=$env:XYO_ICU_PREBUILT_DIR"
Write-Host "using XYO_ICU_NATIVE_LIB_DIR=$env:XYO_ICU_NATIVE_LIB_DIR"
Write-Host "using XYO_ICU_RUNTIME_DIR=$env:XYO_ICU_RUNTIME_DIR"

Push-Location $rootDir
try {
    cargo build --release
}
finally {
    Pop-Location
}

Write-Host 'setup completed'

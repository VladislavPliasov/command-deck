# Command Deck - P0-02 Windows environment check.
#
# READ-ONLY: detects required development tools and libraries on Windows 11.
# It installs nothing, never requires administrator privileges, and never
# modifies system or shell configuration.

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$script:Fail = 0

function Write-Pass { param([string]$Message) Write-Host "[PASS] $Message" }
function Write-Fail { param([string]$Message) Write-Host "[FAIL] $Message"; $script:Fail = 1 }
function Write-Note { param([string]$Message) Write-Host "       $Message" }

function Test-IsAdministrator {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

if (Test-IsAdministrator) {
    Write-Fail "Running elevated (administrator) is not allowed. Run as a normal user."
    exit 1
}

Write-Host "== Command Deck environment check (read-only, Windows) =="

Write-Host "-- Required commands --"
foreach ($cmd in @('rustc', 'cargo', 'node', 'npm')) {
    $resolved = Get-Command $cmd -ErrorAction SilentlyContinue
    if ($resolved) {
        Write-Pass "$cmd ($($resolved.Source))"
    } else {
        Write-Fail "$cmd not found in PATH"
    }
}

Write-Host "-- Versions --"
if (Get-Command rustc -ErrorAction SilentlyContinue) {
    Write-Note "rustc: $(rustc --version 2>&1)"
    Write-Note "cargo: $(cargo --version 2>&1)"
    $hostTriple = ((rustc -vV 2>&1) | Select-String '^host:') -replace '^host:\s*', ''
    Write-Note "host: $hostTriple"
    if ($hostTriple -like '*pc-windows-msvc*') {
        Write-Pass "Rust host target is MSVC ($hostTriple)"
    } else {
        Write-Fail "Rust host target is not MSVC ($hostTriple); the MSVC toolchain is required"
    }
}
if (Get-Command node -ErrorAction SilentlyContinue) {
    Write-Note "node: $(node -v 2>&1)"
    Write-Note "npm: $(npm -v 2>&1)"
}

Write-Host "-- Microsoft C++ toolchain (MSVC) --"
$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
if (Test-Path $vswhere) {
    $vsPath = & $vswhere -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath | Select-Object -First 1
    if ($vsPath) {
        Write-Pass "MSVC toolset (VC.Tools.x86.x64) at $vsPath"
    } else {
        Write-Fail "MSVC C++ toolset not found (vswhere reports no VC.Tools.x86.x64 component)"
    }
} else {
    Write-Fail "vswhere.exe not found (Visual Studio / Build Tools installer missing?)"
}

Write-Host "-- Windows SDK --"
$sdkInclude = 'C:\Program Files (x86)\Windows Kits\10\Include'
$sdkVersions = Get-ChildItem -Path $sdkInclude -Directory -ErrorAction SilentlyContinue |
    Where-Object { $_.Name -like '10.*' } | Select-Object -ExpandProperty Name
if ($sdkVersions) {
    Write-Pass "Windows SDK: $($sdkVersions -join ', ')"
} else {
    Write-Fail "Windows SDK not found under $sdkInclude"
}

Write-Host "-- WebView2 Runtime --"
$wv2Keys = @(
    'HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}',
    'HKLM:\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}'
)
$wv2Found = $false
foreach ($key in $wv2Keys) {
    if (Test-Path $key) {
        $pv = (Get-ItemProperty -Path $key -Name pv -ErrorAction SilentlyContinue).pv
        if ($pv) {
            Write-Pass "WebView2 Runtime pv=$pv"
            $wv2Found = $true
            break
        }
    }
}
if (-not $wv2Found) {
    Write-Fail "WebView2 Runtime not detected (EdgeUpdate client registry value missing)"
}

Write-Host "-- Project-local Tauri CLI --"
$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$cliPkg = Join-Path $repoRoot 'node_modules\@tauri-apps\cli\package.json'
if (Test-Path $cliPkg) {
    $cliVersion = (Get-Content $cliPkg -Raw | ConvertFrom-Json).version
    Write-Pass "Tauri CLI $cliVersion (project-local)"
} elseif (-not (Test-Path (Join-Path $repoRoot 'node_modules'))) {
    Write-Note "node_modules not present (fresh clone) - run: npm ci"
} else {
    Write-Fail "@tauri-apps/cli missing from node_modules (corrupt install? run: npm ci)"
}

Write-Host ""
if ($script:Fail -ne 0) {
    Write-Host "RESULT: FAIL - missing requirements are listed above."
    Write-Host "This script never installs anything. Install missing components"
    Write-Host "manually after reviewing them (see the P0-02 plan, section 3)."
    exit 1
}

Write-Host "RESULT: PASS - all requirements detected."
exit 0

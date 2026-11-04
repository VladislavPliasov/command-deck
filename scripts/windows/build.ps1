# Command Deck - production build (release executable + optional installers) on Windows.
#
# Runs the read-only environment check, verifies project dependencies are
# installed, then runs `npm run tauri build`. It never installs packages,
# never requires administrator privileges, and never modifies system
# configuration.

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'

function Test-IsAdministrator {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

if (Test-IsAdministrator) {
    Write-Error "Run as a normal user, not elevated (administrator)."
    exit 1
}

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Set-Location $repoRoot

powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'check-env.ps1')
if ($LASTEXITCODE -ne 0) {
    Write-Error "environment check failed (see output above)."
    exit 1
}

if (-not (Test-Path (Join-Path $repoRoot 'node_modules'))) {
    Write-Error "node_modules is missing. Run: npm ci"
    exit 1
}

& npm run tauri build
$buildExit = $LASTEXITCODE

Write-Host ""
Write-Host "Expected artifacts:"
Write-Host "  executable: src-tauri\target\release\command-deck.exe"
Write-Host "  installers: src-tauri\target\release\bundle\nsis\ and bundle\msi\ (optional)"
exit $buildExit

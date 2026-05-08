# Build the launcher for Windows (x86_64).
#
# Usage:
#   pwsh scripts/build-windows.ps1
#   powershell -ExecutionPolicy Bypass -File scripts/build-windows.ps1
#
# Output:
#   src-tauri\target\release\rathole-gui-launcher.exe        (raw binary)
#   src-tauri\target\release\bundle\msi\*.msi
#   src-tauri\target\release\bundle\nsis\*-setup.exe

[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir   = Resolve-Path (Join-Path $ScriptDir '..')
Set-Location $RootDir

function Write-Step($msg) { Write-Host "`n==> $msg" -ForegroundColor Blue }
function Write-Ok($msg)   { Write-Host "[ok] $msg" -ForegroundColor Green }
function Write-Warn($msg) { Write-Host "[!]  $msg" -ForegroundColor Yellow }
function Write-Err($msg)  { Write-Host "[x]  $msg" -ForegroundColor Red }

Write-Step "Checking prerequisites"

if (-not $IsWindows -and ($PSVersionTable.PSEdition -ne 'Desktop')) {
    Write-Err "This script must be run on Windows."
    exit 1
}

function Test-Cmd($name) {
    return [bool] (Get-Command $name -ErrorAction SilentlyContinue)
}

if (-not (Test-Cmd 'node')) {
    Write-Err "Node.js is not installed. Install Node 18+ from https://nodejs.org"
    exit 1
}
Write-Ok "Node.js $(node --version)"

if (-not (Test-Cmd 'cargo')) {
    Write-Err "Rust toolchain not found. Install it from https://rustup.rs"
    exit 1
}
Write-Ok "Rust $((rustc --version) -split ' ' | Select-Object -Index 1)"

# Visual Studio C++ Build Tools are required for the MSVC linker.
$LinkExe = Get-Command 'link.exe' -ErrorAction SilentlyContinue
$ClExe   = Get-Command 'cl.exe' -ErrorAction SilentlyContinue
if (-not $LinkExe -or -not $ClExe) {
    Write-Warn "MSVC build tools were not found on PATH."
    Write-Warn "If the build fails with a linker error, install 'Desktop development with C++' from"
    Write-Warn "the Visual Studio Build Tools installer: https://visualstudio.microsoft.com/downloads/"
    Write-Warn "and run this script from a 'Developer PowerShell for VS' window."
} else {
    Write-Ok "MSVC build tools"
}

# WebView2 runtime is preinstalled on Windows 10 21H2+ and Windows 11. Warn otherwise.
$WebView2Reg = Get-ItemProperty -Path 'HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}' -ErrorAction SilentlyContinue
if (-not $WebView2Reg) {
    Write-Warn "Microsoft Edge WebView2 runtime not detected."
    Write-Warn "End users will need it installed: https://developer.microsoft.com/microsoft-edge/webview2/"
} else {
    Write-Ok "WebView2 runtime"
}

Write-Step "Installing npm dependencies"
if (Test-Path 'package-lock.json') {
    npm ci
} else {
    npm install
}
Write-Ok "Dependencies installed"

Write-Step "Building Tauri bundle"
npx tauri build

Write-Step "Done"

$BundleBase = 'src-tauri\target\release\bundle'
$RawBin     = 'src-tauri\target\release\rathole-gui-launcher.exe'

Write-Host ""
Write-Host "Artifacts:"
if (Test-Path $RawBin) {
    Write-Host "  $RawBin"
}
if (Test-Path $BundleBase) {
    Get-ChildItem -Path $BundleBase -Recurse -Include '*.msi', '*-setup.exe' |
        ForEach-Object { Write-Host "  $($_.FullName)" }
}
Write-Host ""
Write-Host "Drop rathole.exe next to rathole-gui-launcher.exe before launching."

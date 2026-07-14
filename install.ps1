# voidfetch Windows Installer
$ErrorActionPreference = "Stop"

$InstallDir = Join-Path $env:LOCALAPPDATA "voidfetch"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "[*] Checking for Rust..." -ForegroundColor Cyan
try {
    $rustVersion = rustc --version 2>&1
    Write-Host "[+] Found $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "[-] Rust is not installed." -ForegroundColor Red
    Write-Host "    Install from: https://rustup.rs" -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host "[*] Building voidfetch..." -ForegroundColor Cyan
Push-Location $ScriptDir
cargo build --release 2>&1
Pop-Location

$Bin = Join-Path $ScriptDir "target\release\voidfetch.exe"
if (!(Test-Path $Bin)) {
    Write-Host "[-] Build failed" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host "[*] Installing to $InstallDir ..." -ForegroundColor Cyan
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

Copy-Item $Bin -Destination $InstallDir -Force

$logosDir = Join-Path $InstallDir "logos"
if (!(Test-Path $logosDir)) {
    New-Item -ItemType Directory -Path $logosDir -Force | Out-Null
}
Copy-Item (Join-Path $ScriptDir "logos\*") -Destination $logosDir -Force

$examplesDir = Join-Path $InstallDir "examples"
if (!(Test-Path $examplesDir)) {
    New-Item -ItemType Directory -Path $examplesDir -Force | Out-Null
}
Copy-Item (Join-Path $ScriptDir "examples\*") -Destination $examplesDir -Force

Write-Host "[*] Adding to PATH..." -ForegroundColor Cyan
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
    Write-Host "[+] Added to PATH" -ForegroundColor Green
    Write-Host "    Restart your terminal for PATH changes." -ForegroundColor Yellow
} else {
    Write-Host "[+] Already in PATH" -ForegroundColor Green
}

$configDir = Join-Path $env:USERPROFILE ".config\voidfetch"
if (!(Test-Path $configDir)) {
    New-Item -ItemType Directory -Path $configDir -Force | Out-Null
}
$configFile = Join-Path $configDir "config.css"
if (!(Test-Path $configFile)) {
    & (Join-Path $InstallDir "voidfetch.exe") --dump-config | Out-File -FilePath $configFile -Encoding utf8
    Write-Host "[+] Created config at $configFile" -ForegroundColor Green
}

Write-Host ""
Write-Host "[+] Installed successfully!" -ForegroundColor Green
Write-Host "    Run 'voidfetch' from a new terminal." -ForegroundColor Cyan
Read-Host "Press Enter to exit"

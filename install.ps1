# voidfetch Windows Installer

$ErrorActionPreference = "Stop"

$InstallDir = Join-Path $env:LOCALAPPDATA "voidfetch"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host "[*] Checking for Python..." -ForegroundColor Cyan
try {
    $pythonVersion = python --version 2>&1
    Write-Host "[+] Found $pythonVersion" -ForegroundColor Green
} catch {
    Write-Host "[-] Python is not installed or not in PATH." -ForegroundColor Red
    Write-Host "    Download Python from: https://www.python.org/downloads/" -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}

Write-Host "[*] Creating install directory at $InstallDir ..." -ForegroundColor Cyan
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

Write-Host "[*] Copying files..." -ForegroundColor Cyan
Copy-Item (Join-Path $ScriptDir "voidfetch.py") -Destination $InstallDir -Force
Copy-Item (Join-Path $ScriptDir "voidfetch.cmd") -Destination $InstallDir -Force

$logosDir = Join-Path $InstallDir "logos"
if (!(Test-Path $logosDir)) {
    New-Item -ItemType Directory -Path $logosDir -Force | Out-Null
}
Copy-Item (Join-Path $ScriptDir "logos\*") -Destination $logosDir -Force

Write-Host "[*] Adding to PATH..." -ForegroundColor Cyan
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
    Write-Host "[+] Added $InstallDir to PATH" -ForegroundColor Green
    Write-Host "    Restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
} else {
    Write-Host "[+] Already in PATH" -ForegroundColor Green
}

Write-Host ""
Write-Host "[+] Installed successfully!" -ForegroundColor Green
Write-Host "    Run 'voidfetch' from a new PowerShell or CMD window." -ForegroundColor Cyan
Read-Host "Press Enter to exit"

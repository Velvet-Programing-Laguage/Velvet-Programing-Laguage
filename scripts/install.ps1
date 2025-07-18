#Requires -RunAsAdministrator

Write-Host "Install Velvet? (y/n)" -ForegroundColor Cyan
$response = Read-Host
if ($response -notmatch "^[yY]$") {
    Write-Host "Aborted." -ForegroundColor Red
    exit 1
}

Write-Host "Single executable? (y/n)" -ForegroundColor Cyan
$single_file = Read-Host
$singleFile = $false
if ($single_file -match "^[yY]$") { $singleFile = $true }

# Check Chocolatey
if (-Not (Get-Command choco -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Chocolatey..." -ForegroundColor Yellow
    Set-ExecutionPolicy Bypass -Scope Process -Force
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
    iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
}

# Install dependencies
Write-Host "Installing dependencies..." -ForegroundColor Yellow
choco install rust python ruby elixir nodejs maven gradle git golang nuget -y
if ($LASTEXITCODE -ne 0) {
    Write-Host "Dependency install failed." -ForegroundColor Red
    exit 1
}

# Build Velvet
if (-Not (Test-Path "velvet")) {
    git clone https://github.com/xai/velvet.git
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Clone failed." -ForegroundColor Red
        exit 1
    }
}
Set-Location velvet
cargo build --release -q
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed." -ForegroundColor Red
    exit 1
}

# Install
$installPath = "C:\Program Files\Velvet"
if ($singleFile) {
    Copy-Item -Path "target\release\velvet.exe" -Destination "C:\Windows\vel.exe" -Force
    Copy-Item -Path "scripts\lib_manager.py" -Destination "C:\ProgramData\Velvet\lib_manager.py" -Force
    Set-ItemProperty -Path "C:\Windows\vel.exe" -Name Attributes -Value ([System.IO.FileAttributes]::Normal)
    Set-ItemProperty -Path "C:\ProgramData\Velvet\lib_manager.py" -Name Attributes -Value ([System.IO.FileAttributes]::Hidden)
} else {
    New-Item -Path $installPath -ItemType Directory -Force
    Copy-Item -Path . -Destination $installPath -Recurse -Force
    $env:Path += ";$installPath\target\release"
    [Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::Machine)
}

$libPath = "$installPath\lib\.velvet_library"
New-Item -Path $libPath -ItemType Directory -Force
icacls $libPath /inheritance:r
icacls $libPath /grant:r "Administrators:F"
icacls $libPath /grant:r "SYSTEM:F"

Write-Host "Velvet installed. Run 'vel help'." -ForegroundColor Green

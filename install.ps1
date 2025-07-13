# install.ps1
# PowerShell script to install Velvet programming language and its dependencies on Windows

# Exit on error
$ErrorActionPreference = "Stop"

# Function to check for Chocolatey and install if missing
function Install-Chocolatey {
    if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
        Write-Host "Chocolatey not found. Installing Chocolatey..."
        Set-ExecutionPolicy Bypass -Scope Process -Force
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
        Write-Host "Chocolatey installed successfully."
    } else {
        Write-Host "Chocolatey is already installed."
    }
}

# Function to detect the OS
function Detect-OS {
    $global:OS = $env:OS
    if ($OS -like "*Windows*") {
        Write-Host "Detected OS: Windows"
    } else {
        Write-Host "Unsupported OS: $OS"
        exit 1
    }
}

# Function to install system dependencies
function Install-SystemDeps {
    Write-Host "Installing system dependencies via Chocolatey..."
    choco install -y curl git python ruby rust go nodejs elixir openjdk11
    # Crystal (not available in Chocolatey, manual installation)
    Write-Host "Installing Crystal..."
    $crystalInstaller = "$env:TEMP\crystal-installer.exe"
    Invoke-WebRequest -Uri "https://github.com/crystal-lang/crystal/releases/download/1.10.1/crystal-1.10.1-1-windows-x86_64.msi" -OutFile $crystalInstaller
    Start-Process -FilePath "msiexec.exe" -ArgumentList "/i $crystalInstaller /quiet" -Wait
    Remove-Item $crystalInstaller
    # Kotlin
    choco install -y kotlin
}

# Function to set permissions (not typically needed on Windows, but ensure script is executable)
function Set-Permissions {
    Write-Host "Ensuring script has appropriate permissions..."
    # PowerShell scripts don't require chmod, but we ensure the script can run
    Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned -Force
}

# Function to clone Velvet repository
function Clone-VelvetRepo {
    Write-Host "Cloning Velvet repository..."
    $velvetDir = "Velvet-Programing-Language"
    if (-not (Test-Path $velvetDir)) {
        git clone https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Language.git
        Set-Location $velvetDir
    } else {
        Set-Location $velvetDir
        git pull origin main
    }
}

# Function to compile and install Velvet
function Install-Velvet {
    # Compile Rust core
    Write-Host "Compiling Velvet core (Rust)..."
    cargo build --release
    Copy-Item -Path "target\release\velvet.exe" -Destination "$env:ProgramFiles\Velvet\velvet_core.exe" -Force
    # Add to PATH
    $env:Path += ";$env:ProgramFiles\Velvet"
    [Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::User)

    # Compile Go CLI
    Write-Host "Compiling Velvet CLI (Go)..."
    go build -o vel.exe src/cli/main.go
    Copy-Item -Path "vel.exe" -Destination "$env:ProgramFiles\Velvet\vel.exe" -Force

    # Install Python dependencies
    Write-Host "Installing Python dependencies..."
    python -m pip install --upgrade pip
    pip install -r requirements.txt

    # Create .velvet-library directory
    $libDir = "$env:ProgramFiles\Velvet\.velvet-library"
    New-Item -ItemType Directory -Path $libDir -Force
    Write-Host "Created $libDir"

    # Copy Python hooks and configuration
    $hooksDir = "$env:ProgramFiles\Velvet\hooks"
    New-Item -ItemType Directory -Path $hooksDir -Force
    Copy-Item -Path "src\hooks\*" -Destination $hooksDir -Recurse -Force
    Copy-Item -Path "src\config\vel.config" -Destination "$env:ProgramFiles\Velvet\" -Force
}

# Main execution
Write-Host "Starting Velvet installation..."

# Set permissions
Set-Permissions

# Detect OS
Detect-OS

# Install Chocolatey if not present
Install-Chocolatey

# Install system dependencies
Install-SystemDeps

# Clone Velvet repository
Clone-VelvetRepo

# Install Velvet
Install-Velvet

Write-Host "Velvet installed successfully!"
Write-Host "You can now use 'vel' command to manage Velvet projects."
Write-Host "Try 'vel --help' for available commands."

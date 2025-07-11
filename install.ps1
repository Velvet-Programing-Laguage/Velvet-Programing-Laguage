# install.ps1
# Script to install Velvet programming language and its dependencies on Windows

# Function to check if running as administrator
function Test-Admin {
    $currentUser = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    $currentUser.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# Elevate to administrator if not already
if (-not (Test-Admin)) {
    Write-Host "This script requires administrative privileges. Elevating..."
    Start-Process powershell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}

# Function to install dependencies
function Install-Dependencies {
    Write-Host "Installing dependencies..."

    # Install Chocolatey (package manager) if not present
    if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
        Set-ExecutionPolicy Bypass -Scope Process -Force
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
    }

    # Install required languages and tools
    choco install -y git python ruby rust go openjdk11 nodejs elixir kotlin
    # Crystal (not available via Chocolatey, install manually)
    Write-Host "Installing Crystal..."
    $crystalUrl = "https://github.com/crystal-lang/crystal/releases/download/1.13.1/crystal-1.13.1-1-windows-x86_64-msvc.zip"
    $crystalZip = "$env:TEMP\crystal.zip"
    Invoke-WebRequest -Uri $crystalUrl -OutFile $crystalZip
    Expand-Archive -Path $crystalZip -DestinationPath "$env:ProgramFiles\Crystal"
    $env:Path += ";$env:ProgramFiles\Crystal\bin"
    [Environment]::SetEnvironmentVariable("Path", $env:Path, [System.EnvironmentVariableTarget]::Machine)
}

# Function to compile and install Velvet
function Install-Velvet {
    # Clone or assume the Velvet project is in the current directory
    if (-not (Test-Path "velvet")) {
        git clone https://github.com/andyh/velvet.git
        Set-Location velvet
    } else {
        Set-Location velvet
    }

    # Compile Rust core
    Write-Host "Compiling Velvet core (Rust)..."
    cargo build --release
    Copy-Item -Path "target\release\velvet.exe" -Destination "$env:ProgramFiles\Velvet\velvet_core.exe"

    # Compile Go CLI
    Write-Host "Compiling Velvet CLI (Go)..."
    go build -o vel.exe src/cli/main.go
    Copy-Item -Path "vel.exe" -Destination "$env:ProgramFiles\Velvet\vel.exe"

    # Install Python dependencies
    Write-Host "Installing Python dependencies..."
    pip install -r requirements.txt

    # Create .velvet-library directory
    New-Item -Path "$env:ProgramFiles\.velvet-library" -ItemType Directory -Force
    Write-Host "Created $env:ProgramFiles\.velvet-library"

    # Copy Python hooks and configuration
    New-Item -Path "$env:ProgramFiles\Velvet\hooks" -ItemType Directory -Force
    Copy-Item -Path "src\hooks\*" -Destination "$env:ProgramFiles\Velvet\hooks" -Recurse
    Copy-Item -Path "src\config\vel.config" -Destination "$env:ProgramFiles\Velvet\"
}

# Function to update system PATH
function Update-Path {
    $velvetPath = "$env:ProgramFiles\Velvet"
    $currentPath = [Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
    if ($currentPath -notlike "*$velvetPath*") {
        [Environment]::SetEnvironmentVariable("Path", "$currentPath;$velvetPath", [System.EnvironmentVariableTarget]::Machine)
        Write-Host "Added $velvetPath to system PATH"
    }
}

# Main execution
Write-Host "Starting Velvet installation..."

# Install dependencies
Install-Dependencies

# Install Velvet
Install-Velvet

# Update PATH
Update-Path

Write-Host "Velvet installed successfully!"
Write-Host "You can now use 'vel' command to manage Velvet projects."
Write-Host "Try 'vel --help' for available commands."

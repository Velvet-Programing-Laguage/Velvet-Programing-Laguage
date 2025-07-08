# install.ps1 - Installation script for Velvet Programming Language on Windows
# Installs Java, Rust, Go, Node.js, Tauri CLI, and builds Velvet from the specified repository

# Logging functions
function Log-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Log-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
    exit 1
}

function Log-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

# 1. Detect operating system
$OS = $PSVersionTable.Platform
if ($OS -eq "Win32NT") {
    Log-Info "Detected system: Windows"
} else {
    Log-Error "Unsupported operating system: $OS"
}

# 2. Determine package manager (winget or Chocolatey)
if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
    Log-Warn "winget not available. Using Chocolatey..."
    if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
        Log-Info "Installing Chocolatey..."
        Set-ExecutionPolicy Bypass -Scope Process -Force
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
    }
    $PKG_MANAGER = "choco"
} else {
    $PKG_MANAGER = "winget"
}

# 3. Install Java (OpenJDK 17)
if (-not (Get-Command java -ErrorAction SilentlyContinue)) {
    Log-Info "Installing Java (OpenJDK 17)..."
    if ($PKG_MANAGER -eq "winget") {
        winget install -e --id EclipseAdoptium.Temurin.17.JDK --accept-package-agreements --accept-source-agreements
    } else {
        choco install -y openjdk17
    }
} else {
    Log-Info "Java already installed: $(java -version 2>&1 | Select-Object -First 1)"
}

# 4. Install Rust
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Log-Info "Installing Rust..."
    Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe
    .\rustup-init.exe -y
    Remove-Item rustup-init.exe
    $env:PATH += ";$HOME\.cargo\bin"
    [Environment]::SetEnvironmentVariable("Path", $env:PATH, [System.EnvironmentVariableTarget]::User)
} else {
    Log-Info "Rust already installed: $(rustc --version)"
}

# 5. Install Go
if (-not (Get-Command go -ErrorAction SilentlyContinue)) {
    Log-Info "Installing Go..."
    if ($PKG_MANAGER -eq "winget") {
        winget install -e --id GoLang.Go --accept-package-agreements --accept-source-agreements
    } else {
        choco install -y golang
    }
} else {
    Log-Info "Go already installed: $(go version)"
}

# 6. Install Node.js and Tauri CLI
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Log-Info "Installing Node.js..."
    if ($PKG_MANAGER -eq "winget") {
        winget install -e --id OpenJS.NodeJS --accept-package-agreements --accept-source-agreements
    } else {
        choco install -y nodejs
    }
} else {
    Log-Info "Node.js already installed: $(node --version)"
}

if (-not (Get-Command tauri -ErrorAction SilentlyContinue)) {
    Log-Info "Installing Tauri CLI..."
    npm install -g @tauri-apps/cli
} else {
    Log-Info "Tauri CLI already installed: $(tauri --version)"
}

# 7. Clone and build Velvet
$VELVET_REPO = "https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Laguage.git"
$VELVET_DIR = "Velvet-Programing-Laguage"

if (Test-Path $VELVET_DIR) {
    Log-Warn "Directory $VELVET_DIR already exists. Updating..."
    Set-Location $VELVET_DIR
    git pull
} else {
    Log-Info "Cloning Velvet repository..."
    git clone $VELVET_REPO
    Set-Location $VELVET_DIR
}

# 8. Build Velvet components
Log-Info "Building Velvet core (Rust)..."
Set-Location core
cargo build --release
Set-Location ..

Log-Info "Building Velvet CLI (Go)..."
Set-Location cli
go build -o vel.exe
New-Item -Path "$env:ProgramFiles\Velvet" -ItemType Directory -Force
Copy-Item vel.exe -Destination "$env:ProgramFiles\Velvet\vel.exe"
$env:PATH += ";$env:ProgramFiles\Velvet"
[Environment]::SetEnvironmentVariable("Path", $env:PATH, [System.EnvironmentVariableTarget]::User)
Set-Location ..

Log-Info "Building Velvet GUI (Tauri)..."
Set-Location gui
npm install
npm run tauri build
Set-Location ..

# 9. Verify installation
if (Get-Command vel -ErrorAction SilentlyContinue) {
    Log-Info "Velvet installed successfully! Version: $(vel --version)"
    Log-Info "Example usage: vel init; vel start"
} else {
    Log-Error "Velvet installation failed. Check logs above."
}

Log-Info "Installation completed successfully!"

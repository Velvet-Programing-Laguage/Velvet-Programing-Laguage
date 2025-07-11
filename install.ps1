# Velvet Install Script for Windows
# Author: VELVET TEAM
# Date: 2025-07-11

#Requires -Version 5.1

$ErrorActionPreference = "Stop"

$LogFile = "C:\Temp\velvet_install.log"
$VelvetRepo = "https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Laguage.git"
$VelvetDir = "Velvet-Programing-Laguage"

function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $TimeStamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $Color = switch ($Level) {
        "INFO" { "Green" }
        "WARN" { "Yellow" }
        "ERROR" { "Red" }
        default { "White" }
    }
    Write-Host "[$Level] $Message" -ForegroundColor $Color
    Add-Content -Path $LogFile -Value "[$Level] $TimeStamp $Message"
    if ($Level -eq "ERROR") { exit 1 }
}

function Show-Banner {
@"
  ___      ___ _______   ___       ___      ___ _______  _________   
 |\  \    /  /|\  ___ \ |\  \     |\  \    /  /|\  ___ \|\___   ___\ 
 \ \  \  /  / | \   __/|\ \  \    \ \  \  /  / | \   __/\|___ \  \_| 
  \ \  \/  / / \ \  \_|/_\ \  \    \ \  \/  / / \ \  \_|/__  \ \  \  
   \ \    / /   \ \  \_|\ \ \  \____\ \    / /   \ \  \_|\ \  \ \  \ 
    \ \__/ /     \ \_______\ \_______\ \__/ /     \ \_______\  \ \__\
     \|__|/       \|_______|\|_______|\|__|/       \|_______|   \|__|

"@
}

function Spinner {
    param([scriptblock]$Script)
    $Job = Start-Job $Script
    $Chars = @('|', '/', '-', '\')
    while ($Job.State -eq 'Running') {
        foreach ($Char in $Chars) {
            Write-Host -NoNewline "$Char `r"
            Start-Sleep -Milliseconds 100
        }
    }
    Receive-Job $Job
    Remove-Job $Job
}

function Check-Command {
    param([string]$Command)
    $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

function Install-Rust {
    if (-not (Check-Command "rustc")) {
        Write-Log "Installing Rust..."
        Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe
        Start-Process -FilePath .\rustup-init.exe -ArgumentList "-y" -Wait
        Remove-Item .\rustup-init.exe
        $Env:Path += ";$Env:USERPROFILE\.cargo\bin"
        Write-Log "Rust installed."
    } else {
        Write-Log "Rust found: $(rustc --version)"
    }
}

function Install-Node {
    if (-not (Check-Command "node")) {
        Write-Log "Please install Node.js (v18+) manually from https://nodejs.org and re-run the script." "ERROR"
    } else {
        Write-Log "Node.js found: $(node --version)"
    }
}

function Install-Java {
    if (-not (Check-Command "java")) {
        Write-Log "Please install OpenJDK 17 manually from https://adoptium.net and re-run the script." "ERROR"
    } else {
        Write-Log "Java found: $(java -version 2>&1 | Select-Object -First 1)"
    }
}

function Install-Tauri {
    if (-not (Check-Command "tauri")) {
        Write-Log "Installing Tauri CLI globally..."
        npm install -g @tauri-apps/cli | Out-Null
        Write-Log "Tauri CLI installed."
    } else {
        Write-Log "Tauri CLI found: $(tauri --version)"
    }
}

function Build-Velvet {
    Set-Location $VelvetDir

    Write-Log "Building Velvet core (Rust)..."
    Set-Location core
    cargo build --release
    Set-Location ..

    Write-Log "Building Velvet GUI (Tauri)..."
    Set-Location gui
    npm install
    npm run build
    Set-Location ..
    
    Set-Location ..

    Write-Log "Velvet build complete."
}

function Main {
    if (-not (Test-Path "C:\Temp")) { New-Item -Path "C:\Temp" -ItemType Directory }
    Remove-Item $LogFile -ErrorAction SilentlyContinue

    Show-Banner
    Write-Log "Starting installation of Velvet Programming Language"

    if (Test-Path $VelvetDir) {
        Write-Log "Removing existing directory $VelvetDir..." "WARN"
        Remove-Item $VelvetDir -Recurse -Force
    }

    Write-Log "Cloning Velvet repository..."
    Spinner { git clone $VelvetRepo }

    Install-Java
    Install-Rust
    Install-Node
    Install-Tauri

    Build-Velvet

    Write-Log "Velvet Programming Language installed successfully!" "INFO"
    Write-Host "Installation complete!" -ForegroundColor Green
}

Main

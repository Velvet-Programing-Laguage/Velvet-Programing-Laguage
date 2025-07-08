#!/bin/bash

# install.sh - Installation script for Velvet Programming Language
# Detects OS, installs dependencies (Java, Rust, Go, Node.js, Tauri CLI), and builds Velvet from the specified repository

set -e # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No color

# Logging functions
log() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# 1. Detect operating system and distribution
OS=$(uname -s)
case "$OS" in
    Linux*)  DISTRO=$( [ -f /etc/os-release ] && . /etc/os-release && echo "$ID" || echo "unknown" );;
    Darwin*) DISTRO="macos";;
    *)       error "Unsupported operating system: $OS";;
esac

log "Detected system: $OS ($DISTRO)"

# 2. Determine package manager
if [ "$DISTRO" = "macos" ]; then
    if ! command -v brew &> /dev/null; then
        log "Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        # Ensure Homebrew is in PATH
        echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zshrc
        eval "$(/opt/homebrew/bin/brew shellenv)"
    fi
    PKG_MANAGER="brew"
elif [ "$DISTRO" = "ubuntu" ] || [ "$DISTRO" = "debian" ]; then
    PKG_MANAGER="apt"
    log "Updating apt..."
    sudo apt update -y
elif [ "$DISTRO" = "fedora" ]; then
    PKG_MANAGER="dnf"
    log "Updating dnf..."
    sudo dnf check-update -y
elif [ "$DISTRO" = "arch" ] || [ "$DISTRO" = "manjaro" ]; then
    PKG_MANAGER="pacman"
    log "Updating pacman..."
    sudo pacman -Syu --noconfirm
else
    warn "Unknown distribution: $DISTRO. Attempting to proceed with manual installations."
    PKG_MANAGER="none"
fi

# 3. Install Java (OpenJDK 17)
if ! command -v java &> /dev/null; then
    log "Installing Java (OpenJDK 17)..."
    case $PKG_MANAGER in
        apt) sudo apt install -y openjdk-17-jdk ;;
        dnf) sudo dnf install -y java-17-openjdk-devel ;;
        pacman) sudo pacman -S --noconfirm jdk17-openjdk ;;
        brew) brew install openjdk@17 ;;
        *) 
            warn "Manual Java installation required. Downloading OpenJDK 17..."
            curl -LO https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz
            sudo mkdir -p /usr/local/java
            sudo tar -xzf openjdk-17.0.2_linux-x64_bin.tar.gz -C /usr/local/java
            echo 'export JAVA_HOME=/usr/local/java/jdk-17.0.2' >> ~/.bashrc
            echo 'export PATH=$JAVA_HOME/bin:$PATH' >> ~/.bashrc
            rm openjdk-17.0.2_linux-x64_bin.tar.gz
            ;;
    esac
else
    log "Java already installed: $(java -version 2>&1 | head -n 1)"
fi

# 4. Install Rust
if ! command -v rustc &> /dev/null; then
    log "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    log "Rust already installed: $(rustc --version)"
fi

# 5. Install Go
if ! command -v go &> /dev/null; then
    log "Installing Go..."
    GO_VERSION="1.21.0"
    if [ "$OS" = "Linux" ]; then
        curl -LO https://golang.org/dl/go${GO_VERSION}.linux-amd64.tar.gz
        sudo tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz
        rm go${GO_VERSION}.linux-amd64.tar.gz
    elif [ "$OS" = "Darwin" ]; then
        brew install go
    fi
    echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.bashrc
    export PATH=$PATH:/usr/local/go/bin
else
    log "Go already installed: $(go version)"
fi

# 6. Install Node.js and Tauri CLI
if ! command -v node &> /dev/null; then
    log "Installing Node.js..."
    case $PKG_MANAGER in
        apt) 
            curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
            sudo apt install -y nodejs ;;
        dnf) sudo dnf install -y nodejs ;;
        pacman) sudo pacman -S --noconfirm nodejs npm ;;
        brew) brew install node ;;
        *) 
            warn "Manual Node.js installation required. Downloading Node.js..."
            curl -LO https://nodejs.org/dist/v18.20.4/node-v18.20.4-linux-x64.tar.xz
            sudo mkdir -p /usr/local/node
            sudo tar -xJf node-v18.20.4-linux-x64.tar.xz -C /usr/local/node
            echo 'export PATH=/usr/local/node/node-v18.20.4-linux-x64/bin:$PATH' >> ~/.bashrc
            export PATH=/usr/local/node/node-v18.20.4-linux-x64/bin:$PATH
            rm node-v18.20.4-linux-x64.tar.xz
            ;;
    esac
else
    log "Node.js already installed: $(node --version)"
fi

if ! command -v tauri &> /dev/null; then
    log "Installing Tauri CLI..."
    npm install -g @tauri-apps/cli
else
    log "Tauri CLI already installed: $(tauri --version)"
fi

# 7. Clone and build Velvet
VELVET_REPO="https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Laguage.git"
VELVET_DIR="Velvet-Programing-Laguage"

if [ -d "$VELVET_DIR" ]; then
    warn "Directory $VELVET_DIR already exists. Updating..."
    cd $VELVET_DIR
    git pull
else
    log "Cloning Velvet repository..."
    git clone $VELVET_REPO
    cd $VELVET_DIR
fi

# 8. Build Velvet components
log "Building Velvet core (Rust)..."
cd core
cargo build --release
cd ..

log "Building Velvet CLI (Go)..."
cd cli
go build -o vel
sudo mv vel /usr/local/bin/vel
cd ..

log "Building Velvet GUI (Tauri)..."
cd gui
npm install
npm run tauri build
cd ..

# 9. Verify installation
if command -v vel &> /dev/null; then
    log "Velvet installed successfully! Version: $(vel --version)"
    log "Example usage: vel init && vel start"
else
    error "Velvet installation failed. Check logs above."
fi

log "Installation completed successfully!"

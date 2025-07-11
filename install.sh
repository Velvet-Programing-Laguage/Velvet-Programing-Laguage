#!/bin/bash

set -euo pipefail

LOGFILE="/tmp/velvet_install.log"
VELVET_REPO="https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Laguage.git"
VELVET_DIR="Velvet-Programing-Laguage"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

ascii_banner() {
cat << "EOF"
  ___      ___ _______   ___       ___      ___ _______  _________   
|\  \    /  /|\  ___ \ |\  \     |\  \    /  /|\  ___ \|\___   ___\ 
\ \  \  /  / | \   __/|\ \  \    \ \  \  /  / | \   __/\|___ \  \_| 
 \ \  \/  / / \ \  \_|/_\ \  \    \ \  \/  / / \ \  \_|/__  \ \  \  
  \ \    / /   \ \  \_|\ \ \  \____\ \    / /   \ \  \_|\ \  \ \  \ 
   \ \__/ /     \ \_______\ \_______\ \__/ /     \ \_______\  \ \__\
    \|__|/       \|_______|\|_______|\|__|/       \|_______|   \|__|
                                                             
EOF
}

spinner() {
    local pid=$1
    local delay=0.08
    local spinstr='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
    tput civis
    while kill -0 "$pid" 2>/dev/null; do
        for ((i=0; i<${#spinstr}; i++)); do
            printf " [%b]  " "${spinstr:i:1}"
            sleep $delay
            printf "\b\b\b\b\b\b"
        done
    done
    tput cnorm
    printf "       \b\b\b\b\b\b"
}

log() {
    echo -e "${GREEN}[INFO]${NC} $1"
    echo "[INFO] $(date '+%Y-%m-%d %H:%M:%S') $1" >> "$LOGFILE"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
    echo "[WARN] $(date '+%Y-%m-%d %H:%M:%S') $1" >> "$LOGFILE"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    echo "[ERROR] $(date '+%Y-%m-%d %H:%M:%S') $1" >> "$LOGFILE"
    exit 1
}

ensure_root() {
    if [ "$EUID" -ne 0 ]; then
        echo -e "${YELLOW}Script requires root privileges. Restarting with sudo...${NC}"
        chmod a+x "$0"
        exec sudo bash "$0" "$@"
    fi
}

detect_distro() {
    OS=$(uname -s)
    if [ "$OS" = "Linux" ]; then
        if [ -f /etc/os-release ]; then
            . /etc/os-release
            DISTRO=$ID
        else
            DISTRO="unknown"
        fi
    elif [ "$OS" = "Darwin" ]; then
        DISTRO="macos"
    else
        error "Unsupported OS: $OS"
    fi
    log "Detected OS: $OS, Distribution: $DISTRO"
}

install_packages() {
    local pkgs="$1"
    case "$PKG_MANAGER" in
        apt)
            log "Updating apt and installing packages: $pkgs"
            apt update -y &>> "$LOGFILE"
            apt install -y $pkgs &>> "$LOGFILE"
            ;;
        dnf)
            log "Updating dnf and installing packages: $pkgs"
            dnf check-update -y &>> "$LOGFILE"
            dnf install -y $pkgs &>> "$LOGFILE"
            ;;
        pacman)
            log "Updating pacman and installing packages: $pkgs"
            pacman -Sy --noconfirm $pkgs &>> "$LOGFILE"
            ;;
        zypper)
            log "Installing packages with zypper: $pkgs"
            zypper refresh &>> "$LOGFILE"
            zypper install -y $pkgs &>> "$LOGFILE"
            ;;
        brew)
            log "Installing packages with brew: $pkgs"
            brew install $pkgs &>> "$LOGFILE"
            ;;
        *)
            warn "Unknown package manager, manual installation may be required: $pkgs"
            ;;
    esac
}

install_rust() {
    if ! command -v rustc &>/dev/null; then
        log "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y &>> "$LOGFILE" &
        spinner $!
        source "$HOME/.cargo/env"
        log "Rust installed."
    else
        log "Rust found: $(rustc --version)"
    fi
}

install_go() {
    if ! command -v go &>/dev/null; then
        log "Installing Go..."
        GO_VERSION="1.21.0"
        if [ "$OS" = "Linux" ]; then
            curl -LO https://golang.org/dl/go${GO_VERSION}.linux-amd64.tar.gz &>> "$LOGFILE" &
            spinner $!
            rm -rf /usr/local/go
            tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz
            rm go${GO_VERSION}.linux-amd64.tar.gz
            export PATH=$PATH:/usr/local/go/bin
        elif [ "$OS" = "Darwin" ]; then
            install_packages "go"
        fi
        log "Go installed."
    else
        log "Go found: $(go version)"
    fi
}

install_node() {
    if ! command -v node &>/dev/null; then
        log "Installing Node.js..."
        case "$PKG_MANAGER" in
            apt)
                curl -fsSL https://deb.nodesource.com/setup_18.x | bash - &>> "$LOGFILE"
                install_packages "nodejs npm"
                ;;
            dnf|pacman|zypper|brew)
                install_packages "nodejs npm"
                ;;
            *)
                warn "Manual Node.js installation required."
                ;;
        esac
        log "Node.js installed."
    else
        log "Node.js found: $(node --version)"
    fi
}

install_java() {
    if ! command -v java &>/dev/null; then
        log "Installing OpenJDK 17..."
        case "$PKG_MANAGER" in
            apt) install_packages "openjdk-17-jdk" ;;
            dnf) install_packages "java-17-openjdk-devel" ;;
            pacman) install_packages "jdk17-openjdk" ;;
            zypper) install_packages "java-17-openjdk-devel" ;;
            brew) install_packages "openjdk@17" ;;
            *) warn "Manual Java installation required." ;;
        esac
        log "Java installed."
    else
        log "Java found: $(java -version 2>&1 | head -n1)"
    fi
}

install_tauri() {
    if ! command -v tauri &>/dev/null; then
        log "Installing Tauri CLI..."
        npm install -g @tauri-apps/cli &>> "$LOGFILE" &
        spinner $!
        log "Tauri CLI installed."
    else
        log "Tauri CLI found: $(tauri --version)"
    fi
}

build_velvet() {
    cd "$VELVET_DIR" || error "Failed to cd into $VELVET_DIR"

    log "Building Velvet core (Rust)..."
    cd core
    cargo build --release &>> "$LOGFILE" &
    spinner $!
    cd ..

    log "Building Velvet CLI (Go)..."
    cd cli
    go build -o vel &>> "$LOGFILE" &
    spinner $!
    mv vel /usr/local/bin/vel
    cd ..

    log "Building Velvet GUI (Tauri)..."
    cd gui
    npm install &>> "$LOGFILE" &
    spinner $!
    npm run build &>> "$LOGFILE" &
    spinner $!
    cd ..

    log "Velvet build complete."
}

main() {
    ensure_root "$@"
    > "$LOGFILE"

    ascii_banner

    log "Starting installation of Velvet Programming Language"

    if [ -d "$VELVET_DIR" ]; then
        warn "Directory $VELVET_DIR exists, removing..."
        rm -rf "$VELVET_DIR"
    fi

    log "Cloning Velvet repository..."
    git clone "$VELVET_REPO" &>> "$LOGFILE" &
    spinner $!

    detect_distro

    case "$DISTRO" in
        ubuntu|debian) PKG_MANAGER="apt" ;;
        fedora) PKG_MANAGER="dnf" ;;
        arch|manjaro) PKG_MANAGER="pacman" ;;
        opensuse*|suse) PKG_MANAGER="zypper" ;;
        macos) PKG_MANAGER="brew" ;;
        *)
            PKG_MANAGER="none"
            warn "Unknown distro, automatic package installation skipped"
            ;;
    esac

    install_java
    install_rust
    install_go
    install_node
    install_tauri

    build_velvet

    log "Velvet Programming Language installed successfully!"
    echo -e "${GREEN}Installation complete!${NC}"
}

main "$@"

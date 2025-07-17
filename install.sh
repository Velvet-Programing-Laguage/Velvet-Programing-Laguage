#!/bin/bash

# install.sh
# Script to install Velvet programming language and its dependencies

set -e

# Logging function
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# Error handling function
handle_error() {
    log "ERROR: $1"
    exit 1
}

# Function to check if sudo is required and elevate privileges
check_sudo() {
    if [[ $EUID -ne 0 ]]; then
        log "This script requires root privileges. Elevating to sudo..."
        exec sudo "$0" "$@"
    fi
}

# Function to set executable permissions
set_permissions() {
    log "Setting executable permissions for $0"
    chmod +x "$0" || handle_error "Failed to set executable permissions for $0"
}

#Clone repository
git clone https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Language.git
cd /home/$(whoami)/Velvet-Programing-Language

# Function to detect the OS and distribution
detect_os() {
    log "Detecting OS and distribution..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if [ -f /etc/os-release ]; then
            . /etc/os-release
            DISTRO=$ID
            VERSION=$VERSION_ID
        else
            handle_error "Cannot detect Linux distribution! /etc/os-release not found."
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        DISTRO="macos"
    else
        handle_error "Unsupported OS: $OSTYPE"
    fi
    log "Detected OS: $DISTRO (Version: ${VERSION:-N/A})"
}

# Function to check if a package is installed
check_package() {
    local pkg=$1
    local cmd=$2
    if command -v "$cmd" &>/dev/null; then
        log "$pkg is already installed."
        return 0
    else
        log "$pkg is not installed."
        return 1
    fi
}

# Function to install system dependencies
install_system_deps() {
    log "Installing system dependencies for $DISTRO..."
    case $DISTRO in
        "ubuntu"|"debian")
            sudo apt-get update || handle_error "Failed to update package lists"
            local packages=(curl git build-essential python3 python3-venv python3-pip ruby rubygems rustc cargo golang openjdk-11-jdk nodejs npm elixir)
            for pkg in "${packages[@]}"; do
                check_package "$pkg" "$pkg" || sudo apt-get install -y "$pkg" || handle_error "Failed to install $pkg"
            done
            # Crystal (third-party)
            if ! check_package "crystal" "crystal"; then
                curl -fsSL https://crystal-lang.org/install.sh | sudo bash || handle_error "Failed to install Crystal"
            fi
            # Kotlin
            if ! check_package "kotlin" "kotlin"; then
                sudo snap install --classic kotlin || handle_error "Failed to install Kotlin"
            fi
            ;;
        "fedora")
            local packages=(curl git gcc python3 python3-pip ruby rubygems rust cargo golang java-11-openjdk nodejs npm elixir kotlin)
            for pkg in "${packages[@]}"; do
                check_package "$pkg" "$pkg" || sudo dnf install -y "$pkg" || handle_error "Failed to install $pkg"
            done
            # Crystal
            if ! check_package "crystal" "crystal"; then
                curl -fsSL https://crystal-lang.org/install.sh | sudo bash || handle_error "Failed to install Crystal"
            fi
            ;;
        "bazzite"|"fedora-silverblue")
            local packages=(curl git gcc python3 python3-pip ruby rubygems rust cargo golang java-11-openjdk nodejs npm elixir kotlin)
            for pkg in "${packages[@]}"; do
                if ! rpm -q "$pkg" &>/dev/null; then
                    log "Installing $pkg..."
                    sudo rpm-ostree install -y "$pkg" || handle_error "Failed to install $pkg"
                else
                    log "$pkg is already installed."
                fi
            done
            # Crystal
            if ! check_package "crystal" "crystal"; then
                curl -fsSL https://crystal-lang.org/install.sh | sudo bash || handle_error "Failed to install Crystal"
            fi
            log "System reboot may be required to apply changes. Run 'sudo systemctl reboot' after installation."
            ;;
        "arch"|"manjaro")
            sudo pacman -Syu --noconfirm || handle_error "Failed to update package lists"
            local packages=(curl git base-devel python python-pip ruby rust go jdk11-openjdk nodejs npm elixir crystal kotlin)
            for pkg in "${packages[@]}"; do
                check_package "$pkg" "$pkg" || sudo pacman -S --noconfirm "$pkg" || handle_error "Failed to install $pkg"
            done
            ;;
        "gentoo")
            sudo emerge --sync || handle_error "Failed to sync Portage"
            local packages=(net-misc/curl dev-vcs/git sys-devel/gcc dev-lang/python dev-lang/ruby dev-lang/rust dev-lang/go virtual/jdk nodejs dev-lang/elixir dev-lang/crystal dev-lang/kotlin)
            for pkg in "${packages[@]}"; do
                check_package "${pkg##*/}" "${pkg##*/}" || sudo emerge -av "$pkg" || handle_error "Failed to install $pkg"
            done
            ;;
        "slackware")
            sudo slackpkg update || handle_error "Failed to update Slackware packages"
            local packages=(curl git gcc python3 ruby rust go jdk nodejs)
            for pkg in "${packages[@]}"; do
                check_package "$pkg" "$pkg" || sudo slackpkg install "$pkg" || handle_error "Failed to install $pkg"
            done
            # Crystal
            if ! check_package "crystal" "crystal"; then
                curl -fsSL https://crystal-lang.org/install.sh | sudo bash || handle_error "Failed to install Crystal"
            fi
            # Elixir
            if ! check_package "elixir" "elixir"; then
                wget https://github.com/elixir-lang/elixir/releases/download/v1.15.7/elixir-otp-26.zip || handle_error "Failed to download Elixir"
                unzip elixir-otp-26.zip -d /usr/local/elixir || handle_error "Failed to unzip Elixir"
                echo 'export PATH="$PATH:/usr/local/elixir/bin"' >> ~/.bashrc
            fi
            # Kotlin
            check_package "kotlin" "kotlin" || sudo slackpkg install kotlin || handle_error "Failed to install Kotlin"
            ;;
        "macos")
            brew update || handle_error "Failed to update Homebrew"
            local packages=(curl git python ruby rust go openjdk@11 node elixir crystal kotlin)
            for pkg in "${packages[@]}"; do
                check_package "$pkg" "$pkg" || brew install "$pkg" || handle_error "Failed to install $pkg"
            done
            ;;
        *)
            handle_error "Unsupported distribution: $DISTRO"
            ;;
    esac
    log "System dependencies installed successfully."
}

# Function to clone Velvet repository
clone_velvet_repo() {
    log "Cloning Velvet repository..."
    if [ ! -d "Velvet-Programing-Language" ]; then
        git clone https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Language.git || handle_error "Failed to clone Velvet repository"
        cd Velvet-Programing-Language
    else
        cd Velvet-Programing-Language
        git pull origin main || handle_error "Failed to update Velvet repository"
    fi
}

# Function to compile and install Velvet
install_velvet() {
    log "Installing Velvet..."
    # Compile Rust core
    log "Compiling Velvet core (Rust)..."
    cargo build --release || handle_error "Failed to compile Velvet core"
    sudo cp target/release/velvet /usr/bin/velvet_core || handle_error "Failed to install Velvet core"

    # Compile Go CLI
    log "Compiling Velvet CLI (Go)..."
    go build -o vel src/cli/main.go || handle_error "Failed to compile Velvet CLI"
    sudo cp vel /usr/bin/vel || handle_error "Failed to install Velvet CLI"

    # Install Python dependencies
    log "Installing Python dependencies..."
    pip3 install -r requirements.txt || handle_error "Failed to install Python dependencies"

    # Create .velvet-library directory
    log "Creating /usr/lib/.velvet-library..."
    sudo mkdir -p /usr/lib/.velvet-library || handle_error "Failed to create /usr/lib/.velvet-library"
    sudo chmod 755 /usr/lib/.velvet-library || handle_error "Failed to set permissions for /usr/lib/.velvet-library"

    # Copy Python hooks and configuration
    log "Copying hooks and configuration..."
    sudo mkdir -p /usr/lib/velvet/hooks || handle_error "Failed to create /usr/lib/velvet/hooks"
    sudo cp -r src/hooks/* /usr/lib/velvet/hooks/ || handle_error "Failed to copy hooks"
    sudo cp src/config/vel.config /usr/lib/velvet/ || handle_error "Failed to copy configuration"
    sudo chmod -R 755 /usr/lib/velvet || handle_error "Failed to set permissions for /usr/lib/velvet"
}

# Main execution
log "Starting Velvet installation..."

# Check for sudo privileges
check_sudo "$@"

# Set permissions
set_permissions

# Detect OS
detect_os

# Install system dependencies
install_system_deps

# Clone Velvet repository
clone_velvet_repo

# Install Velvet
install_velvet

log "Velvet installed successfully!"
log "You can now use 'vel' command to manage Velvet projects."
log "Try 'vel --help' for available commands."

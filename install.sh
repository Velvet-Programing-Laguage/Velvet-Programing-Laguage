#!/bin/bash

# install.sh
# Script to install Velvet programming language and its dependencies

set -e

# ANSI color codes for logging
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

#Clone repository
git clone https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Language.git
cd /home/$(whoami)/Velvet-Programing-Language

# Logging function
log() {
    local level=$1
    local message=$2
    case $level in
        "INFO")  echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] ${YELLOW}INFO: $message${NC}" ;;
        "SUCCESS") echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] ${GREEN}SUCCESS: $message${NC}" ;;
        "ERROR") echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] ${RED}ERROR: $message${NC}" >&2 ;;
    esac
}

# Error handling function
handle_error() {
    log "ERROR" "$1"
    exit 1
}

# Cleanup function for interruptions
cleanup() {
    log "INFO" "Installation interrupted. Cleaning up..."
    if [ -d "Velvet-Programing-Language" ]; then
        log "INFO" "Removing temporary Velvet repository..."
        rm -rf Velvet-Programing-Language
    fi
    exit 1
}

# Trap interruptions (Ctrl+C)
trap cleanup SIGINT SIGTERM

# Function to display a simple progress bar
progress_bar() {
    local duration=$1
    local message=$2
    log "INFO" "$message"
    for ((i=0; i<duration; i++)); do
        printf "."
        sleep 0.1
    done
    printf "\n"
}

# Function to check if sudo is required and elevate privileges
check_sudo() {
    if [[ $EUID -ne 0 ]]; then
        log "INFO" "This script requires root privileges. Elevating to sudo..."
        exec sudo "$0" "$@"
    fi
}

# Function to set executable permissions
set_permissions() {
    log "INFO" "Setting executable permissions for $0"
    chmod +x "$0" || handle_error "Failed to set executable permissions for $0"
}

# Function to detect the OS and distribution
detect_os() {
    log "INFO" "Detecting OS and distribution..."
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
    log "SUCCESS" "Detected OS: $DISTRO (Version: ${VERSION:-N/A})"
}

# Function to check if a package is installed
check_package() {
    local pkg=$1
    local cmd=$2
    if command -v "$cmd" &>/dev/null || rpm -q "$pkg" &>/dev/null 2>/dev/null; then
        log "SUCCESS" "$pkg is already installed."
        return 0
    else
        log "INFO" "$pkg is not installed."
        return 1
    fi
}

# Function to check package availability for rpm-ostree
check_rpm_ostree_package() {
    local pkg=$1
    if rpm-ostree status | grep -q "$pkg"; then
        log "SUCCESS" "$pkg is already layered."
        return 0
    elif dnf list available "$pkg" &>/dev/null; then
        log "INFO" "$pkg is available for installation."
        return 0
    else
        log "ERROR" "$pkg is not available in repositories."
        return 1
    fi
}

# Function to install system dependencies
install_system_deps() {
    log "INFO" "Installing system dependencies for $DISTRO..."
    case $DISTRO in
        "ubuntu"|"debian")
            sudo apt-get update -q || handle_error "Failed to update package lists"
            local packages=(curl git build-essential python3 python3-venv python3-pip ruby rubygems rustc cargo golang openjdk-11-jdk nodejs npm elixir)
            for pkg in "${packages[@]}"; do
                check_package "$pkg" "$pkg" || { progress_bar 5 "Installing $pkg"; sudo apt-get install -y "$pkg" || handle_error "Failed to install $pkg"; }
            done
            # Crystal
            if ! check_package "crystal" "crystal"; then
                progress_bar 5 "Installing Crystal"
                curl -fsSL https://crystal-lang.org/install.sh | sudo bash || handle_error "Failed to install Crystal"
            fi
            # Kotlin
            if ! check_package "kotlin" "kotlin"; then
                progress_bar 5 "Installing Kotlin"
                sudo snap install --classic kotlin || handle_error "Failed to install Kotlin"
            fi
            ;;
        "fedora")
            local packages=(curl git gcc python3 python3-pip ruby rubygems rust cargo golang java-11-openjdk-headless nodejs npm elixir kotlin)
            for pkg in "${packages[@]}"; do
                check_package "$pkg" "$pkg" || { progress_bar 5 "Installing $pkg"; sudo dnf install -y "$pkg" || handle_error "Failed to install $pkg"; }
            done
            # Crystal
            if ! check_package "crystal" "crystal"; then
                progress_bar 5 "Installing Crystal"
                curl -fsSL https://crystal-lang.org/install.sh | sudo bash || handle_error "Failed to install Crystal"
            fi
            ;;
        "bazzite"|"fedora-silverblue")
            local packages=(curl git gcc python3 python3-pip ruby rubygems rust cargo golang java-11-openjdk-headless nodejs npm elixir kotlin)
            for pkg in "${packages[@]}"; do
                if ! check_package "$pkg" "$pkg"; then
                    if check_rpm_ostree_package "$pkg"; then
                        progress_bar 10 "Installing $pkg with rpm-ostree"
                        for attempt in {1..3}; do
                            if sudo rpm-ostree install -y "$pkg"; then
                                log "SUCCESS" "$pkg installed successfully."
                                break
                            else
                                log "INFO" "Retry $attempt: Failed to install $pkg. Retrying..."
                                sleep 2
                            fi
                            [[ $attempt -eq 3 ]] && handle_error "Failed to install $pkg after 3 attempts"
                        done
                    else
                        handle_error "Package $pkg not available for installation"
                    fi
                fi
            done
            # Crystal
            if ! check_package "crystal" "crystal"; then
                progress_bar 5 "Installing Crystal"
                curl -fsSL https://crystal-lang.org/install.sh | sudo bash || handle_error "Failed to install Crystal"
            fi
            log "INFO" "System reboot required to apply changes. Run 'sudo systemctl reboot' after installation."
            ;;
        "arch"|"manjaro")
            sudo pacman -Syu --noconfirm || handle_error "Failed to update package lists"
            local packages=(curl git base-devel python python-pip ruby rust go jdk11-openjdk nodejs npm elixir crystal kotlin)
            for pkg in "${packages[@]}"; do
                check_package "$pkg" "$pkg" || { progress_bar 5 "Installing $pkg"; sudo pacman -S --noconfirm "$pkg" || handle_error "Failed to install $pkg"; }
            done
            ;;
        "gentoo")
            sudo emerge --sync || handle_error "Failed to sync Portage"
            local packages=(net-misc/curl dev-vcs/git sys-devel/gcc dev-lang/python dev-lang/ruby dev-lang/rust dev-lang/go virtual/jdk nodejs dev-lang/elixir dev-lang/crystal dev-lang/kotlin)
            for pkg in "${packages[@]}"; do
                check_package "${pkg##*/}" "${pkg##*/}" || { progress_bar 10 "Installing $pkg"; sudo emerge -av "$pkg" || handle_error "Failed to install $pkg"; }
            done
            ;;
        "slackware")
            sudo slackpkg update || handle_error "Failed to update Slackware packages"
            local packages=(curl git gcc python3 ruby rust go jdk nodejs)
            for pkg in "${packages[@]}"; do
                check_package "$pkg" "$pkg" || { progress_bar 5 "Installing $pkg"; sudo slackpkg install "$pkg" || handle_error "Failed to install $pkg"; }
            done
            # Crystal
            if ! check_package "crystal" "crystal"; then
                progress_bar 5 "Installing Crystal"
                curl -fsSL https://crystal-lang.org/install.sh | sudo bash || handle_error "Failed to install Crystal"
            fi
            # Elixir
            if ! check_package "elixir" "elixir"; then
                progress_bar 5 "Installing Elixir"
                wget https://github.com/elixir-lang/elixir/releases/download/v1.15.7/elixir-otp-26.zip || handle_error "Failed to download Elixir"
                unzip elixir-otp-26.zip -d /usr/local/elixir || handle_error "Failed to unzip Elixir"
                echo 'export PATH="$PATH:/usr/local/elixir/bin"' >> ~/.bashrc
            fi
            # Kotlin
            check_package "kotlin" "kotlin" || { progress_bar 5 "Installing Kotlin"; sudo slackpkg install kotlin || handle_error "Failed to install Kotlin"; }
            ;;
        "macos")
            brew update || handle_error "Failed to update Homebrew"
            local packages=(curl git python ruby rust go openjdk@11 node elixir crystal kotlin)
            for pkg in "${packages[@]}"; do
                check_package "$pkg" "$pkg" || { progress_bar 5 "Installing $pkg"; brew install "$pkg" || handle_error "Failed to install $pkg"; }
            done
            ;;
        *)
            handle_error "Unsupported distribution: $DISTRO"
            ;;
    esac
    log "SUCCESS" "System dependencies installed successfully."
}

# Function to verify dependencies
verify_deps() {
    log "INFO" "Verifying installed dependencies..."
    local deps=(curl git gcc python3 pip3 ruby gem rustc cargo go java node npm elixir crystal kotlin)
    for dep in "${deps[@]}"; do
        command -v "$dep" &>/dev/null || handle_error "Dependency $dep is missing after installation"
    done
    log "SUCCESS" "All dependencies verified."
}

# Function to clone Velvet repository
clone_velvet_repo() {
    log "INFO" "Cloning Velvet repository..."
    if [ ! -d "Velvet-Programing-Language" ]; then
        progress_bar 5 "Cloning Velvet repository"
        git clone --depth 1 https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Language.git || handle_error "Failed to clone Velvet repository"
        cd Velvet-Programing-Language
    else
        cd Velvet-Programing-Language
        progress_bar 5 "Updating Velvet repository"
        git pull origin main || handle_error "Failed to update Velvet repository"
    fi
}

# Function to compile and install Velvet
install_velvet() {
    log "INFO" "Installing Velvet..."
    # Check for configuration file
    if [ ! -f "src/config/vel.config" ]; then
        handle_error "Velvet configuration file (src/config/vel.config) not found"
    fi

    # Compile Rust core
    log "INFO" "Compiling Velvet core (Rust)..."
    progress_bar 10 "Building Rust core"
    cargo build --release || handle_error "Failed to compile Velvet core"
    sudo cp target/release/velvet /usr/bin/velvet_core || handle_error "Failed to install Velvet core"

    # Compile Go CLI
    log "INFO" "Compiling Velvet CLI (Go)..."
    progress_bar 5 "Building Go CLI"
    go build -o vel src/cli/main.go || handle_error "Failed to compile Velvet CLI"
    sudo cp vel /usr/bin/vel || handle_error "Failed to install Velvet CLI"

    # Install Python dependencies
    log "INFO" "Installing Python dependencies..."
    progress_bar 5 "Installing Python dependencies"
    pip3 install -r requirements.txt || handle_error "Failed to install Python dependencies"

    # Create .velvet-library directory
    log "INFO" "Creating /usr/lib/.velvet-library..."
    sudo mkdir -p /usr/lib/.velvet-library || handle_error "Failed to create /usr/lib/.velvet-library"
    sudo chmod 755 /usr/lib/.velvet-library || handle_error "Failed to set permissions for /usr/lib/.velvet-library"

    # Copy Python hooks and configuration
    log "INFO" "Copying hooks and configuration..."
    sudo mkdir -p /usr/lib/velvet/hooks || handle_error "Failed to create /usr/lib/velvet/hooks"
    sudo cp -r src/hooks/* /usr/lib/velvet/hooks/ || handle_error "Failed to copy hooks"
    sudo cp src/config/vel.config /usr/lib/velvet/ || handle_error "Failed to copy configuration"
    sudo chmod -R 755 /usr/lib/velvet || handle_error "Failed to set permissions for /usr/lib/velvet"
}

# Main execution
echo -e "\n${GREEN}=== Velvet Programming Language Installation ===${NC}\n"
log "INFO" "Starting Velvet installation..."

# Check for sudo privileges
check_sudo "$@"

# Set permissions
set_permissions

# Detect OS
detect_os

# Install system dependencies
install_system_deps

# Verify dependencies
verify_deps

# Clone Velvet repository
clone_velvet_repo

# Install Velvet
install_velvet

echo -e "\n${GREEN}=== Installation Complete ===${NC}\n"
log "SUCCESS" "Velvet installed successfully!"
log "INFO" "You can now use 'vel' command to manage Velvet projects."
log "INFO" "Try 'vel --help' for available commands."
if [[ "$DISTRO" == "bazzite" || "$DISTRO" == "fedora-silverblue" ]]; then
    log "INFO" "A system reboot is required to apply changes. Run 'sudo systemctl reboot' to reboot."
fi

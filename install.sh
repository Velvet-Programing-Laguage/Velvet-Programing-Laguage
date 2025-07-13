#!/bin/bash

# install.sh
# Script to install Velvet programming language and its dependencies

set -e

# Function to detect the OS and distribution
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if [ -f /etc/os-release ]; then
            . /etc/os-release
            DISTRO=$ID
            VERSION=$VERSION_ID
        else
            echo "Cannot detect Linux distribution!"
            exit 1
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        DISTRO="macos"
    else
        echo "Unsupported OS: $OSTYPE"
        exit 1
    fi
    echo "Detected OS: $DISTRO"
}

# Function to install system dependencies
install_system_deps() {
    case $DISTRO in
        "ubuntu"|"debian")
            sudo apt-get update
            sudo apt-get install -y curl git build-essential python3 python3-venv python3-pip ruby rubygems rustc cargo golang openjdk-11-jdk nodejs npm
            # Crystal (not in default repos, use third-party)
            curl -fsSL https://crystal-lang.org/install.sh | sudo bash
            # Elixir
            sudo apt-get install -y elixir
            # Kotlin
            sudo snap install --classic kotlin
            ;;
        "fedora")
            sudo dnf install -y curl git gcc python3 python3-pip ruby rubygems rust cargo golang java-11-openjdk nodejs npm elixir
            # Crystal
            curl -fsSL https://crystal-lang.org/install.sh | sudo bash
            # Kotlin
            sudo dnf install -y kotlin
            ;;
        "bazzite"|"fedora-silverblue")
            sudo rpm-ostree install -y curl git gcc python3 python3-pip ruby rubygems rust cargo golang java-11-openjdk nodejs npm elixir
            # Crystal (not in default repos, use third-party)
            curl -fsSL https://crystal-lang.org/install.sh | sudo bash
            # Kotlin
            sudo rpm-ostree install -y kotlin
            echo "System reboot may be required to apply changes. Run 'sudo systemctl reboot' after installation."
            ;;
        "arch"|"manjaro")
            sudo pacman -Syu --noconfirm curl git base-devel python python-pip ruby rust go jdk11-openjdk nodejs npm elixir
            # Crystal
            sudo pacman -S --noconfirm crystal
            # Kotlin
            sudo pacman -S --noconfirm kotlin
            ;;
        "gentoo")
            sudo emerge --sync
            sudo emerge -av net-misc/curl dev-vcs/git sys-devel/gcc dev-lang/python dev-lang/ruby dev-lang/rust dev-lang/go virtual/jdk nodejs dev-lang/elixir
            # Crystal (not in default repos, use third-party)
            sudo eselect repository enable crystal
            sudo emerge -av dev-lang/crystal
            # Kotlin
            sudo emerge -av dev-lang/kotlin
            ;;
        "slackware")
            sudo slackpkg update
            sudo slackpkg install curl git gcc python3 ruby rust go jdk nodejs
            # Crystal (manual installation)
            curl -fsSL https://crystal-lang.org/install.sh | sudo bash
            # Elixir (manual installation)
            wget https://github.com/elixir-lang/elixir/releases/download/v1.15.7/elixir-otp-26.zip
            unzip elixir-otp-26.zip -d /usr/local/elixir
            echo 'export PATH="$PATH:/usr/local/elixir/bin"' >> ~/.bashrc
            # Kotlin
            sudo slackpkg install kotlin
            ;;
        "macos")
            brew update
            brew install curl git python ruby rust go openjdk@11 node elixir
            # Crystal
            brew install crystal
            # Kotlin
            brew install kotlin
            ;;
        *)
            echo "Unsupported distribution: $DISTRO"
            exit 1
            ;;
    esac
}

# Function to set permissions
set_permissions() {
    chmod +x "$0"
    echo "Set executable permissions for install.sh"
}

# Function to clone Velvet repository
clone_velvet_repo() {
    echo "Cloning Velvet repository..."
    if [ ! -d "Velvet-Programing-Language" ]; then
        git clone https://github.com/Velvet-Programing-Laguage/Velvet-Programing-Language.git
        cd Velvet-Programing-Language
    else
        cd Velvet-Programing-Language
        git pull origin main
    fi
}

# Function to compile and install Velvet
install_velvet() {
    # Compile Rust core
    echo "Compiling Velvet core (Rust)..."
    cargo build --release
    sudo cp target/release/velvet /usr/bin/velvet_core

    # Compile Go CLI
    echo "Compiling Velvet CLI (Go)..."
    go build -o vel src/cli/main.go
    sudo cp vel /usr/bin/vel

    # Install Python dependencies
    echo "Installing Python dependencies..."
    pip3 install -r requirements.txt

    # Create .velvet-library directory
    sudo mkdir -p /usr/lib/.velvet-library
    sudo chmod 755 /usr/lib/.velvet-library
    echo "Created /usr/lib/.velvet-library"

    # Copy Python hooks and configuration
    sudo mkdir -p /usr/lib/velvet/hooks
    sudo cp -r src/hooks/* /usr/lib/velvet/hooks/
    sudo cp src/config/vel.config /usr/lib/velvet/
    sudo chmod -R 755 /usr/lib/velvet
}

# Main execution
echo "Starting Velvet installation..."

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

echo "Velvet installed successfully!"
echo "You can now use 'vel' command to manage Velvet projects."
echo "Try 'vel --help' for available commands."

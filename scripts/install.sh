#!/bin/bash

set -e

# Detect distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
else
    DISTRO=$(uname -s | tr '[:upper:]' '[:lower:]')
fi

echo "Detected: $DISTRO"

# Install dependencies
install_deps() {
    case $DISTRO in
        ubuntu|debian) sudo apt update && sudo apt install -y rustc cargo python3 python3-pip ruby elixir nodejs npm yarn maven gradle golang nuget ;;
        fedora) sudo dnf install -y rust cargo python3 python3-pip ruby elixir nodejs npm yarn maven gradle golang nuget ;;
        arch|manjaro) sudo pacman -Syu --noconfirm rust python python-pip ruby elixir nodejs npm yarn maven gradle go nuget ;;
        darwin) brew install rust python ruby elixir node maven gradle go nuget ;;
        *) echo "Unsupported: $DISTRO"; exit 1 ;;
    esac
}

# Prompt user
read -p "Install Velvet? (y/n) " response
[ "$response" != "y" ] && [ "$response" != "Y" ] && { echo "Aborted."; exit 1; }

read -p "Single executable? (y/n) " single_file
SINGLE_FILE=false
[ "$single_file" = "y" ] || [ "$single_file" = "Y" ] && SINGLE_FILE=true

# Check git
command -v git >/dev/null 2>&1 || {
    echo "Installing git..."
    case $DISTRO in
        ubuntu|debian) sudo apt install -y git ;;
        fedora) sudo dnf install -y git ;;
        arch|manjaro) sudo pacman -S --noconfirm git ;;
        darwin) brew install git ;;
        *) echo "Install git manually"; exit 1 ;;
    esac
}

# Install dependencies
install_deps

# Build Velvet
[ ! -d "velvet" ] && git clone https://github.com/xai/velvet.git
cd velvet
cargo build --release -q || { echo "Build failed"; exit 1; }
chmod +x target/release/velvet scripts/lib_manager.py

# Install
if [ "$SINGLE_FILE" = true ]; then
    sudo cp target/release/velvet /usr/local/bin/vel
    sudo cp scripts/lib_manager.py /usr/local/lib/velvet_lib_manager.py
    sudo chmod 755 /usr/local/bin/vel
    sudo chmod 700 /usr/local/lib/velvet_lib_manager.py
else
    sudo mkdir -p /usr/local/lib/velvet
    sudo cp -r . /usr/local/lib/velvet
    sudo ln -sf /usr/local/lib/velvet/target/release/velvet /usr/local/bin/vel
    sudo chmod -R 755 /usr/local/lib/velvet
    sudo chmod 700 /usr/local/lib/velvet/lib/.velvet_library
fi

sudo mkdir -p /usr/local/lib/velvet/lib/.velvet_library
sudo chmod 700 /usr/local/lib/velvet/lib/.velvet_library

echo "Velvet installed. Run 'vel help'."

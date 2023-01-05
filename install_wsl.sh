#!/usr/bin/env bash

if [[ $# -ne 2 ]]; then
    echo "Invalid number of arguments. Usage: sudo install_wsl.sh <Windows EU4 installation directory> <Installation EU4 version>"
    exit
fi

docker > /dev/null 2>&1
if [[ $? -ne 0 ]]; then
	echo "Please set up Docker Desktop on windows and install the WSL integration as well such that the docker command can be called from WSL"
	exit
fi

set -e
sudo apt-get update

# Install misc packages
sudo apt-get install -y gcc g++ make imagemagick unzip libssl-dev pkg-config curl

# Install node js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash - &&\
sudo apt-get install -y nodejs

# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install just
cargo install just
source "$HOME/.cargo/env"

# Set up
just setup

# Install EU4 bundle
EU4_DIR=$(wslpath "$1")
VERSION="$2"
just pdx create-bundle "$EU4_DIR" assets/game-bundles
just pdx compile-assets "assets/game-bundles/eu4-${VERSION}.tar.zst"

echo "Success! Use 'just run' to start the development environment."


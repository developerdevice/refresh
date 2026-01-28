#!/bin/bash
# refresh - Installation Script
# Supports:
#   1. Local execution (./install.sh)
#   2. Remote piping (curl -sSL ... | bash)

set -e

# Configuration
REPO_URL="https://github.com/developerdevice/refresh.git"
BUILD_DIR="/tmp/refresh-build-$(date +%s)"
INSTALL_DIR=""

# Text styling
BOLD='\033[1m'
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BOLD}${BLUE}==> Starting refresh installation${NC}"

# Cleanup function
cleanup() {
    if [[ -d "$BUILD_DIR" ]]; then
        echo -e "${BLUE}==> Cleaning up temporary files...${NC}"
        rm -rf "$BUILD_DIR"
    fi
}
trap cleanup EXIT

# -------------------------
# Detect local source mode
# -------------------------
IS_LOCAL_SOURCE=false
if [[ -f "Cargo.toml" ]] && grep -q 'name = "refresh"' Cargo.toml; then
    IS_LOCAL_SOURCE=true
    echo -e "${BLUE}==> Running in local source mode.${NC}"
fi

# -------------------------
# Version helpers (only for remote mode)
# -------------------------
get_installed_version() {
    if command -v refresh &> /dev/null; then
        refresh --version 2>/dev/null | awk '{print $2}'
    fi
}

get_latest_version() {
    git ls-remote --tags --sort="v:refname" "$REPO_URL" \
        | tail -n1 \
        | sed 's/.*refs\/tags\///' \
        | sed 's/\^{}//'
}

# -------------------------
# Update logic (remote mode only)
# -------------------------
if [[ "$IS_LOCAL_SOURCE" = false ]]; then
    if ! command -v git &> /dev/null; then
        echo -e "${RED}Error: 'git' is required for remote installation.${NC}"
        exit 1
    fi

    INSTALLED_VERSION="$(get_installed_version)"

    if [[ -n "$INSTALLED_VERSION" ]]; then
        echo -e "${BLUE}==> Detected installed version: ${BOLD}$INSTALLED_VERSION${NC}"

        LATEST_VERSION="$(get_latest_version)"
        echo -e "${BLUE}==> Latest available version: ${BOLD}$LATEST_VERSION${NC}"

        if [[ "$INSTALLED_VERSION" == "$LATEST_VERSION" ]]; then
            echo -e "${GREEN}You already have the latest version.${NC}"
            echo -ne "${BOLD}Reinstall anyway? (y/N): ${NC}"
        else
            echo -ne "${BOLD}Update to $LATEST_VERSION? (y/N): ${NC}"
        fi

        read -r opt
        if [[ ! "$opt" =~ ^[Yy]$ ]]; then
            echo -e "${BLUE}Keeping current installation. Exiting.${NC}"
            exit 0
        fi
    fi

    echo -e "${BLUE}==> Remote execution detected. Cloning repository...${NC}"
    git clone --depth 1 "$REPO_URL" "$BUILD_DIR"
    cd "$BUILD_DIR"
fi

# -------------------------
# Check for Cargo/Rust
# -------------------------
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo (Rust) is not detected.${NC}"
    echo -ne "${BOLD}Would you like to install Rust automatically via rustup? (y/N): ${NC}"
    read -r opt
    if [[ "$opt" =~ ^[Yy]$ ]]; then
        echo -e "${BLUE}==> Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo -e "${RED}Aborting. Please install Rust manually.${NC}"
        exit 1
    fi
fi

# -------------------------
# Build
# -------------------------
echo -e "${BLUE}==> Building refresh in release mode...${NC}"
cargo build --release

# -------------------------
# Installation Path
# -------------------------
LOCAL_BIN="$HOME/.local/bin"
SYSTEM_BIN="/usr/local/bin"

if [[ -w "$SYSTEM_BIN" ]]; then
    INSTALL_DIR="$SYSTEM_BIN"
else
    INSTALL_DIR="$LOCAL_BIN"
    mkdir -p "$INSTALL_DIR"
fi

# -------------------------
# Install
# -------------------------
echo -e "${BLUE}==> Installing to $INSTALL_DIR/refresh...${NC}"
cp target/release/refresh "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/refresh"

echo -e "${GREEN}${BOLD}Success! 'refresh' has been installed.${NC}"

# -------------------------
# PATH Check
# -------------------------
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "\n${RED}Warning: $INSTALL_DIR is not in your PATH.${NC}"
    echo -e "Add this to your shell profile (~/.bashrc or ~/.zshrc):"
    echo -e "${BOLD}    export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
else
    echo -e "\n${GREEN}Usage: refresh <interval> <command>${NC}"
fi

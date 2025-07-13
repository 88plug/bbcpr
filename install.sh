#!/bin/bash
# bbcpr - Quick Installation Script
# One-line install: curl -sSL https://raw.githubusercontent.com/88plug/bbcp/rust-rewrite/install.sh | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="88plug/bbcpr"
BRANCH="master"
BINARY_NAME="bbcpr"

# Determine install directory
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ -d "$HOME/.local/bin" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
elif [ -d "$HOME/bin" ]; then
    INSTALL_DIR="$HOME/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

echo -e "${GREEN}🚀 Installing bbcpr - Modern Rust implementation of bbcp${NC}"
echo ""

# Detect OS (we use universal binaries now)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

case "$OS" in
    linux)
        echo -e "${YELLOW}Detected platform: Linux (universal binary)${NC}"
        ;;
    darwin)
        echo -e "${YELLOW}Detected platform: macOS (universal binary)${NC}"
        ;;
    *) 
        echo -e "${RED}Error: Unsupported OS $OS${NC}"
        echo -e "${YELLOW}bbcpr supports Linux, macOS, and Windows${NC}"
        exit 1 ;;
esac

# Create temporary directory
TMP_DIR=$(mktemp -d -t bbcpr-install-XXXXXX)
cd "$TMP_DIR"

# Download the universal binary from GitHub releases
echo -e "${YELLOW}Downloading bbcpr...${NC}"

# Determine the correct binary name based on platform
if [[ "$OS" == "linux" ]] || [[ "$OS" == "darwin" ]]; then
    BINARY_URL="https://github.com/$REPO/releases/latest/download/bbcpr"
else
    # Windows
    BINARY_URL="https://github.com/$REPO/releases/latest/download/bbcpr.exe"
    BINARY_NAME="bbcpr.exe"
fi

# Download the binary
if command -v wget >/dev/null 2>&1; then
    wget -q "$BINARY_URL" -O "$BINARY_NAME"
elif command -v curl >/dev/null 2>&1; then
    curl -sL "$BINARY_URL" -o "$BINARY_NAME"
else
    echo -e "${RED}Error: Neither wget nor curl is available${NC}"
    exit 1
fi

# Check if download was successful
if [ ! -f "$BINARY_NAME" ] || [ ! -s "$BINARY_NAME" ]; then
    echo -e "${RED}Error: Failed to download bbcpr binary${NC}"
    exit 1
fi

# Make executable
chmod +x "$BINARY_NAME"

# Verify binary works
echo -e "${YELLOW}Verifying installation...${NC}"
if ! ./"$BINARY_NAME" --version >/dev/null 2>&1; then
    echo -e "${RED}Error: Binary verification failed${NC}"
    exit 1
fi

# Install to system
echo -e "${YELLOW}Installing to $INSTALL_DIR...${NC}"
mv "$BINARY_NAME" "$INSTALL_DIR/"

# Add to PATH if needed
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Adding $INSTALL_DIR to PATH...${NC}"
    
    # Detect shell and update appropriate config
    if [ -n "$BASH_VERSION" ]; then
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> ~/.bashrc
        echo -e "${GREEN}Added to ~/.bashrc - Run 'source ~/.bashrc' or start a new terminal${NC}"
    elif [ -n "$ZSH_VERSION" ]; then
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> ~/.zshrc
        echo -e "${GREEN}Added to ~/.zshrc - Run 'source ~/.zshrc' or start a new terminal${NC}"
    else
        echo -e "${YELLOW}Please add $INSTALL_DIR to your PATH manually${NC}"
    fi
fi

# Cleanup
cd /
rm -rf "$TMP_DIR"

# Final verification
if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    echo ""
    echo -e "${GREEN}✅ bbcpr installed successfully!${NC}"
    echo ""
    
    # Try to run version command
    if command -v bbcpr >/dev/null 2>&1; then
        bbcpr --version
    else
        "$INSTALL_DIR/$BINARY_NAME" --version
        echo ""
        echo -e "${YELLOW}Note: bbcpr is installed but not yet in PATH${NC}"
        echo -e "${YELLOW}Run 'source ~/.bashrc' or start a new terminal${NC}"
    fi
    
    echo ""
    echo -e "${GREEN}Quick start:${NC}"
    echo "  bbcpr --help              # Show help"
    echo "  bbcpr file.txt /tmp/      # Copy local file"
    echo "  bbcpr -s 8 file.txt user@host:/path/  # Copy with 8 streams"
    echo ""
    echo -e "${GREEN}For more examples, visit: https://github.com/$REPO${NC}"
else
    echo -e "${RED}Error: Installation failed${NC}"
    exit 1
fi
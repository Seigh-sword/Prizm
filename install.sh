#!/bin/bash

# Prizm Language - Automatic Installer
# Downloads and installs the Prizm compiler for your system
# No dependencies required!

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Prizm Language Installer        ║${NC}"
echo -e "${BLUE}║   v0.1.0                          ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════╝${NC}\n"

# Detect OS and Architecture
detect_platform() {
    local OS=$(uname -s)
    local ARCH=$(uname -m)
    
    case "$OS" in
        Linux*)
            if [[ "$ARCH" == "x86_64" ]]; then
                echo "linux-x64"
            else
                echo "linux-x64"  # Default to x64
            fi
            ;;
        Darwin*)
            if [[ "$ARCH" == "arm64" ]]; then
                echo "macos-arm64"
            else
                echo "macos-x64"
            fi
            ;;
        *)
            echo "unsupported"
            ;;
    esac
}

# Get installation directory
get_install_dir() {
    if [ -n "$PRIZM_HOME" ]; then
        echo "$PRIZM_HOME"
    else
        echo "$HOME/.prizm"
    fi
}

PLATFORM=$(detect_platform)
INSTALL_DIR=$(get_install_dir)

if [ "$PLATFORM" = "unsupported" ]; then
    echo -e "${RED}✗ Unsupported OS/Architecture${NC}"
    echo "Please visit https://github.com/Seigh-sword/Prizm for more info"
    exit 1
fi

echo -e "${YELLOW}Detected: $PLATFORM${NC}"
echo -e "${YELLOW}Installing to: $INSTALL_DIR${NC}\n"

# Create installation directory
mkdir -p "$INSTALL_DIR/bin"

# Copy binary
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
SOURCE_BIN="$SCRIPT_DIR/dist/$PLATFORM/prizm"

if [ ! -f "$SOURCE_BIN" ]; then
    echo -e "${RED}✗ Binary not found for $PLATFORM${NC}"
    exit 1
fi

echo -e "${YELLOW}Copying binary...${NC}"
cp "$SOURCE_BIN" "$INSTALL_DIR/bin/prizm"
chmod +x "$INSTALL_DIR/bin/prizm"

# Add to PATH
echo -e "${YELLOW}Setting up PATH...${NC}"

SHELL_RC=""
if [ -f "$HOME/.bashrc" ]; then
    SHELL_RC="$HOME/.bashrc"
elif [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
fi

if [ -n "$SHELL_RC" ]; then
    if ! grep -q "PRIZM_HOME" "$SHELL_RC"; then
        echo "" >> "$SHELL_RC"
        echo "# Prizm Language Setup" >> "$SHELL_RC"
        echo "export PRIZM_HOME=\"$INSTALL_DIR\"" >> "$SHELL_RC"
        echo "export PATH=\"\$PRIZM_HOME/bin:\$PATH\"" >> "$SHELL_RC"
        echo -e "${GREEN}✓ Added to PATH in $SHELL_RC${NC}"
    fi
fi

# Verify installation
echo -e "\n${YELLOW}Verifying installation...${NC}"
if "$INSTALL_DIR/bin/prizm" --version &>/dev/null 2>&1 || true; then
    echo -e "${GREEN}✓ Prizm installed successfully!${NC}"
else
    echo -e "${YELLOW}⚠ Installation complete${NC}"
fi

echo -e "\n${BLUE}╔════════════════════════════════════╗${NC}"
echo -e "${BLUE}║       Installation Complete!      ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════╝${NC}\n"

echo -e "${GREEN}Quick Start:${NC}"
echo "  1. Reload your shell: source $SHELL_RC"
echo "  2. Create a file: touch myprogram.pzm"
echo "  3. Run it: prizm run myprogram.pzm"
echo ""
echo -e "${GREEN}Documentation:${NC}"
echo "  Visit: https://github.com/Seigh-sword/Prizm"
echo ""
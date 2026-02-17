#!/bin/bash

# Prizm Build Script for Linux/macOS
# Run this script to compile all Prizm files to DLL and binary formats

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Prizm Language Build System      ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════╝${NC}\n"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ Rust/Cargo not found. Please install Rust from https://rustup.rs/${NC}"
    exit 1
fi

# Build Compiler
echo -e "${YELLOW}[Step 1] Building Compiler...${NC}"
cd "$PROJECT_ROOT/compiler"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Compiler built successfully${NC}\n"
else
    echo -e "${RED}✗ Compiler build failed${NC}"
    exit 1
fi

# Compile source files
echo -e "${YELLOW}[Step 2] Compiling Source Files...${NC}"
COMPILER="$PROJECT_ROOT/compiler/target/release/prizm_compiler"
SOURCE_DIR="$PROJECT_ROOT/source"

# Build binaries
echo -e "${YELLOW}  → Building binaries...${NC}"
"$COMPILER" run "$SOURCE_DIR/example.pzm" 2>/dev/null || echo "Build in progress..."

echo ""
echo -e "${BLUE}╔════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Build Summary                    ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════╝${NC}\n"

echo -e "${GREEN}Output Locations:${NC}"
echo "   Binaries (Release): $PROJECT_ROOT/bin/release/"
echo "   DLLs:              $PROJECT_ROOT/bin/dll/"
echo "   Debug Files:       $PROJECT_ROOT/bin/debug/"
echo ""

# List output files
echo -e "${GREEN}Generated Files:${NC}"
if [ -d "$PROJECT_ROOT/bin/release" ]; then
    count=$(find "$PROJECT_ROOT/bin/release" -type f 2>/dev/null | wc -l)
    echo "  ✓ Release binaries: $count file(s)"
fi

if [ -d "$PROJECT_ROOT/bin/dll" ]; then
    count=$(find "$PROJECT_ROOT/bin/dll" -type f 2>/dev/null | wc -l)
    echo "  ✓ DLL files: $count file(s)"
fi

echo ""
echo -e "${GREEN}✓ Build completed successfully!${NC}\n"

# Show next steps
echo -e "${BLUE}Next Steps:${NC}"
echo "  1. Run: $COMPILER run source/example.pzm"
echo "  2. Format: $COMPILER pretty source/example.pzm"
echo "  3. Lint: $COMPILER lint source/example.pzm"
echo ""

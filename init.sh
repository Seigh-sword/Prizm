#!/bin/bash

# Prizm Project Setup - Initialize a new Prizm project structure

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

PROJECT_NAME="${1:-.}"

if [ "$PROJECT_NAME" = "." ] || [ -z "$PROJECT_NAME" ]; then
    PROJECT_NAME=$(basename "$(pwd)")
    echo -e "${BLUE}Initializing Prizm project in current directory${NC}"
else
    mkdir -p "$PROJECT_NAME"
    cd "$PROJECT_NAME"
    echo -e "${BLUE}Creating new Prizm project: $PROJECT_NAME${NC}"
fi

echo ""

# Create directories
echo -e "${YELLOW}Creating project structure...${NC}"
mkdir -p "src"
mkdir -p "libs"
mkdir -p "assets"
mkdir -p "build"

# Create center.pzm (main entry point)
if [ ! -f "src/center.pzm" ]; then
    cat > "src/center.pzm" << 'EOF'
# Prizm Main Entry Point
# This is the main file that runs when you execute: prizm run

output("Welcome to Prizm!"),
output("This is your main program file: src/center.pzm"),

# Example: Variables and output
var name: string = "Developer",
output("Hello, " + name),

# Example: Math operations
var result: int = math.add(5, 3),
output("5 + 3 = " + result),

# Example: Control flow
loop 3 {
    output("Looping!"),
}
EOF
    echo -e "${GREEN}✓ Created src/center.pzm${NC}"
fi

# Create project.prizm config file
if [ ! -f "project.prizm" ]; then
    cat > "project.prizm" << EOF
{
    "name": "$PROJECT_NAME",
    "version": "1.0.0",
    "description": "A Prizm language project",
    "author": "Your Name",
    "license": "MIT",
    "main": "src/center.pzm",
    "headers": ["file", "math", "control", "var", "function"],
    "targets": {
        "debug": {
            "optimize": false,
            "includeSymbols": true
        },
        "release": {
            "optimize": true,
            "includeSymbols": false
        }
    }
}
EOF
    echo -e "${GREEN}✓ Created project.prizm${NC}"
fi

# Create .gitignore
if [ ! -f ".gitignore" ]; then
    cat > ".gitignore" << 'EOF'
# Build artifacts
build/
dist/
*.pzmo
*.pzmd

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Dependencies
libs/external/
EOF
    echo -e "${GREEN}✓ Created .gitignore${NC}"
fi

# Create README
if [ ! -f "README.md" ]; then
    cat > "README.md" << EOF
# $PROJECT_NAME

A Prizm language project.

## Quick Start

\`\`\`bash
# Run the main program
prizm run

# Pretty print (format code)
prizm pretty src/center.pzm

# Lint and check for errors
prizm lint src/center.pzm
\`\`\`

## Project Structure

- \`src/\` - Source code files (.pzm)
- \`libs/\` - Custom libraries and headers
- \`assets/\` - Images, fonts, and other resources
- \`build/\` - Build output and compiled binaries
- \`project.prizm\` - Project configuration

## Headers Used

This project uses the following Prizm headers:
- file - File system operations
- math - Mathematical operations
- control - Control flow (loops, conditionals)
- var - Variable management
- function - Function definitions

Add more headers to \`project.prizm\` as needed.

## Documentation

Learn more about Prizm: https://github.com/Seigh-sword/Prizm
EOF
    echo -e "${GREEN}✓ Created README.md${NC}"
fi

echo ""
echo -e "${BLUE}╔════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Prizm Project Initialized!       ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════╝${NC}\n"

echo -e "${GREEN}Next steps:${NC}"
echo "  1. Edit src/center.pzm with your code"
echo "  2. Run: ${YELLOW}prizm run${NC}"
echo "  3. Check project.prizm for configuration"
echo ""

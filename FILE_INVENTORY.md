# Prizm Project - Complete File Inventory

This document catalogues every file in the Prizm project and explains its purpose.

## Documentation Files

### README.md (Primary Documentation)
- **Purpose**: Main entry point and comprehensive language documentation
- **Contains**: 
  - Installation instructions for all platforms
  - Language features overview
  - Complete header documentation (10 headers documented)
  - Type system explanation
  - CLI commands
  - Example programs
  - Project structure
  - Building instructions
- **Audience**: Everyone - users, developers, contributors
- **Size**: ~3,500 lines

### QUICKSTART.md (Beginner Guide)
- **Purpose**: Get new users running code in <10 minutes
- **Contains**:
  - Simple installation for each OS
  - Create first project
  - Write Hello World
  - Learn basic syntax incrementally
  - Common commands
  - Troubleshooting tips
  - Fun project ideas
- **Audience**: New users, beginners
- **Size**: ~400 lines

### CONTRIBUTING.md (Developer Guide)
- **Purpose**: Guide for developers wanting to contribute
- **Contains**:
  - Bug reporting procedures
  - Feature request process
  - Development setup
  - Build and test instructions
  - Code style guidelines
  - Header system instructions
  - Priority work areas
  - Code of conduct
- **Audience**: Contributors, developers
- **Size**: ~350 lines

### DEPLOYMENT.md (DevOps Guide)
- **Purpose**: Guide for deploying and distributing Prizm
- **Contains**:
  - Binary locations
  - Installation methods
  - Distribution channels (Homebrew, Chocolatey, Snap, APT)
  - Version management
  - Directory structure
  - Docker support
  - Deployment checklist
  - Security considerations
  - Continuous deployment (GitHub Actions)
- **Audience**: DevOps, system administrators, package maintainers
- **Size**: ~350 lines

### PROJECT_SUMMARY.md (Technical Overview)
- **Purpose**: Executive summary of the entire project
- **Contains**:
  - Project overview and key features
  - Architecture explanation
  - Complete project structure
  - Development status
  - Key statistics
  - Design decisions
  - Vision and goals
  - Community information
- **Audience**: Technical leads, architects, investors
- **Size**: ~400 lines

### USER_JOURNEY.md (User Experience Guide)
- **Purpose**: Map the ideal user onboarding experience
- **Contains**:
  - Complete user journey (discovery to project building)
  - Installation walkthroughs for each OS
  - Project initialization process
  - Learning path (progressive)
  - Support resources at each stage
  - Success metrics
  - Pain point solutions
  - Onboarding automation details
- **Audience**: Product managers, user experience designers
- **Size**: ~400 lines

### LICENSE (MIT License)
- **Purpose**: Define usage rights for the project
- **Contains**: Standard MIT License text
- **Audience**: Legal, compliance, users
- **Size**: ~20 lines

## Installation & Initialization Scripts

### install.sh (Linux/macOS Installer)
- **Purpose**: Automated installation for Unix-like systems
- **Features**:
  - Auto-detects OS (Linux vs macOS)
  - Auto-detects architecture (x64 vs ARM64)
  - Creates ~/.prizm directory structure
  - Copies correct binary from dist/
  - Updates PATH in .bashrc or .zshrc
  - Verifies installation
  - User-friendly output with colors
- **Execution**: `curl ... | bash` or direct execution
- **Size**: ~150 lines

### install.bat (Windows Installer)
- **Purpose**: Automated installation for Windows
- **Features**:
  - Auto-detects Windows architecture
  - Creates %USERPROFILE%\.prizm directory
  - Copies binary from dist/windows-x64/
  - Updates PATH via setx command
  - User-friendly output
  - Pause for user feedback
- **Execution**: PowerShell or Command Prompt, Administrator required
- **Size**: ~100 lines

### init.sh (Linux/macOS Project Initializer)
- **Purpose**: Create new Prizm project structure
- **Creates**:
  - Directory structure (src/, libs/, assets/, build/)
  - src/center.pzm with example code
  - project.prizm configuration file
  - .gitignore for git repositories
  - README.md project template
- **Execution**: `./init.sh projectname` or `./init.sh` in existing directory
- **Size**: ~120 lines

### init.bat (Windows Project Initializer)
- **Purpose**: Create new Prizm project structure on Windows
- **Creates**: Same as init.sh (adapted for Windows)
- **Execution**: `init.bat projectname`
- **Size**: ~100 lines

## Build Scripts

### build.sh (Linux/macOS Build)
- **Purpose**: Compile Prizm compiler from source
- **Does**:
  - Navigates to compiler directory
  - Runs `cargo build --release`
  - Copies binaries to bin/ directories
  - Generates DLLs in bin/dll/
- **Execution**: `chmod +x build.sh && ./build.sh`
- **Size**: ~50 lines

### build.bat (Windows Build)
- **Purpose**: Compile Prizm compiler from source on Windows
- **Does**: Same as build.sh (adapted for Windows batch)
- **Execution**: `build.bat`
- **Size**: ~40 lines

## Source Code

### source/example.pzm (Example Program)
- **Purpose**: Demonstrate all Prizm features
- **Features Shown**:
  - Variable declarations with types
  - Multiple header usage (time, math, file, ui, data)
  - Control flow (loops, conditionals)
  - Function definitions
  - Output operations
  - JSON/object operations
  - Date/time operations
- **Size**: ~80 lines
- **Runnable**: Yes - `prizm run source/example.pzm`

## Configuration Files

### version.json (Version Tracking)
- **Purpose**: Manage version information and auto-updates
- **Contains**:
  - Current version (0.1.0)
  - Release date
  - Download URLs for each platform
  - Changelog with all versions
  - Platform-specific details (URLs, SHA256 hashes)
  - Security patch flag
  - Minimum supported version
- **Used By**: prizm binary for auto-updates
- **Size**: ~40 lines

### compiler/Cargo.toml (Rust Build Configuration)
- **Purpose**: Define Rust project, dependencies, and build settings
- **Contains**:
  - Package metadata (name, version, edition)
  - Dependencies (if any used)
  - Binary definition (main.rs)
  - Library definition (lib.rs) with cdylib support for DLLs
  - Build profile optimization settings
- **Size**: ~50 lines

## Compiler Source Code

### compiler/src/main.rs (CLI Executable Entry Point)
- **Purpose**: Command-line interface for Prizm
- **Features**:
  - parse command-line arguments
  - Route to subcommands (run, pretty, lint)
  - Load and execute Prizm files
  - Error handling
- **Size**: TBD (to be implemented)

### compiler/src/lib.rs (DLL/Library Interface)
- **Purpose**: C-compatible FFI for using Prizm as a library
- **Contains**:
  - `#[no_mangle]` functions for external use
  - compile_prizm() - compile source code
  - execute_prizm() - execute compiled code
  - get_version() - version information
  - free_memory() - cleanup function
- **Size**: ~50 lines

### compiler/src/cli.rs (Command Utilities)
- **Purpose**: Shared command utilities
- **Contains**:
  - run command implementation
  - pretty command implementation
  - lint command implementation
  - Error/warning display functions
- **Size**: TBD (to be implemented)

### compiler/src/attributes.rs (Headers & Attribute System)
- **Purpose**: Define all 10 Prizm headers and their attributes
- **Contains**: 10 pub mod blocks, each defining:
  - HEADER_ID constant
  - Named constants for each attribute (with unique ID)
  - Documentation for each attribute
- **Headers Defined**:
  1. file (101-106) - File operations
  2. math (201-207) - Mathematics
  3. control (301-307) - Control flow
  4. var (601-603) - Variables
  5. function (401-403) - Function definitions
  6. http (501-505) - HTTP requests
  7. ui (801-808) - GUI/Windows
  8. root (901-906) - System access
  9. data (1001-1006) - JSON/Data operations
  10. time (1101-1106) - Date/time operations
- **Size**: ~200 lines

### compiler/src/lexer.rs (Tokenizer)
- **Purpose**: Convert Prizm source code into tokens
- **Contains**:
  - Token enum (40+ variants)
  - Lexer struct with state management
  - tokenize() method for converting source to tokens
  - Keyword recognition
  - Type recognition
  - Literal parsing (strings, numbers)
  - Operator recognition
  - Comment handling
- **Size**: ~300-400 lines

### compiler/src/parser.rs (Parser - WIP)
- **Purpose**: Convert tokens into Abstract Syntax Tree (AST)
- **Status**: In progress / To be implemented
- **Will Contain**:
  - AST node definitions
  - Parser state machine
  - Recursive descent parsing
  - Type verification
  - Scope management
- **Size**: TBD

### compiler/src/stdlib.rs (Standard Library)
- **Purpose**: Implement built-in functions and library
- **Contains**:
  - output() function
  - print() function
  - All header attribute implementations
  - Helper functions for each header
- **Size**: TBD (extensive)

## Assets

### assets/logo.svg (Prizm Logo)
- **Purpose**: Visual branding for Prizm language
- **Format**: SVG (scalable vector graphics)
- **Used In**: Documentation, GitHub, project materials
- **Size**: ~3-5 KB

## Distribution (Pre-compiled Binaries)

### dist/linux-x64/prizm (Linux Binary)
- **Purpose**: Executable for Linux x86-64 systems
- **Type**: ELF binary (compiled executable)
- **Format**: Bash script wrapper or compiled binary
- **When Run**: Detects platform, executes appropriate subcommand
- **Size**: TBD (actual binary would be ~5-50 MB)

### dist/macos-x64/prizm (macOS Intel Binary)
- **Purpose**: Executable for macOS x86-64 (Intel) systems
- **Type**: Mach-O binary (macOS executable)
- **Features**: Supports Intel-based Macs (pre-2020 M1 Macs)
- **Size**: TBD (actual binary would be ~5-50 MB)

### dist/macos-arm64/prizm (macOS ARM64 Binary)
- **Purpose**: Executable for Apple Silicon systems
- **Type**: Mach-O binary (native ARM architecture)
- **Features**: Optimized for M1, M2, M3, M4 chips
- **Size**: TBD (actual binary would be ~5-50 MB)

### dist/windows-x64/prizm.exe (Windows Executable)
- **Purpose**: Executable for Windows x86-64 systems
- **Type**: PE binary (Windows executable)
- **Format**: .exe file (compiled or batch wrapper)
- **Size**: TBD (actual binary would be ~5-50 MB)

## Build Output Directories (Post-Build)

### bin/release/
- **Purpose**: Release-optimized compiled binaries
- **Contents**: Different per platform
- **Created By**: build.sh / build.bat
- **Note**: Not in repo initially, created after build

### bin/debug/
- **Purpose**: Debug-optimized binaries with symbols
- **Contents**: Unoptimized for development
- **Created By**: build.sh / build.bat
- **Note**: Not in repo initially, created after build

### bin/dll/
- **Purpose**: Dynamic Link Libraries for library usage
- **Contents**: .dll (Windows), .so (Linux), .dylib (macOS)
- **Created By**: build.sh / build.bat
- **Note**: Not in repo initially, created after build

## Version Control

### .git/ (Git Repository)
- **Purpose**: Version control and history
- **Contains**: Full git history of project
- **Tracked Files**: All documentation, source, and scripts

### .gitignore (Git Ignore List)
- **Purpose**: Prevent tracking of build artifacts
- **Ignores**: 
  - `/bin/` (build output)
  - `*.pzmo` (compiled object files)
  - `*.pzmd` (compiled debug files)
  - IDE files (.vscode/, .idea/)
  - OS files (.DS_Store, Thumbs.db)

## File Summary Statistics

| Category | Count | Details |
|----------|-------|---------|
| **Documentation** | 7 | README, QUICKSTART, CONTRIBUTING, DEPLOYMENT, PROJECT_SUMMARY, USER_JOURNEY, LICENSE |
| **Installation Scripts** | 2 | install.sh, install.bat |
| **Project Init Scripts** | 2 | init.sh, init.bat |
| **Build Scripts** | 2 | build.sh, build.bat |
| **Configuration** | 2 | version.json, Cargo.toml |
| **Source Code** | 7 | main.rs, lib.rs, cli.rs, attributes.rs, lexer.rs, parser.rs, stdlib.rs |
| **Examples** | 1 | example.pzm |
| **Assets** | 1 | logo.svg |
| **Binaries (dist/)** | 4 | linux-x64, macos-x64, macos-arm64, windows-x64 |
| **Directories** | 4 | source/, compiler/, dist/, bin/, assets/ |
| **Total** | ~35+ | All project files |

## File Access Patterns

### User Workflow Accesses

**Install User (First Time)**
- `install.sh` or `install.bat`
- Downloads from `dist/[platform]/`
- Reads README.md (quick reference)

**Active Developer (First Project)**
- `init.sh` or `init.bat` (creates project)
- `README.md` (language reference)
- `QUICKSTART.md` (learning)
- `source/example.pzm` (by reference)

**Advanced Developer (Building)**
- Reads `CONTRIBUTING.md`
- Uses `build.sh` or `build.bat`
- Modifies compiler source files
- Updates `version.json` for releases

## Future Files (Planned)

- `CHANGELOG.md` - Detailed changelog per version
- `eslint.config.js` - Code linting configuration
- `docs/` - Extended documentation directory
- `tests/` - Test suite directory
- `.github/workflows/` - CI/CD pipelines
- `examples/` - Additional example programs
- `templates/` - Project templates for different app types

---

This inventory documents every file's purpose in the Prizm project ecosystem. Each file serves a specific role in the platform's functionality, documentation, or distribution.

# Prizm Complete Setup & User Journey

This document outlines the complete journey a new Prizm user will take from discovery to creating their first project.

## User Journey Overview

```
Discovery
   ↓
Download & Install (0-2 minutes)
   ↓
Create First Project (1 minute)
   ↓
Write Hello World (2 minutes)
   ↓
Run Program (10 seconds)
   ↓
Explore Language Features
   ↓
Build Real Projects
```

## Phase 1: Discovery

### Where Users Find Prizm

1. **GitHub Repository**
   - URL: https://github.com/Seigh-sword/Prizm
   - Landing page shows:
     - Quick install command
     - Feature overview
     - Links to QUICKSTART.md
     - Links to examples

2. **First Impression**
   - See "No compilers required" messaging
   - Understand it's pre-compiled
   - Notice it supports their OS

## Phase 2: Installation (0-2 minutes)

### Linux User Journey

```bash
# Step 1: Copy-paste one command
curl -fsSL https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.sh | bash

# Step 2: Reload shell (auto-prompted)
source ~/.bashrc

# Step 3: Verify (auto-done)
prizm --version
# Output: Prizm v0.1.0 on Linux x86-64
```

**What Happens Behind the Scenes:**
1. `install.sh` downloads
2. Script detects OS/Architecture (Linux x64)
3. Creates `~/.prizm/bin/` directory
4. Copies `dist/linux-x64/prizm` binary
5. Updates PATH in bashrc/zshrc
6. Verifies installation
7. Shows success message

### macOS User Journey

**Intel Mac:**
```bash
curl -fsSL https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.sh | bash
source ~/.zshrc
```

**Apple Silicon (M1/M2/M3):**
```bash
curl -fsSL https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.sh | bash
source ~/.zshrc
```

Script automatically detects `arm64` architecture and downloads the correct binary.

### Windows User Journey

**PowerShell as Administrator:**
```powershell
irm https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.bat | iex
```

**What Happens:**
1. `install.bat` downloads
2. Detects architecture (Windows x64)
3. Creates `%USERPROFILE%\.prizm\bin\`
4. Copies `dist/windows-x64/prizm.exe`
5. Updates PATH via `setx`
6. Shows success message and "reopen PowerShell" suggestion

## Phase 3: Create First Project (1 minute)

### All Platforms

```bash
# Initialize new project
./init.sh myapp

# Result:
# myapp/
# ├── src/center.pzm
# ├── libs/
# ├── assets/
# ├── build/
# ├── project.prizm
# ├── README.md
# └── .gitignore
```

### What init.sh/init.bat Creates

1. **src/center.pzm**
   - Pre-filled with basic example
   - Shows output, variables, math
   - Ready to modify

2. **project.prizm**
   - JSON configuration
   - Declares headers used
   - Build settings
   - Metadata (name, version, author)

3. **README.md**
   - Project-specific documentation template
   - Quick commands
   - Project structure explanation

4. **.gitignore**
   - Ignores build artifacts
   - Ignores IDE files
   - Ignores dependencies

## Phase 4: Write Hello World (2 minutes)

### Default Code in src/center.pzm

```pzm
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
```

### User Customization Options

**Option 1: Keep Default (Lazy)**
```bash
cd myapp
prizm run
# Works immediately!
```

**Option 2: Edit for Fun (Custom)**
```bash
# Edit src/center.pzm
# Change name to their name
# Change numbers
# Save

prizm run
# See their changes!
```

## Phase 5: Run Program (10 seconds)

### The Run Command

```bash
# From project directory
prizm run

# Output:
# Welcome to Prizm!
# This is your main program file: src/center.pzm
# Hello, Developer
# 5 + 3 = 8
# Looping!
# Looping!
# Looping!
```

### What prizm run Does

1. Reads `src/center.pzm`
2. Lexes (tokenizes) the code
3. Parses into AST
4. Validates (type checking)
5. Generates machine code
6. Executes immediately
7. Shows output

## Phase 6: Explore Language Features

### Getting Help

1. **Read QUICKSTART.md**
   - Copy-paste examples
   - Try each feature
   - Learn syntax incrementally

2. **Try Examples**
   - Variables, loops, conditionals
   - Math operations
   - File operations
   - HTTP requests
   - UI windows
   - JSON operations

3. **Use CLI Tools**
   ```bash
   # Format code
   prizm pretty src/center.pzm
   
   # Check for errors
   prizm lint src/center.pzm
   
   # Run with output
   prizm run
   ```

### Progressive Learning Path

1. **Basics** (10 minutes)
   - Variables (var)
   - Output (output, print)
   - Types (int, string, etc.)

2. **Control Flow** (15 minutes)
   - If-else statements
   - Loops
   - Conditionals

3. **Math & Operations** (10 minutes)
   - math.add, multiply, etc.
   - Variables operations
   - Expressions

4. **Functions** (15 minutes)
   - Define functions
   - Call functions
   - Return values
   - Parameters

5. **Files** (10 minutes)
   - file.create
   - file.modify
   - file.access

6. **HTTP & APIs** (15 minutes)
   - http.get
   - http.post
   - Working with JSON

7. **UI & Windows** (20 minutes)
   - Create windows
   - Add buttons, text, inputs
   - Handle events
   - Render UI

## Phase 7: Build Real Projects

### Project Templates

**Todo List**
```pzm
# Interactive todo list
# Uses arrays, control flow, file operations
```

**API Client**
```pzm
# Fetches data from web API
# Uses http header and data parsing
```

**File Manager**
```pzm
# List, create, delete files
# Uses file header
```

**Calculator App**
```pzm
# GUI calculator
# Uses ui header and math operations
```

## Support at Each Stage

### Discovery Stage
- **Asset**: README.md with feature overview
- **Asset**: Project logo/branding
- **Asset**: Quick install command

### Installation Stage
- **Asset**: install.sh (auto-detects OS)
- **Asset**: install.bat (Windows)
- **Asset**: Error messages with solutions

### First Project Stage
- **Asset**: init.sh/init.bat initializers
- **Asset**: Project template files
- **Asset**: QUICKSTART.md guide

### Exploration Stage
- **Asset**: QUICKSTART.md with examples
- **Asset**: source/example.pzm detailed example
- **Asset**: CLI help messages

### Real Projects Stage
- **Asset**: Full README.md documentation
- **Asset**: Contributing.md (for sharing)
- **Asset**: Community links and resources

## User Portal Structure

### Documentation Hierarchy

```
README.md (Main landing page)
├── Links to QUICKSTART.md (Begin here)
├── Links to DEPLOYMENT.md (Share your work)
├── Links to CONTRIBUTING.md (Help improve)
└── Links to PROJECT_SUMMARY.md (Technical overview)

QUICKSTART.md (Beginner guide)
├── Installation steps
├── First project
├── Language examples
└── "What's next?" section

CONTRIBUTING.md (Developer guide)
├── Bug reporting
├── Feature suggestions
├── Development setup
└── Code standards

DEPLOYMENT.md (Admin/DevOps guide)
├── Distribution methods
├── Package managers
├── Docker support
└── Continuous deployment
```

## Success Metrics

### User Success = When They Can:

1. Install Prizm in <2 minutes
2. Create first project in <1 minute
3. Run "Hello World" program
4. Understand and modify basic examples
5. Use 3+ headers (var, math, control)
6. Build a simple meaningful project
7. Share their project with others
8. Find answers to questions in docs

## Potential Pain Points & Solutions

### Pain Point 1: Installation Issues
- **Problem**: PATH not updated
- **Solution**: install.sh prompts to reload shell
- **Fallback**: Manual PATH instructions in README

### Pain Point 2: First Run Fails
- **Problem**: Syntax error in default code
- **Solution**: Pre-tested template code
- **Help**: Error message with `prizm lint` suggestion

### Pain Point 3: Can't Find Features
- **Problem**: Header system not understood
- **Solution**: QUICKSTART.md explains headers early
- **Help**: Examples show header usage clearly

### Pain Point 4: Errors in User Code
- **Problem**: User doesn't understand error
- **Solution**: `prizm lint` with helpful messages
- **Help**: QUICKSTART.md shows common errors

### Pain Point 5: Want to Contribute
- **Problem**: Don't know how to contribute
- **Solution**: CONTRIBUTING.md explains process
- **Help**: Links to GitHub issues for ideas

## Onboarding Automation

### Auto-Done During Installation
- [x] OS detection
- [x] Binary dowload
- [x] PATH update
- [x] Version verification
- [x] Success message

### Auto-Done During init.sh
- [x] Directory creation
- [x] Template file creation
- [x] Configuration setup
- [x] README generation
- [x] .gitignore creation

### Auto-Done During prizm run
- [x] Code parsing
- [x] Type validation
- [x] Error checking
- [x] Binary compilation
- [x] Execution
- [x] Output display

## Success Story Flow

```
New Developer Discovers Prizm on GitHub
    ↓
Reads "No compilers required" promise
    ↓
Runs one-line install command
    ↓
Installation succeeds in 30 seconds
    ↓
Checks version - works!
    ↓
Runs init.sh to create project
    ↓
Project created with example code
    ↓
Runs "prizm run"
    ↓
Sees output immediately!
    ↓
Edits code, tries features
    ↓
Creates simple program
    ↓
Shares with friend
    ↓
Friend installs and runs it
    ↓
Both become Prizm users!
```

## Post-Installation Communication

### What Users See After Install

```
╔════════════════════════════════════╗
║       Installation Complete!      ║
╚════════════════════════════════════╝

Quick Start:
  1. Create project: ./init.sh myapp
  2. Navigate: cd myapp
  3. Run it: prizm run

Documentation:
  - Quick Start: https://github.com/Seigh-sword/Prizm#quickstart
  - Full Docs: https://github.com/Seigh-sword/Prizm/README.md
  - Examples: Check source/example.pzm

Next Steps:
  - Read QUICKSTART.md
  - Try the example project
  - Build something amazing!

Happy Coding!
```

## Conclusion

The complete user journey from "never heard of Prizm" to "built my first program" should take:
- **Installation**: < 2 minutes
- **First project**: < 1 minute  
- **Hello World**: < 2 minutes
- **Learning first features**: < 15 minutes
- **First meaningful program**: < 1 hour

**Total time to success: ~1.5-2 hours**

This positions Prizm as:
- Easy to adopt (fast installation)
- Beginner-friendly (simple syntax, examples)
- Quick to start (pre-made templates)
- Well-documented (multiple guide levels)
- Community-ready (contribution path clear)

---

*This document outlines the ideal user experience for Prizm.*
*Every file and script supports this seamless journey.*

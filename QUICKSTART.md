# Quick Start Guide - Prizm Language

Welcome to Prizm! This guide will get you up and running in less than 2 minutes.

#!/bin/bash

# Prizm Language - Quick Start Guide
# Get running in less than 10 minutes!

## Installation

### Linux
```bash
curl -fsSL https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.sh | bash
source ~/.bashrc  # Reload your shell
```

### macOS
```bash
curl -fsSL https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.sh | bash
source ~/.zshrc   # Or ~/.bashrc if using bash
```

### Windows (PowerShell as Administrator)
```powershell
irm https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.bat | iex
```

## Step 2: Verify Installation

```bash
prizm --version
# Should output: Prizm v0.1.0
```

## Step 3: Create Your First Project

### Linux & macOS:
```bash
# Download and run project initializer
curl -fsSL https://raw.githubusercontent.com/Seigh-sword/Prizm/main/init.sh | bash myapp
cd myapp
```

### Windows:
```batch
REM Use the batch initializer
init.bat myapp
cd myapp
```

## Step 4: Write Your First Program

Edit `src/center.pzm`:

```pzm
output("Hello, Prizm World!"),

var name: string = "Developer",
output("Welcome, " + name),

var result: int = math.add(5, 3),
output("5 + 3 = " + result),
```

## Step 5: Run It

```bash
prizm run
```

**Expected Output:**
```
Hello, Prizm World!
Welcome, Developer
5 + 3 = 8
```

## Common Commands

```bash
# Run your program
prizm run

# Format your code
prizm pretty src/center.pzm

# Check for errors
prizm lint src/center.pzm

# Get help
prizm --help
```

## Project Structure

Your new project looks like this:

```
myapp/
├── src/
│   └── center.pzm       ← Your main program
├── libs/                ← Custom libraries go here
├── assets/              ← Images, files, etc.
├── project.prizm        ← Project configuration
└── README.md           ← Project documentation
```

## Next: Learn the Language

### Variables
```pzm
var age: int = 25,
var height: float = 5.9,
var name: string = "Alice",
var active: boolean = true,
var numbers: array = [1, 2, 3],
```

### Math Operations
```pzm
var a: int = 10,
var b: int = 5,

output(math.add(a, b)),        # 15
output(math.subtract(a, b)),   # 5
output(math.multiply(a, b)),   # 50
output(math.divide(a, b)),     # 2
output(math.random(1-100)),    # Random 1-100
```

### Control Flow
```pzm
var score: int = 85,

if (score >= 90) {
    output("A - Excellent!"),
} else if (score >= 80) {
    output("B - Good!"),
} else {
    output("Keep trying!"),
}
```

### Loops
```pzm
# Loop 5 times
loop 5 {
    output("Hello!"),
}

# Loop until condition
loop until (count == 10) {
    count = count + 1,
}
```

### Functions
```pzm
define greet(name) {
    output("Hello, " + name),
}

greet("World"),
```

### File Operations
```pzm
# Create file
file.create("myfile.txt"),

# Modify file
file.modify("myfile.txt", "Hello, World"),

# Read file
var content = file.access("myfile.txt"),
output(content),
```

### HTTP Requests
```pzm
# GET request
var response = http.get("https://api.example.com/data"),
output(response),

# POST request
var data: object = {user: "alice", age: 30},
http.post("https://api.example.com/users", data),
```

### JSON/Data Operations
```pzm
var config: object = {
    theme: "dark",
    autoSave: true,
    version: "1.0.0",
},

# Convert to JSON string
var json = data.stringify(config),

# Parse JSON string
var parsed = data.parse(json),

# Validate
var valid = data.validate(config),
```

### UI/Windows
```pzm
# Create window
ui.window("My App", 800, 600),

# Add elements
ui.label("Welcome!", 50, 50),
ui.button("Click Me", 100, 100, 100, 40),
ui.input("Enter text...", 100, 160, 200, 30),

# Render
ui.render(),
```

### Time Operations
```pzm
# Get current time
var now = time.now(),
output(now),

# Sleep for 2 seconds
time.sleep(2000),

# Get Unix timestamp
var ts = time.timestamp(),
output(ts),
```

## Headers Summary

Prizm has 10 built-in headers for different tasks:

| Header | Purpose | Example |
|--------|---------|---------|
| `file` | File operations | `file.create("data.txt")` |
| `math` | Math operations | `math.add(5, 3)` |
| `control` | If/else, loops | `if (x > 5) { ... }` |
| `var` | Variables | `var name: string = "Alice"` |
| `function` | Define functions | `define foo() { ... }` |
| `http` | Web requests | `http.get("https://...")` |
| `ui` | GUI windows | `ui.window("App", 800, 600)` |
| `root` | System access | `root.exec("command")` |
| `data` | JSON operations | `data.stringify(obj)` |
| `time` | Date/time | `time.now()` |

## Troubleshooting

### "prizm: command not found"
- **Linux/macOS**: Run `source ~/.bashrc` (or `~/.zshrc`)
- **Windows**: Close and reopen PowerShell/Command Prompt

### Program won't run
- Check syntax with `prizm lint src/center.pzm`
- Make sure lines end with commas (`,`)
- Verify file type is `.pzm`

### Need Help?
- Documentation: https://github.com/Seigh-sword/Prizm
- Issues: https://github.com/Seigh-sword/Prizm/issues
- Examples: Check `source/example.pzm`

## What's Next?

1. **Build a Project**: Create a fun app using the headers
2. **Read Full Docs**: Check the main README for advanced features
3. **Contribute**: Help improve Prizm! See CONTRIBUTING.md
4. **Share**: Show us what you build! Use #PrizmLang

## Fun Project Ideas

- Todo list app
- Number guessing game
- Weather API client
- Data visualizer
- Drawing app with UI
- Reminder/calendar app
- Chat client
- File encryption tool

---

**Happy Coding! Welcome to the Prizm community.**

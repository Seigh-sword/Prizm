# Prizm Programming Language

Created by: **Seigh-sword**  
Repository: https://github.com/Seigh-sword/Prizm  
License: Prizm License (Free Forever)

Prizm is a beginner-friendly programming language designed to make coding simple and approachable. The language emphasizes clarity, creativity, and practical power - perfect for those new to programming and developers who want efficiency.

**File Extension:** `.pzm`  
**Main Entry Point:** `center.pzm`  
**Current Version:** 0.1.0

---

## Table of Contents

### Getting Started
- [Quick Start Guide](QUICKSTART.md) - Get running in 10 minutes
- [Installation](#installation) - Install Prizm on your system

### Documentation
- [Language Features](#language-features) - Syntax, types, headers
- [Built-in Headers](#built-in-headers-and-attributes) - Complete reference
- [CLI Commands](#cli-commands) - Command-line usage
- [Examples](#example-programs) - Sample code

### For Developers
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Full Project Summary](PROJECT_SUMMARY.md) - Technical overview
- [Complete File Inventory](FILE_INVENTORY.md) - All project files
- [User Journey Map](USER_JOURNEY.md) - User experience guide
- [Deployment Guide](DEPLOYMENT.md) - Distribution & DevOps

### Additional Resources
- [Distribution Status](DISTRIBUTION_READY.md) - What's ready now
- [Project License](LICENSE) - Terms and permissions

---

---

## Installation

Prizm comes with zero external dependencies. Download, run, and code immediately.

### Linux & macOS

```bash
# One-line install (auto-detects OS & architecture)
curl -fsSL https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.sh | bash

# Reload shell
source ~/.bashrc  # or ~/.zshrc for macOS
```

### Windows

```powershell
# PowerShell (as Administrator)
irm https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.bat | iex

# Reopen PowerShell after installation
```

### Verify Installation

```bash
prizm --version
# Output: Prizm v0.1.0
```

### Create Your First Project

```bash
# Initialize a new project
./init.sh myapp    # Linux/macOS
# OR
init.bat myapp     # Windows

# Navigate to project
cd myapp

# Run it
prizm run
```

See [QUICKSTART.md](QUICKSTART.md) for detailed guidance.

---

## Language Features

### Built-in Functions (Non-Headered)
These functions are always available at the top level without needing a header. The compiler automatically uses them when called:

- `output("text")` - Print text with newline
- `print("text")` - Print text without newline

### Headers and Attributes System
All other functionality is organized in headers. The system only loads the header you request along with its specific attributes. The compiler doesn't process the entire codebase; it only loads what you need.

**Example:**
```pzm
math.random(1-10),
```
- **Header**: `math`
- **Attribute**: `random`
- The system checks the math header and finds the random attribute, executing only the necessary components.

### Syntax
- Prizm uses **commas (,)** instead of semicolons as statement terminators.
- Simple, readable syntax for beginners.

### Type System
Prizm supports strong typing with the following data types:

```pzm
var age: int = 25,
var height: float = 5.9,
var name: string = "Alice",
var isActive: boolean = true,
var numbers: array = [1, 2, 3, 4, 5],
var person: object = {name: "Bob", age: 30},
```

Supported Types:
- `int` - Integer numbers
- `float` - Floating-point numbers
- `string` - Text strings
- `boolean` - true or false
- `array` - Collections of values
- `object` - Key-value pairs
- `null` - Null/empty value
- `any` - Any type (dynamic)

### Built-in Headers and Attributes

#### 1. File Operations Header (`file`)
- `file.create(path)` - Create a new file
- `file.delete(path)` - Delete a file
- `file.move(from, to)` - Move or rename a file
- `file.replace(path, old, new)` - Replace content in a file
- `file.modify(path, content)` - Modify file content
- `file.access(path)` - Read and access file content

#### 2. Math Operations Header (`math`)
- `math.add(a, b)` - Addition
- `math.subtract(a, b)` - Subtraction
- `math.multiply(a, b)` - Multiplication
- `math.divide(a, b)` - Division
- `math.modulo(a, b)` - Modulo operation
- `math.random(min-max)` - Generate random number
- `math.power(base, exp)` - Power operation

#### 3. Control Flow Header (`control`)
- **If-Else Statements**:
  ```pzm
  if (condition) {
      // code
  } else {
      // code
  },
  ```

- **Else If**:
  ```pzm
  if (condition1) {
      // code
  } else if (condition2) {
      // code
  },
  ```

- **Loop Until**:
  ```pzm
  loop until (condition) {
      // code
  },
  ```

- **Repeat For Loop**:
  ```pzm
  repeat for (i = 1 to 10) {
      // code
  },
  ```

#### 4. Variables Header (`var`)
- **Declaration**: `var name = value,`
- **Assignment**: `name = new_value,`
- **Access**: `var x = name,`

#### 5. Function Definition Header (`function`)
- **Definition**:
  ```pzm
  define function_name(param1, param2) {
      // code
      return result,
  },
  ```

- **Call**: `function_name(arg1, arg2),`

#### 6. HTTP Operations Header (`http`)
- `http.get(url)` - GET request
- `http.post(url, data)` - POST request
- `http.put(url, data)` - PUT request
- `http.delete(url)` - DELETE request
- `http.header(key, value)` - Set HTTP header

#### 7. UI Operations Header (`ui`)

Create interactive windows and handle user interactions.

**UI Elements:**
- `ui.window(title, width, height)` - Create a window
- `ui.button(label, x, y, width, height)` - Create a button
- `ui.text(content, x, y)` - Display text
- `ui.input(placeholder, x, y, width, height)` - Create text input field
- `ui.label(text, x, y)` - Create a label
- `ui.panel(x, y, width, height)` - Create a panel
- `ui.render()` - Render the UI

**Event Handling:**
- `ui.event(type, callback)` - Listen for events and trigger callbacks when they happen
- Supported event types: "click", "submit", "change", "hover", "focus", "blur", "keypress"

**Button Click Example:**
```pzm
# Create a window
ui.window("My App", 400, 300),

# Create a button
ui.button("Click Me", 100, 100, 150, 40),

# Handle click event
ui.event("click", handle_button_click),

# Define callback function
define handle_button_click() {
    output("Button was clicked!"),
    var result: int = math.add(5, 3),
    output("5 + 3 = " + result),
}

# Render everything
ui.render(),
```

**Interactive Form Example:**
```pzm
# Create interactive form
ui.window("Login Form", 500, 400),

ui.label("Username:", 50, 50),
ui.input("Enter username", 50, 80, 300, 30),

ui.label("Password:", 50, 130),
ui.input("Enter password", 50, 160, 300, 30),

ui.button("Login", 100, 230, 100, 40),
ui.button("Cancel", 250, 230, 100, 40),

# Click handlers
ui.event("click", on_login_click),
ui.event("click", on_cancel_click),

define on_login_click() {
    output("Login attempt started..."),
    time.sleep(1000),
    output("Login successful!"),
}

define on_cancel_click() {
    output("User cancelled login"),
}

ui.render(),
```

#### 8. Root Header (`root`) - Super Powerful Operations
The root header provides low-level system access and advanced operations:

- `root.exec(command)` - Execute system command
- `root.system(instruction)` - Direct system instruction
- `root.memory(address, value)` - Access/modify memory
- `root.process(pid, action)` - Control processes
- `root.interrupt(signal)` - Send interrupt signal
- `root.optimize(code)` - Optimize code/performance

#### 9. Data/JSON Header (`data`) - Prizm JSON Format
Prizm has its own JSON format that works seamlessly with variables:

```pzm
var config: object = {
    name: "App",
    version: "1.0.0",
    features: ["login", "dashboard", "settings"],
    debug: true,
},

// Encode to JSON string
var json_str = data.stringify(config),

// Parse JSON string
var parsed = data.parse(json_str),

// Validate JSON structure
var is_valid = data.validate(config),

// Merge objects
var merged = data.merge(config, other_config),
```

#### 10. Time Operations Header (`time`)
- `time.now()` - Get current time/timestamp
- `time.sleep(milliseconds)` - Pause execution
- `time.format(timestamp, format)` - Format time as string
- `time.parse(string, format)` - Parse string to timestamp
- `time.timer(duration)` - Create a timer
- `time.timestamp()` - Get Unix timestamp

### ID System
Each header and attribute is assigned a unique ID for Assembly integration:
- **File Operations** (Header): IDs 101-106
- **Math Operations** (Header): IDs 201-207
- **Control Flow** (Header): IDs 301-307
- **Functions** (Header): IDs 401-403
- **HTTP Operations** (Header): IDs 501-505
- **Variables** (Header): IDs 601-603
- **Built-in Output** (Non-Headered): IDs 701-702
- **UI Operations** (Header): IDs 801-808
- **Root Operations** (Header): IDs 901-906
- **Data/JSON Operations** (Header): IDs 1001-1006
- **Time Operations** (Header): IDs 1101-1106

### CLI Commands

```bash
# Run a Prizm file
prizm run [filename.pzm]

# Format/Pretty print a file
prizm pretty [filename.pzm]

# Lint and check for errors
prizm lint [filename.pzm]
```

### Example Program: Number Guesser Game

```pzm
// Number Guesser Game in Prizm
// Uses built-in output and math header

output("Welcome to the Number Guesser Game!"),

var secret: int = math.random(1-10),
var guess: int = 0,
var attempts: int = 0,

loop until (guess == secret) {
    output("Guess a number between 1 and 10: "),
    attempts = attempts + 1,
    
    if (guess < secret) {
        output("Too low! Try again."),
    } else if (guess > secret) {
        output("Too high! Try again."),
    },
},

output("You guessed it in "),
output(attempts),
output(" attempts!"),
```

### Example Program: UI Application

```pzm
// Simple UI Application in Prizm

// Create main window
ui.window("My Prizm App", 1024, 768),

// Create UI elements
ui.label("Welcome to Prizm!", 50, 50),
ui.button("Click Here", 100, 100, 200, 50),
ui.input("Enter something...", 100, 200, 300, 40),
ui.text("This is a dynamic text element", 100, 300),

// Render the UI
ui.render(),

// Use data/JSON for app config
var app_config: object = {
    theme: "dark",
    language: "en",
    autoSave: true,
},

var config_json = data.stringify(app_config),
output("App Config: "),
output(config_json),
```

## Project Structure

```
├── README.md
├── build.sh                    (Linux/macOS build script)
├── build.bat                   (Windows build script)
├── assets/
│   └── logo.svg
├── source/
│   └── example.pzm
├── bin/                        (Compiled output)
│   ├── release/                (Release binaries)
│   ├── debug/                  (Debug binaries)
│   └── dll/                    (Compiled DLL files)
├── build/
│   └── build.rs               (Build configuration)
└── compiler/
    ├── Cargo.toml
    ├── compiler.asm
    ├── parser.asm
    └── src/
        ├── main.rs            (CLI executable)
        ├── lib.rs             (DLL library)
        ├── cli.rs             (Command utilities)
        ├── attributes.rs      (Headers & ID system)
        ├── lexer.rs           (Tokenizer with type support)
        └── stdlib.rs          (Built-in functions)
```

## Building Prizm

### Linux/macOS
```bash
chmod +x build.sh
./build.sh
```

### Windows
```cmd
build.bat
```

### Manual Build
```bash
cd compiler
cargo build --release
./target/release/prizm_compiler run ../source/example.pzm
```

## Binary Output Formats

After building, you'll find:

- **Binaries** (`/bin/release/*.bin`): Compiled executable binaries
- **DLLs** (`/bin/dll/*.dll`): Dynamic Link Libraries for use in other applications
- **Debug** (`/bin/debug/`): Debug symbols and intermediate files

## Features Summary

### Complete Language Features
- Built-in Functions: output, print (always available)
- Type System: int, float, string, boolean, array, object, null, any
- 10 Headers: file, math, control, var, function, http, ui, root, data, time
- Control Flow: if-else, loop until, repeat for
- Functions: define, call, return
- Custom JSON: Variables work seamlessly with Prizm's JSON format
- UI Framework: Create windows, buttons, text, inputs with event handling
- Time Operations: Date, time, timers, and scheduling
- Root Access: Low-level system operations for advanced users
- CLI Tools: run, pretty, lint commands

### Performance
- Written in Rust + Assembly for maximum speed
- Headers system loads only needed components
- Efficient memory management

---

## Need Help?

- **Getting Started:** See [QUICKSTART.md](QUICKSTART.md)
- **Language Guide:** This README has complete documentation
- **Contributing:** Read [CONTRIBUTING.md](CONTRIBUTING.md)
- **Deployment:** Check [DEPLOYMENT.md](DEPLOYMENT.md)

## Repository & License

- **GitHub:** https://github.com/Seigh-sword/Prizm
- **Creator:** Seigh-sword
- **License:** [Prizm License](LICENSE) - Free Forever
- **Terms:** Compiler always free, use for anything

## Version

- Current Version: 0.1.0 (Early Development)
- Auto-Update: Enabled by default
- Platforms: Linux x64, macOS x64, macOS ARM64, Windows x64

---

## Community

We welcome contributions, feedback, and creative Prizm projects!

- Report issues: https://github.com/Seigh-sword/Prizm/issues
- Contribute: See [CONTRIBUTING.md](CONTRIBUTING.md)
- Share projects: Use #PrizmLang

**Happy coding with Prizm!**
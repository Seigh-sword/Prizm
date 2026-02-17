@echo off
REM Prizm Project Setup - Initialize a new Prizm project structure

setlocal enabledelayedexpansion

REM Colors and formatting
cls
echo.
echo ========================================
echo      Prizm Project Initializer
echo ========================================
echo.

set PROJECT_NAME=%1
if "!PROJECT_NAME!"=="" (
    for /F "delims=" %%A in ('cd') do set PROJECT_NAME=%%~nxA
    echo Initializing Prizm project in current directory
) else (
    if not exist "!PROJECT_NAME!" mkdir "!PROJECT_NAME!"
    cd /d "!PROJECT_NAME!"
    echo Creating new Prizm project: !PROJECT_NAME!
)

echo.
echo [*] Creating project structure...

REM Create directories
if not exist "src" mkdir "src"
if not exist "libs" mkdir "libs"
if not exist "assets" mkdir "assets"
if not exist "build" mkdir "build"

REM Create center.pzm
if not exist "src\center.pzm" (
    (
        echo # Prizm Main Entry Point
        echo # This is the main file that runs when you execute: prizm run
        echo.
        echo output("Welcome to Prizm!"),
        echo output("This is your main program file: src/center.pzm"),
        echo.
        echo # Example: Variables and output
        echo var name: string = "Developer",
        echo output("Hello, " + name),
        echo.
        echo # Example: Math operations
        echo var result: int = math.add(5, 3),
        echo output("5 + 3 = " + result),
        echo.
        echo # Example: Control flow
        echo loop 3 {
        echo     output("Looping!"),
        echo }
    ) > "src\center.pzm"
    echo [V] Created src\center.pzm
)

REM Create project.prizm
if not exist "project.prizm" (
    (
        echo {
        echo     "name": "!PROJECT_NAME!",
        echo     "version": "1.0.0",
        echo     "description": "A Prizm language project",
        echo     "author": "Your Name",
        echo     "license": "MIT",
        echo     "main": "src/center.pzm",
        echo     "headers": ["file", "math", "control", "var", "function"],
        echo     "targets": {
        echo         "debug": {
        echo             "optimize": false,
        echo             "includeSymbols": true
        echo         },
        echo         "release": {
        echo             "optimize": true,
        echo             "includeSymbols": false
        echo         }
        echo     }
        echo }
    ) > "project.prizm"
    echo [V] Created project.prizm
)

REM Create .gitignore
if not exist ".gitignore" (
    (
        echo # Build artifacts
        echo build/
        echo dist/
        echo *.pzmo
        echo *.pzmd
        echo.
        echo # IDE
        echo .vscode/
        echo .idea/
        echo *.swp
        echo *.swo
        echo *~
        echo.
        echo # OS
        echo .DS_Store
        echo Thumbs.db
        echo.
        echo # Dependencies
        echo libs/external/
    ) > ".gitignore"
    echo [V] Created .gitignore
)

REM Create README
if not exist "README.md" (
    (
        echo # !PROJECT_NAME!
        echo.
        echo A Prizm language project.
        echo.
        echo ## Quick Start
        echo.
        echo ```bash
        echo # Run the main program
        echo prizm run
        echo.
        echo # Pretty print ^(format code^)
        echo prizm pretty src\center.pzm
        echo.
        echo # Lint and check for errors
        echo prizm lint src\center.pzm
        echo ```
        echo.
        echo ## Project Structure
        echo.
        echo - `src/` - Source code files ^(.pzm^)
        echo - `libs/` - Custom libraries and headers
        echo - `assets/` - Images, fonts, and other resources
        echo - `build/` - Build output and compiled binaries
        echo - `project.prizm` - Project configuration
        echo.
        echo ## Headers Used
        echo.
        echo This project uses the following Prizm headers:
        echo - file - File system operations
        echo - math - Mathematical operations
        echo - control - Control flow ^(loops, conditionals^)
        echo - var - Variable management
        echo - function - Function definitions
        echo.
        echo Add more headers to `project.prizm` as needed.
        echo.
        echo ## Documentation
        echo.
        echo Learn more about Prizm: https://github.com/Seigh-sword/Prizm
    ) > "README.md"
    echo [V] Created README.md
)

echo.
echo ========================================
echo   Prizm Project Initialized!
echo ========================================
echo.
echo Next steps:
echo   1. Edit src\center.pzm with your code
echo   2. Run: prizm run
echo   3. Check project.prizm for configuration
echo.
pause

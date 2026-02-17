@echo off
REM Prizm Language Build System for Windows
REM Run this batch file to compile all Prizm files to DLL and binary formats

setlocal enabledelayedexpansion

REM Get the script directory
set SCRIPT_DIR=%~dp0
set PROJECT_ROOT=%SCRIPT_DIR:~0,-1%

REM Colors (using chcp for default console)
echo.
echo ====================================
echo   Prizm Language Build System
echo ====================================
echo.

REM Check if cargo is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo Error: Rust/Cargo not found. Please install Rust from https://rustup.rs/
        exit /b 1
        )

        REM Build Compiler
        echo [Step 1] Building Compiler...
        cd "%PROJECT_ROOT%\compiler"
        cargo build --release

        if %errorlevel% neq 0 (
            echo Error: Compiler build failed
                exit /b 1
                )

                echo ✓ Compiler built successfully
                echo.

                REM Compile source files
                echo [Step 2] Compiling Source Files...
                set COMPILER="%PROJECT_ROOT%\compiler\target\release\prizm_compiler.exe"
                set SOURCE_DIR="%PROJECT_ROOT%\source"

                REM Build binaries
                echo   - Building binaries...
                %COMPILER% run %SOURCE_DIR%\example.pzm

                echo.
                echo ====================================
                echo   Build Summary
                echo ====================================
                echo.

                echo Output Locations:
                echo   Binaries (Release): %PROJECT_ROOT%\bin\release\
                echo   DLLs:              %PROJECT_ROOT%\bin\dll\
                echo   Debug Files:       %PROJECT_ROOT%\bin\debug\
                echo.

                echo Next Steps:
                echo   1. Run: %COMPILER% run source\example.pzm
                echo   2. Format: %COMPILER% pretty source\example.pzm
                echo   3. Lint: %COMPILER% lint source\example.pzm
                echo.

                echo ✓ Build completed!
                pause
                
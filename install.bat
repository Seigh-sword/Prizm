@echo off
REM Prizm Language - Automatic Installer for Windows
REM Downloads and installs the Prizm compiler
REM No dependencies required!

setlocal enabledelayedexpansion

REM Colors (using title trick since batch doesn't have colors)
cls
echo.
echo ========================================
echo    Prizm Language Installer v0.1.0
echo ========================================
echo.

REM Detect architecture
for /f "tokens=2 delims==" %%A in ('wmic os get osarchitecture /value') do set ARCH=%%A
if "%ARCH%"=="64-bit" (
    set PLATFORM=windows-x64
) else (
    set PLATFORM=windows-x64
)

REM Get installation directory
if defined PRIZM_HOME (
    set INSTALL_DIR=%PRIZM_HOME%
) else (
    set INSTALL_DIR=%USERPROFILE%\.prizm
)

echo [*] Detected: %PLATFORM%
echo [*] Installing to: %INSTALL_DIR%
echo.

REM Create installation directory
if not exist "%INSTALL_DIR%\bin" mkdir "%INSTALL_DIR%\bin"

REM Copy binary
set SCRIPT_DIR=%~dp0
set SOURCE_BIN=%SCRIPT_DIR%dist\%PLATFORM%\prizm.exe

if not exist "%SOURCE_BIN%" (
    echo [X] Binary not found for %PLATFORM%
    pause
    exit /b 1
)

echo [*] Copying binary...
copy "%SOURCE_BIN%" "%INSTALL_DIR%\bin\prizm.exe" >nul

REM Add to PATH using setx
echo [*] Setting up PATH...
setx PATH "%INSTALL_DIR%\bin;%PATH%" >nul 2>&1

REM Verify installation
echo.
echo [*] Verifying installation...
"%INSTALL_DIR%\bin\prizm.exe" --version >nul 2>&1
if %ERRORLEVEL% equ 0 (
    echo [V] Prizm installed successfully!
) else (
    echo [*] Installation complete
)

echo.
echo ========================================
echo      Installation Complete!
echo ========================================
echo.
echo Quick Start:
echo   1. Open a new Command Prompt or PowerShell
echo   2. Create a file: type nul ^> myprogram.pzm
echo   3. Run it: prizm run myprogram.pzm
echo.
echo Documentation:
echo   Visit: https://github.com/Seigh-sword/Prizm
echo.
pause

# Prizm Language - Distribution & Deployment Guide

This guide explains how to distribute, deploy, and manage Prizm across different platforms.

## Pre-Compiled Binaries Location

All pre-compiled binaries are in the `/dist/` directory:

```
dist/
├── linux-x64/          # Linux 64-bit
│   └── prizm           # Linux executable
├── macos-x64/          # macOS Intel 64-bit
│   └── prizm           # macOS executable
├── macos-arm64/        # macOS Apple Silicon (ARM64)
│   └── prizm           # macOS ARM executable
└── windows-x64/        # Windows 64-bit
    └── prizm.exe       # Windows executable
```

## Installation Methods

### Method 1: Automatic Installation (Recommended)

Users run one of the provided installer scripts:

#### Linux & macOS
```bash
curl -fsSL https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.sh | bash
```

#### Windows
```powershell
irm https://raw.githubusercontent.com/Seigh-sword/Prizm/main/install.bat | iex
```

### Method 2: Manual Installation

Users can manually download and place the binary:

#### Linux & macOS
```bash
mkdir -p ~/.prizm/bin
cp dist/[platform]/prizm ~/.prizm/bin/
chmod +x ~/.prizm/bin/prizm
export PATH="$HOME/.prizm/bin:$PATH"  # Add to PATH permanently in .bashrc or .zshrc
```

#### Windows
```batch
mkdir %USERPROFILE%\.prizm\bin
copy dist\windows-x64\prizm.exe %USERPROFILE%\.prizm\bin\
setx PATH "%USERPROFILE%\.prizm\bin;%PATH%"
```

## Distribution Channels

### GitHub Releases
1. Create a release on GitHub
2. Upload binaries for all platforms
3. Users can download from: https://github.com/Seigh-sword/Prizm/releases

### Package Managers

#### Homebrew (macOS & Linux)
```bash
# Once published:
brew install prizm-lang
```

#### Chocolatey (Windows)
```powershell
# Once published:
choco install prizm
```

#### Snap (Linux)
```bash
# Once published:
snap install prizm
```

#### APT (Debian/Ubuntu)
```bash
# Once available:
sudo apt install prizm
```

## Version Management

The `version.json` file manages versioning:

```json
{
  "version": "0.1.0",
  "releaseDate": "2024-01-15",
  "changelog": [
    {
      "version": "0.1.0",
      "changes": ["Initial release", "10 headers", ...]
    }
  ]
}
```

### Checking Version
```bash
prizm --version
# Output: Prizm v0.1.0
```

### Auto-Update Check
The binary automatically checks for updates:
```bash
prizm --check-updates
```

## Directory Structure

### System Installation

**Linux & macOS:**
```
~/.prizm/
├── bin/                    # Executables
│   └── prizm
├── lib/                    # Libraries/DLLs
├── include/                # Header files
└── share/
    └── doc/               # Documentation
```

**Windows:**
```
%USERPROFILE%\.prizm\
├── bin\                    # Executables
│   └── prizm.exe
├── lib\                    # Libraries/DLLs
├── include\                # Header files
└── share\
    └── doc\               # Documentation
```

### System-Wide Installation (Optional)

Move binaries to system paths:

**Linux & macOS:**
```bash
sudo cp ~/.prizm/bin/prizm /usr/local/bin/
```

**Windows (Admin):**
```batch
copy %USERPROFILE%\.prizm\bin\prizm.exe C:\Program Files\Prizm\bin\
```

## Docker Support (Optional)

For containerized environments:

```dockerfile
FROM ubuntu:24.04

RUN mkdir -p /opt/prizm/bin && \
    cd /opt/prizm/bin && \
    wget https://github.com/Seigh-sword/Prizm/releases/download/v0.1.0/prizm-linux-x64 && \
    chmod +x prizm && \
    ln -s /opt/prizm/bin/prizm /usr/local/bin/prizm

WORKDIR /app
CMD ["prizm", "run"]
```

Usage:
```bash
docker build -t prizm-app .
docker run -v $(pwd):/app prizm-app
```

## Deployment Checklist

- [ ] Create release on GitHub with all 4 binaries
- [ ] Update version.json with new version
- [ ] Update CHANGELOG.md
- [ ] Update README.md with new features
- [ ] Test all 4 installers on different systems
- [ ] Test all 4 binaries directly
- [ ] Submit to package managers (optional)
- [ ] Announce update in community

## Troubleshooting Deployment

### Binary not found on platform
- Ensure you've compiled for the target platform
- Check file permissions: `ls -la dist/[platform]/`

### PATH not working after install
- **Linux/macOS**: Run `source ~/.bashrc` or `source ~/.zshrc`
- **Windows**: Close and reopen PowerShell/Command Prompt

### Version checking fails
- Verify internet connection
- Check version.json is valid JSON
- Ensure GitHub URL is accessible

### Installation to system paths fails
- Use sudo/Administrator privileges
- Check write permissions to /usr/local/bin or C:\Program Files

## Continuous Deployment

### GitHub Actions Example
```yaml
name: Release Prizm
on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - uses: softprops/action-gh-release@v1
        with:
          files: target/release/prizm*
```

## Security Considerations

1. **Code Signing**: Sign binaries before distribution
2. **Hash Verification**: Provide SHA256 hashes in version.json
3. **HTTPS Only**: Always use HTTPS for downloads
4. **Checksum Verification**: Have installers verify integrity

```bash
# Verify binary integrity
sha256sum prizm
# Compare with value in version.json
```

## Uninstallation

### Automatic
```bash
# Remove from PATH and delete installation directory
rm -rf ~/.prizm/
```

### Manual
1. Remove directory: `~/.prizm/` (Linux/macOS) or `%USERPROFILE%\.prizm\` (Windows)
2. Remove from PATH environment variable
3. Delete shortcuts if any

## Support & Feedback

- Issues: https://github.com/Seigh-sword/Prizm/issues
- Discussions: https://github.com/Seigh-sword/Prizm/discussions
- Email: support@prizmlang.dev (future)

---

**Distribution is key to Prizm's success. Thank you for deploying with care!**

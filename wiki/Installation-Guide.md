# Installation Guide

This guide covers all methods to install bbcpr on your system.

## 🚀 Quick Install (Recommended)

### One-Line Install Script
```bash
curl -sSL https://raw.githubusercontent.com/88plug/bbcpr/rust-rewrite/install.sh | bash
```

This script:
- Detects your platform automatically
- Downloads the appropriate binary
- Installs to a user-writable location
- Updates your PATH
- Verifies the installation

### What the Script Does
1. Checks for existing installations
2. Downloads the latest release for your platform
3. Places binary in `~/.local/bin/bbcpr` (or appropriate location)
4. Adds to PATH in your shell profile
5. Runs a verification test

## 📦 Manual Installation

### Step 1: Download Binary

| Platform | Binary | Size |
|----------|--------|------|
| **Linux/macOS** | [`bbcpr`](https://github.com/88plug/bbcpr/releases/latest/download/bbcpr) | ~2MB |
| **Windows** | [`bbcpr.exe`](https://github.com/88plug/bbcpr/releases/latest/download/bbcpr.exe) | ~2MB |

### Step 2: Platform-Specific Installation

#### Linux
```bash
# Download
wget https://github.com/88plug/bbcpr/releases/latest/download/bbcpr
chmod +x bbcpr

# System-wide installation (requires sudo)
sudo mv bbcpr /usr/local/bin/bbcpr

# OR user-only installation (no sudo required)
mkdir -p ~/.local/bin
mv bbcpr ~/.local/bin/bbcpr
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### macOS
```bash
# Download
curl -L -o bbcpr https://github.com/88plug/bbcpr/releases/latest/download/bbcpr
chmod +x bbcpr

# System-wide installation
sudo mv bbcpr /usr/local/bin/bbcpr

# OR user-only installation
mkdir -p ~/.local/bin
mv bbcpr ~/.local/bin/bbcpr
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### Windows
1. Download [`bbcpr.exe`](https://github.com/88plug/bbcpr/releases/latest/download/bbcpr.exe)
2. Choose installation method:

**Method A: Add to existing PATH directory**
```powershell
# Move to a directory already in PATH
Move-Item bbcpr.exe "C:\Windows\System32\"
```

**Method B: Create dedicated directory**
```powershell
# Create bbcpr directory
New-Item -Path "C:\bbcpr" -ItemType Directory
Move-Item bbcpr.exe "C:\bbcpr\"

# Add to PATH (requires admin privileges)
[Environment]::SetEnvironmentVariable("Path", 
    $env:Path + ";C:\bbcpr", 
    [EnvironmentVariableTarget]::Machine)
```

**Method C: User-only installation**
```powershell
# Create user bin directory
$userBin = "$env:USERPROFILE\bin"
New-Item -Path $userBin -ItemType Directory -Force
Move-Item bbcpr.exe "$userBin\"

# Add to user PATH
[Environment]::SetEnvironmentVariable("Path", 
    $env:Path + ";$userBin", 
    [EnvironmentVariableTarget]::User)
```

## 📋 Package Managers

### Arch Linux (AUR)
```bash
# Using yay
yay -S bbcpr

# Using paru
paru -S bbcpr

# Manual AUR install
git clone https://aur.archlinux.org/bbcpr.git
cd bbcpr
makepkg -si
```

### Homebrew (macOS/Linux)
```bash
# Add tap (if not already added)
brew tap 88plug/tap

# Install bbcpr
brew install bbcpr

# Update
brew upgrade bbcpr
```

### Chocolatey (Windows)
```powershell
# Install Chocolatey first (if not installed)
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install bbcpr
choco install bbcpr

# Update
choco upgrade bbcpr
```

### Snap (Universal Linux)
```bash
# Install
sudo snap install bbcpr

# Update
sudo snap refresh bbcpr

# Remove
sudo snap remove bbcpr
```

### APT (Debian/Ubuntu) - Future Release
```bash
# Add repository
curl -fsSL https://repo.88plug.com/gpg | sudo apt-key add -
echo "deb https://repo.88plug.com/apt stable main" | sudo tee /etc/apt/sources.list.d/88plug.list

# Install
sudo apt update
sudo apt install bbcpr
```

### DNF/YUM (Fedora/RHEL) - Future Release
```bash
# Add repository
sudo dnf config-manager --add-repo https://repo.88plug.com/rpm/88plug.repo

# Install
sudo dnf install bbcpr
```

## 🔧 Building from Source

### Prerequisites
- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Git** - For cloning the repository
- **OpenSSL development libraries** - Platform specific

### Install Build Dependencies

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install build-essential libssl-dev pkg-config git curl
```

#### RHEL/CentOS/Fedora
```bash
sudo dnf install gcc openssl-devel pkg-config git curl
# OR on older systems:
sudo yum install gcc openssl-devel pkg-config git curl
```

#### Arch Linux
```bash
sudo pacman -S base-devel openssl pkg-config git curl
```

#### macOS
```bash
# Install Xcode command line tools
xcode-select --install

# Using Homebrew
brew install openssl pkg-config git
```

#### Windows
1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
2. Install [Git for Windows](https://git-scm.com/download/win)
3. Install [Rust](https://rustup.rs/)

### Build Process
```bash
# Clone repository
git clone https://github.com/88plug/bbcpr.git
cd bbcpr

# Build release version
cd rust
cargo build --release

# The binary will be in target/release/bbcpr (or bbcpr.exe on Windows)
```

### Development Build
```bash
# Development build (faster compile, slower runtime)
cargo build

# Run tests
cargo test

# Run with debug output
cargo run -- --help
```

### Custom Build Options
```bash
# Static linking (Linux)
cargo build --release --target x86_64-unknown-linux-musl

# Optimized for size
cargo build --release --config profile.release.opt-level='"z"'

# Cross-compilation (requires cross)
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

## ✅ Verification

After installation, verify bbcpr is working:

```bash
# Check version
bbcpr --version

# Run help
bbcpr --help

# Test basic functionality
echo "test" > test.txt
bbcpr test.txt test_copy.txt
ls -la test*
rm test.txt test_copy.txt
```

Expected output:
```
bbcpr version 0.1.0
Copyright (C) 2025 Andrew Mello
License GPLv3+: GNU GPL version 3 or later
```

## 🔄 Updates

### Automatic Updates (Install Script)
```bash
# Re-run install script to get latest version
curl -sSL https://raw.githubusercontent.com/88plug/bbcpr/rust-rewrite/install.sh | bash
```

### Manual Updates
1. Download new binary from [releases page](https://github.com/88plug/bbcpr/releases)
2. Replace existing binary
3. Verify new version: `bbcpr --version`

### Package Manager Updates
```bash
# Arch Linux
yay -Syu bbcpr

# Homebrew
brew upgrade bbcpr

# Chocolatey
choco upgrade bbcpr

# Snap
sudo snap refresh bbcpr
```

## 🗑️ Uninstallation

### Manual Installation
```bash
# Remove binary
sudo rm /usr/local/bin/bbcpr
# OR for user installation
rm ~/.local/bin/bbcpr

# Remove from PATH (edit your shell profile)
# Remove the export PATH line that was added
```

### Package Managers
```bash
# Arch Linux
yay -R bbcpr

# Homebrew
brew uninstall bbcpr

# Chocolatey
choco uninstall bbcpr

# Snap
sudo snap remove bbcpr
```

### Complete Cleanup
```bash
# Remove configuration files (if any)
rm -rf ~/.config/bbcpr
rm -rf ~/.bbcpr

# Remove from shell profile
# Edit ~/.bashrc, ~/.zshrc, etc. and remove bbcpr-related lines
```

## 🐛 Troubleshooting

### Common Issues

#### "Command not found"
- Check if binary is in PATH: `echo $PATH`
- Verify installation location: `which bbcpr`
- Source your shell profile: `source ~/.bashrc`

#### Permission denied
```bash
# Make binary executable
chmod +x /path/to/bbcpr
```

#### SSL/TLS errors
```bash
# Install CA certificates
sudo apt install ca-certificates  # Ubuntu/Debian
sudo dnf install ca-certificates  # Fedora/RHEL
```

#### Build failures
```bash
# Update Rust
rustup update

# Clear build cache
cargo clean

# Install missing dependencies (see platform-specific sections above)
```

### Getting Help
- Check [Troubleshooting Guide](Troubleshooting)
- File an issue: [GitHub Issues](https://github.com/88plug/bbcpr/issues)
- Join discussions: [GitHub Discussions](https://github.com/88plug/bbcpr/discussions)

---

**Next Steps:**
- [Quick Start Guide](Quick-Start) - Basic usage
- [Command Reference](Command-Reference) - All available options
- [Performance Guide](Performance-Guide) - Optimization tips
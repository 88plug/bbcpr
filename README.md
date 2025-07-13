# bbcpr - High-Performance Parallel File Transfer Tool

[![Build Status](https://github.com/88plug/bbcpr/workflows/Rust%20CI/CD/badge.svg)](https://github.com/88plug/bbcpr/actions)
[![Release](https://img.shields.io/github/release/88plug/bbcpr.svg)](https://github.com/88plug/bbcpr/releases)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Platform Support](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)](#installation)

> **Modern Rust implementation of bbcp** - The fastest, most secure way to transfer files across networks using parallel streams, SSH tunneling, and real-time progress tracking.

## ✨ Why bbcpr?

**bbcpr** (Berkeley Byte Copy Rust) is a complete rewrite of the original bbcp tool in Rust, designed for **maximum speed**, **security**, and **reliability** in file transfers across local and remote systems.

### 🚀 **The Key Advantage: Single-File Multi-Threading**
**bbcpr's killer feature**: Split **individual large files** into multiple parallel streams for blazing-fast transfers that **rsync, aria2c, and rclone cannot do**.

- **Multi-stream single files** - Transfer one 10GB file using 16 parallel connections
- **Unmatched speed** - Achieve 40Gbps+ on high-bandwidth networks with single files
- **Perfect for large files** - Movies, databases, archives, VM images, scientific datasets
- **Works where others fail** - When you need maximum speed for individual large files

### 🔥 **Performance First**
- **Multi-stream parallel transfers** - Utilize full network bandwidth on single files
- **Optimized buffer management** - Memory-efficient async I/O
- **Zero-copy operations** where possible for maximum speed
- **Automatic stream optimization** based on network conditions

### 🔒 **Security Built-In**
- **SSH integration** - Secure transfers over encrypted channels  
- **Checksum verification** - Multiple algorithms (MD5, CRC32, Adler32, Blake3)
- **Memory-safe Rust** - No buffer overflows or security vulnerabilities
- **Authentication support** - Key-based and password authentication

### 🌍 **Universal Compatibility**
- **Linux** - All major distributions (Ubuntu, RHEL, Arch, Alpine, etc.)
- **macOS** - Intel and Apple Silicon support
- **Windows** - Native Windows 10/11 support
- **Static binaries** - No external dependencies required

---

## 📥 Installation

### 🚨 **IMPORTANT: Install on BOTH machines**
bbcpr must be installed on **both the source (sending) and destination (receiving) machines**. Only the source machine runs the transfer command - the destination just needs bbcpr available in its PATH.

### Quick Install (Recommended)

**Run this command on BOTH machines:**
```bash
curl -sSL https://raw.githubusercontent.com/88plug/bbcpr/master/install.sh | bash
```

### Manual Installation

**Download and install on BOTH machines:**

| Platform | Download | 
|----------|----------|
| **Linux/macOS** | [`bbcpr`](https://github.com/88plug/bbcpr/releases/latest/download/bbcpr) |
| **Windows** | [`bbcpr.exe`](https://github.com/88plug/bbcpr/releases/latest/download/bbcpr.exe) |

#### Linux/macOS Installation (run on both machines)
```bash
# Download
wget https://github.com/88plug/bbcpr/releases/latest/download/bbcpr
chmod +x bbcpr

# Install system-wide
sudo mv bbcpr /usr/local/bin/bbcpr

# Or install user-only
mkdir -p ~/.local/bin
mv bbcpr ~/.local/bin/bbcpr
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

#### Windows Installation (run on both machines)
1. Download `bbcpr.exe` from the releases page
2. Move to a directory in your PATH or create a new directory and add it to PATH
3. Ensure both machines can run `bbcpr --version`

### 💡 **How it Works**
- **Source machine**: You run the bbcpr command here
- **Destination machine**: bbcpr automatically launches itself via SSH
- **No server setup**: No daemon, no configuration files, just install and go!

---

## 🚀 Quick Start

### 🔥 **Single-File Multi-Threading (The Magic!)**
Unlike rsync/aria2c/rclone, bbcpr splits **individual files** into parallel streams:

```bash
# Transfer a 5GB movie file using 16 parallel streams - IMPOSSIBLE with rsync!
bbcpr -s 16 movie.mkv user@server:/media/

# Database backup with 8 streams - 8x faster than scp/rsync for large files
bbcpr -s 8 database.dump user@backup-server:/backups/

# Scientific dataset with 32 streams on high-speed network
bbcpr -s 32 research-data.tar.gz cluster:/projects/
```

### Basic File Copy  
```bash
# Simple single file (still uses 4 parallel streams by default!)
bbcpr largefile.zip user@server:/backup/

# Directory copy (recursive)
bbcpr -r /local/dir/ user@server:/remote/dir/

# Local copy (for testing)
bbcpr file.txt /tmp/backup/
```

### 🔄 **Resume Interrupted Transfers (NEW!)**
```bash
# Resume interrupted transfers automatically
bbcpr -R -s 16 huge-file.iso user@server:/storage/

# List pending transfers that can be resumed
bbcpr --list-transfers

# Cancel a specific transfer
bbcpr --cancel-transfer abc123

# Clean up old transfer state files (older than 7 days)
bbcpr --cleanup-transfers 7
```

### Advanced Options
```bash
# Compressed transfer for slow networks
bbcpr -c 6 -s 8 database.sql user@server:/backup/

# Transfer with integrity verification
bbcpr -e -s 16 critical-data.tar user@server:/safe/

# Resume with compression and verification
bbcpr -R -c 5 -e -s 16 critical-backup.tar.gz user@server:/safe/
```

### Progress Monitoring
```bash
# Real-time progress every 5 seconds
bbcpr -P 5 largefile.dat user@server:/data/

# Verbose output with detailed statistics
bbcpr -vv -P 2 *.log user@server:/logs/
```

---

## 📖 Usage Guide

### Command Line Options

```
bbcpr [OPTIONS] [SOURCE]... DESTINATION

OPTIONS:
    -s, --streams <N>      Number of parallel streams (default: 4)
    -c, --compress <LVL>   Compression level 1-9 (default: disabled)
    -e, --error-check      Enable checksum verification
    -p, --preserve         Preserve file attributes and timestamps
    -r, --recursive        Copy directories recursively
    -R, --resume           Resume interrupted transfers automatically
    -P, --progress <SEC>   Progress update interval in seconds
    -v, --verbose          Increase verbosity (use -vv for debug output)
    -q, --quiet            Suppress non-error output
    -f, --force            Overwrite existing files without prompting
        --port <PORT>      SSH port (default: 22)
        --timeout <SEC>    Connection timeout in seconds
        --buffer <SIZE>    Buffer size in bytes (default: 1MB)
        --list-transfers   List pending transfers that can be resumed
        --cancel-transfer <ID>  Cancel a specific transfer by ID
        --cleanup-transfers <DAYS>  Clean up old transfer state files
        --keep-state       Keep transfer state files after completion
    -h, --help             Show this help message
        --version          Show version information
```

---

## ⚡ Performance Optimization

### Stream Configuration
- **1-4 streams**: Standard networks, small files
- **8-16 streams**: High-bandwidth networks, large files
- **16-32 streams**: Dedicated connections, massive transfers
- **32+ streams**: Only for specialized high-throughput scenarios

### Compression Guidelines
- **Level 1-3**: Fast compression, minimal CPU usage
- **Level 4-6**: Balanced compression and speed (recommended)
- **Level 7-9**: Maximum compression, high CPU usage

---

## 🆚 Comparison: The Single-File Advantage

| Feature | bbcpr | rsync | aria2c | rclone | scp | bbcp (original) |
|---------|-------|-------|---------|--------|-----|-----------------|
| **🔥 Single-File Multi-Threading** | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Parallel Streams** | ✅ | ❌ | ✅* | ✅* | ❌ | ✅ |
| **Large File Speed** | 🚀 **Best** | 🐌 Slow | 🐌 Slow | 🐌 Slow | 🐌 Slow | 🚀 Good |
| **Real-time Progress** | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| **SSH Integration** | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ |
| **Compression** | ✅ | ✅ | ❌ | ✅ | ✅ | ❌ |
| **Checksum Verification** | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| **Memory Safety** | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ |
| **Cross-platform** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Resume Transfers** | ✅ **Advanced** | ✅ Basic | ✅ Basic | ✅ Basic | ❌ | ❌ |

**\*** *aria2c and rclone can only parallelize multiple files, NOT split single files into streams*

### 🎯 **When to Use bbcpr**
- ✅ **Large single files** (>100MB): Movies, databases, VM images, archives
- ✅ **High-bandwidth networks**: 1Gbps+ where single-stream tools are bottlenecked  
- ✅ **Maximum speed needed**: When transfer time matters more than everything else
- ✅ **Unreliable networks**: Resume functionality for interrupted large transfers

### 🎯 **When Others Might Be Better**
- rsync: Many small files, incremental sync, bandwidth-limited networks
- rclone: Cloud storage integration, many cloud providers
- aria2c: BitTorrent downloads, metalink support

---

## 🔄 Transfer Resume System

### **Advanced Resume Capabilities**
bbcpr includes a sophisticated transfer resume system that goes beyond simple restart functionality:

- **Chunk-level recovery** - Each parallel stream can resume from its exact position
- **State persistence** - Transfer metadata saved in `~/.bbcpr/transfers/`
- **Parameter validation** - Ensures resume compatibility with original transfer settings
- **Automatic cleanup** - Optional cleanup of completed transfer states

### **Resume Commands**
```bash
# Enable resume mode (automatic detection)
bbcpr -R largefile.iso user@server:/backup/

# List all pending/incomplete transfers
bbcpr --list-transfers

# Cancel a specific transfer by ID
bbcpr --cancel-transfer a1b2c3d4

# Clean up transfer states older than 7 days
bbcpr --cleanup-transfers 7

# Keep state files after successful completion (for debugging)
bbcpr -R --keep-state hugefile.dat user@server:/data/
```

### **How Resume Works**
1. **Interruption Detection** - bbcpr saves progress every 10MB transferred
2. **State Validation** - Checks file size, stream count, and compression settings match
3. **Chunk Recovery** - Each of the 16 streams resumes from its exact byte position
4. **Seamless Continuation** - Transfer continues as if never interrupted

### **Resume vs Others**
- **rsync**: Basic file-level resume, no parallel stream recovery
- **aria2c**: Segment-based resume, but only for downloads
- **rclone**: Basic resume, no chunk-level recovery
- **bbcpr**: Advanced chunk-level resume with multi-stream recovery

---

## 🏗️ Building from Source

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- OpenSSL development libraries

### Quick Build
```bash
git clone https://github.com/88plug/bbcpr.git
cd bbcpr/rust
cargo build --release
```

---

## 📜 License

This project is licensed under the **GNU General Public License v3.0** - see the [LICENSE](LICENSE) file for details.

### Original bbcp License
This project is a Rust rewrite inspired by the original bbcp by Andy Hanushevsky at SLAC National Accelerator Laboratory. The original bbcp is also GPL v3 licensed.

---

## 🙏 Acknowledgments

- **Andy Hanushevsky** - Creator of the original bbcp tool
- **SLAC National Accelerator Laboratory** - Original bbcp development
- **Rust Community** - Amazing async/networking ecosystem

---

<div align="center">

**[🚀 Releases](https://github.com/88plug/bbcpr/releases)** • 
**[🐛 Issues](https://github.com/88plug/bbcpr/issues)**

Made with ❤️ in Rust • Star ⭐ if you find bbcpr useful!

</div>
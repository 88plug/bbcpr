# bbcpr v0.2.0 Release - Advanced Resume Functionality

## 🔄 Major New Features
- **Advanced resume functionality** with chunk-level recovery
- **Transfer state persistence** - Transfers survive interruptions 
- **Transfer management** - List, cancel, and cleanup transfers
- **Parameter validation** - Ensures resume compatibility

## Resume Commands
```bash
# Resume interrupted transfers automatically
bbcpr -R -s 16 huge-file.iso user@server:/storage/

# List pending transfers that can be resumed
bbcpr --list-transfers

# Cancel a specific transfer by ID
bbcpr --cancel-transfer abc123

# Clean up old transfer state files (older than 7 days)
bbcpr --cleanup-transfers 7
```

## Core Features
- **Single-file multi-threading** - Split individual large files into parallel streams (unique advantage over rsync/aria2c/rclone)
- Cross-platform file transfer utility written in Rust
- Multi-stream parallel transfers for high performance  
- SSH and TCP connection support
- Real-time progress reporting
- Multiple checksum algorithms (MD5, CRC32, Adler32, Blake3)
- Memory-safe implementation with modern async I/O

## Installation

**IMPORTANT**: Install bbcpr on BOTH source and destination machines.

### Quick Install (Both machines)
```bash
curl -sSL https://raw.githubusercontent.com/88plug/bbcpr/master/install.sh | bash
```

### Manual Installation

**Linux/macOS (both machines)**
```bash
wget https://github.com/88plug/bbcpr/releases/download/v0.2.0/bbcpr
chmod +x bbcpr
sudo mv bbcpr /usr/local/bin/bbcpr
```

**Windows (both machines)**
Download `bbcpr.exe` and add to PATH.

## What's Changed
- Implemented comprehensive transfer resume system
- Added chunk-level recovery for interrupted transfers
- Added transfer state management commands
- Enhanced documentation with resume functionality
- Fixed project name from "Berkeley Byte Copy Plus Rust" to "Berkeley Byte Copy Rust"
- Updated all CLI options and help text

## Compatibility
- **Linux**: Works on all major distributions (Ubuntu, RHEL, Arch, Alpine, etc.)
- **macOS**: Intel and Apple Silicon Macs  
- **Windows**: Windows 10/11 x64
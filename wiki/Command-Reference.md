# Command Reference

Complete reference for all bbcpr command-line options and usage patterns.

## 📝 Synopsis

```
bbcpr [OPTIONS] [SOURCE]... DESTINATION
```

## 🎯 Basic Usage

### Single File Copy
```bash
bbcpr source.txt destination.txt
bbcpr file.dat /backup/
bbcpr file.txt user@server:/remote/path/
```

### Multiple Files
```bash
bbcpr file1.txt file2.txt file3.txt /backup/
bbcpr *.log user@server:/logs/
bbcpr data/*.csv user@server:/datasets/
```

### Directory Copy
```bash
bbcpr -r /local/dir/ /backup/dir/
bbcpr -r documents/ user@server:/backup/
```

## ⚙️ Command-Line Options

### Transfer Options

#### `-s, --streams <N>`
Number of parallel streams to use for transfer.

```bash
bbcpr -s 4 file.dat server:/dest/     # 4 streams (default)
bbcpr -s 16 file.dat server:/dest/    # 16 streams (high performance)
bbcpr -s 1 file.dat server:/dest/     # Single stream
```

**Guidelines:**
- **1-4**: Standard networks, small files
- **8-16**: High-bandwidth networks, large files  
- **16-32**: Dedicated connections, massive transfers
- **Default**: 4 streams

#### `-c, --compress <LEVEL>`
Enable compression with specified level (1-9).

```bash
bbcpr -c 1 data/ server:/backup/      # Fast compression
bbcpr -c 4 data/ server:/backup/      # Balanced (recommended)
bbcpr -c 9 data/ server:/backup/      # Maximum compression
```

**Compression Levels:**
- **1-3**: Fast compression, low CPU usage
- **4-6**: Balanced compression and speed *(recommended)*
- **7-9**: Maximum compression, high CPU usage
- **Default**: Disabled

#### `-e, --error-check`
Enable checksum verification for data integrity.

```bash
bbcpr -e critical.data server:/safe/
bbcpr -s 8 -e database.dump server:/backup/
```

**Checksum Algorithms:**
- MD5, CRC32, Adler32, Blake3
- Automatic algorithm selection
- Verifies data integrity end-to-end

#### `--buffer <SIZE>`
Set buffer size for data transfer.

```bash
bbcpr --buffer 1MB file.dat server:/dest/    # Default
bbcpr --buffer 4MB file.dat server:/dest/    # Large buffer
bbcpr --buffer 512KB file.dat server:/dest/  # Small buffer
```

**Buffer Size Guidelines:**
- **256KB-1MB**: Standard networks, low memory
- **1MB-4MB**: High-bandwidth networks *(default: 1MB)*
- **4MB-8MB**: Very high-speed networks
- **>8MB**: Specialized high-throughput scenarios

### File Options

#### `-r, --recursive`
Copy directories recursively.

```bash
bbcpr -r /source/dir/ /dest/dir/
bbcpr -r documents/ user@server:/backup/
```

#### `-p, --preserve`
Preserve file attributes, permissions, and timestamps.

```bash
bbcpr -p file.txt server:/backup/
bbcpr -r -p /important/ server:/archive/
```

**Preserves:**
- File permissions (mode)
- Ownership (where possible)
- Timestamps (access, modification)
- Extended attributes (where supported)

#### `-f, --force`
Overwrite existing files without prompting.

```bash
bbcpr -f file.txt server:/existing/
bbcpr -r -f backup/ server:/restore/
```

### Output Options

#### `-v, --verbose`
Increase output verbosity. Use multiple times for more detail.

```bash
bbcpr -v file.dat server:/dest/       # Basic verbose
bbcpr -vv file.dat server:/dest/      # Detailed verbose
bbcpr -vvv file.dat server:/dest/     # Debug level
```

**Verbosity Levels:**
- **Default**: Errors and basic progress
- **-v**: Transfer statistics, warnings
- **-vv**: Detailed transfer info, performance metrics
- **-vvv**: Debug information, protocol details

#### `-q, --quiet`
Suppress all non-error output.

```bash
bbcpr -q file.dat server:/dest/
bbcpr -q -s 16 largefile.iso server:/storage/
```

#### `-P, --progress <SECONDS>`
Show progress updates every N seconds.

```bash
bbcpr -P 5 file.dat server:/dest/     # Every 5 seconds
bbcpr -P 1 file.dat server:/dest/     # Every second
bbcpr -P 30 file.dat server:/dest/    # Every 30 seconds
```

**Progress Information:**
- Transfer rate (MB/s)
- Percentage complete
- Estimated time remaining
- Data transferred / total size

### Network Options

#### `--port <PORT>`
Specify SSH port for remote connections.

```bash
bbcpr --port 2022 file.dat user@server:/dest/
bbcpr --port 22222 data/ admin@secure-server:/backup/
```

**Default**: 22 (standard SSH port)

#### `--timeout <SECONDS>`
Set connection timeout for network operations.

```bash
bbcpr --timeout 30 file.dat user@server:/dest/   # 30 second timeout
bbcpr --timeout 120 file.dat user@slow:/dest/    # 2 minute timeout
```

**Default**: 60 seconds

#### `-i, --identity <KEY_FILE>`
Specify SSH private key file for authentication.

```bash
bbcpr -i ~/.ssh/backup_key file.dat server:/backup/
bbcpr -i /path/to/key data/ user@server:/dest/
```

**Key Requirements:**
- Private key file (RSA, DSA, ECDSA, Ed25519)
- Proper permissions (600 recommended)
- Corresponding public key on remote server

### Information Options

#### `-h, --help`
Show help message and exit.

```bash
bbcpr --help
bbcpr -h
```

#### `--version`
Show version information and exit.

```bash
bbcpr --version
```

**Output includes:**
- bbcpr version number
- Build information
- License information

## 🔗 Option Combinations

### High-Performance Transfer
```bash
bbcpr -s 16 --buffer 4MB -P 5 hugefile.dat server:/storage/
```

### Secure Critical Data Transfer
```bash
bbcpr -s 8 -e -i ~/.ssh/secure_key -P 10 critical.data vault:/secure/
```

### Compressed Backup
```bash
bbcpr -r -c 4 -p -P 30 /data/ backup-server:/daily/$(date +%Y%m%d)/
```

### Debug Network Issues
```bash
bbcpr -vvv -s 1 --timeout 120 -P 1 test.dat problematic-server:/tmp/
```

### Quiet Automated Transfer
```bash
bbcpr -q -s 8 -c 4 logs/ automated-backup:/archive/
```

## 📁 Path Specifications

### Local Paths
```bash
bbcpr file.txt /absolute/path/
bbcpr file.txt ./relative/path/
bbcpr file.txt ~/home/path/
bbcpr file.txt ../parent/path/
```

### Remote Paths (SSH)
```bash
bbcpr file.txt user@host:/absolute/path/
bbcpr file.txt user@host:relative/path/
bbcpr file.txt user@host:~/home/path/
bbcpr file.txt user@host:.              # Home directory
```

### Special Characters
```bash
# Spaces in paths (quote the path)
bbcpr "file with spaces.txt" "user@host:/path with spaces/"

# Special characters (escape or quote)
bbcpr "file[special].txt" user@host:"/path/special\$dir/"
```

## 🌐 Remote Server Specifications

### Basic Format
```
[user@]hostname[:path]
```

### Examples
```bash
# Default user (current username)
bbcpr file.txt server:/dest/

# Specific user
bbcpr file.txt backup@server:/backups/

# Custom port (use --port option)
bbcpr --port 2022 file.txt user@server:/dest/

# IPv6 addresses
bbcpr file.txt user@[2001:db8::1]:/dest/

# Hostname with domain
bbcpr file.txt user@backup.example.com:/storage/
```

## 📊 Exit Codes

| Code | Meaning | Description |
|------|---------|-------------|
| **0** | Success | Transfer completed successfully |
| **1** | General Error | Generic error (see error message) |
| **2** | Usage Error | Invalid command-line arguments |
| **3** | File Error | File/directory access issues |
| **4** | Network Error | Connection or transfer failures |
| **5** | Auth Error | Authentication/permission failures |
| **6** | Checksum Error | Data integrity verification failed |

### Using Exit Codes in Scripts
```bash
#!/bin/bash

bbcpr -s 8 data.dat backup-server:/storage/
case $? in
    0) echo "Transfer successful" ;;
    1) echo "General error occurred" ; exit 1 ;;
    2) echo "Usage error - check command" ; exit 1 ;;
    3) echo "File access error" ; exit 1 ;;
    4) echo "Network error" ; exit 1 ;;
    5) echo "Authentication failed" ; exit 1 ;;
    6) echo "Checksum verification failed" ; exit 1 ;;
    *) echo "Unknown error" ; exit 1 ;;
esac
```

## 🔧 Environment Variables

bbcpr respects these environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `BBCPR_STREAMS` | 4 | Default number of streams |
| `BBCPR_COMPRESS` | disabled | Default compression level |
| `BBCPR_PROGRESS` | disabled | Default progress interval |
| `BBCPR_BUFFER` | 1MB | Default buffer size |
| `BBCPR_TIMEOUT` | 60 | Default timeout (seconds) |
| `SSH_AUTH_SOCK` | (system) | SSH agent socket |
| `SSH_CONFIG_FILE` | ~/.ssh/config | SSH configuration file |

### Setting Environment Variables
```bash
# Temporary (current session)
export BBCPR_STREAMS=16
export BBCPR_COMPRESS=4
bbcpr file.dat server:/dest/

# Permanent (add to ~/.bashrc or ~/.zshrc)
echo 'export BBCPR_STREAMS=8' >> ~/.bashrc
echo 'export BBCPR_COMPRESS=4' >> ~/.bashrc
source ~/.bashrc
```

## 📝 Configuration Files

### SSH Configuration
bbcpr uses standard SSH configuration files:

**System-wide**: `/etc/ssh/ssh_config`
**User-specific**: `~/.ssh/config`

Example `~/.ssh/config`:
```
Host backup-server
    HostName backup.example.com
    User backup-user
    Port 2022
    IdentityFile ~/.ssh/backup_key
    Compression yes
    CompressionLevel 4
```

Usage:
```bash
bbcpr -s 16 data/ backup-server:/storage/
```

## 🎯 Usage Patterns

### Basic Patterns
```bash
# File to file
bbcpr source.txt destination.txt

# File to directory
bbcpr file.txt /backup/

# Multiple files to directory
bbcpr *.txt /backup/

# Directory to directory
bbcpr -r source_dir/ dest_dir/

# Remote copy
bbcpr file.txt user@server:/path/
```

### Advanced Patterns
```bash
# High-performance large file
bbcpr -s 32 --buffer 8MB -P 5 hugefile.dat server:/storage/

# Compressed directory backup
bbcpr -r -c 6 -p /important/ backup:/daily/$(date +%Y%m%d)/

# Verified critical transfer
bbcpr -s 8 -e -i ~/.ssh/secure_key critical.data vault:/secure/

# Bandwidth-limited transfer
bbcpr -s 2 -c 9 data/ slow-server:/archive/

# Debug problematic transfer
bbcpr -vvv -s 1 --timeout 300 problem.dat debug-server:/tmp/
```

## 🚨 Common Errors and Solutions

### Authentication Errors
```bash
# Error: Permission denied (publickey)
# Solution: Check SSH key and server configuration
ssh user@server  # Test SSH access first
bbcpr -i ~/.ssh/specific_key file.dat server:/dest/
```

### Network Errors
```bash
# Error: Connection timeout
# Solution: Increase timeout or reduce streams
bbcpr --timeout 120 -s 2 file.dat server:/dest/
```

### File Errors
```bash
# Error: No such file or directory
# Solution: Check paths and permissions
ls -la source.txt  # Verify source exists
ssh server "ls -la /dest/"  # Verify destination accessible
```

### Performance Issues
```bash
# Problem: Slow transfer
# Solution: Monitor and adjust parameters
bbcpr -vv -P 1 -s 16 file.dat server:/dest/  # Monitor performance
bbcpr -s 8 --buffer 4MB file.dat server:/dest/  # Adjust parameters
```

---

**Related Pages:**
- [Quick Start](Quick-Start) - Basic usage examples
- [Performance Guide](Performance-Guide) - Optimization techniques
- [SSH Configuration](SSH-Configuration) - Remote access setup
- [Troubleshooting](Troubleshooting) - Problem solving
# Quick Start Guide

Get up and running with bbcpr in 5 minutes! This guide covers the essential commands you need to start transferring files efficiently.

## 🚀 Installation (30 seconds)

```bash
curl -sSL https://raw.githubusercontent.com/88plug/bbcpr/rust-rewrite/install.sh | bash
```

Verify installation:
```bash
bbcpr --version
```

## 📁 Basic File Transfers

### Local Copies
```bash
# Copy a single file
bbcpr source.txt destination.txt

# Copy to a directory
bbcpr file.txt /backup/

# Copy multiple files
bbcpr *.log /var/log/backup/
```

### Remote Copies (SSH)
```bash
# Copy to remote server
bbcpr file.txt user@server:/remote/path/

# Copy from remote server
bbcpr user@server:/remote/file.txt ./local/

# Copy between remote servers
bbcpr user1@server1:/file.txt user2@server2:/dest/
```

### Directory Copies
```bash
# Copy directory recursively
bbcpr -r /local/dir/ user@server:/remote/dir/

# Preserve permissions and timestamps
bbcpr -r -p /important/ user@server:/backup/
```

## ⚡ High-Performance Transfers

### Parallel Streams
```bash
# Use 8 parallel streams for faster transfers
bbcpr -s 8 largefile.iso user@server:/storage/

# Maximum performance for dedicated networks
bbcpr -s 16 hugefile.dat user@server:/fast-storage/
```

### Compression
```bash
# Compress data for slow networks
bbcpr -c 4 database.sql user@server:/backup/

# High compression for text files
bbcpr -c 6 logs/ user@server:/archive/
```

### Progress Monitoring
```bash
# Show progress every 5 seconds
bbcpr -P 5 largefile.dat user@server:/dest/

# Verbose output with detailed stats
bbcpr -vv -P 2 dataset/ user@server:/storage/
```

## 🔒 Secure Transfers

### SSH Key Authentication
```bash
# Use specific SSH key
bbcpr -i ~/.ssh/backup_key sensitive.data secure-server:/vault/

# Custom SSH port
bbcpr --port 2022 data/ user@server:/secure/
```

### Integrity Verification
```bash
# Enable checksum verification
bbcpr -e critical.data user@server:/safe/

# Combine with other options
bbcpr -s 8 -e -P 5 database.dump backup-server:/secure/
```

## 📊 Real-World Examples

### Daily Backup
```bash
# Automated backup with compression and progress
bbcpr -r -c 4 -P 10 /home/user/documents/ backup-server:/daily/$(date +%Y%m%d)/
```

### Large Dataset Transfer
```bash
# Scientific data with maximum performance
bbcpr -s 32 -e -P 5 research_data.hdf5 cluster:/projects/
```

### Log File Collection
```bash
# Compressed log transfer
bbcpr -c 6 /var/log/*.log log-server:/collected/$(hostname)/
```

### Media File Transfer
```bash
# Video files (already compressed, use maximum streams)
bbcpr -s 16 *.mkv media-server:/videos/
```

## 🛠️ Common Options Quick Reference

| Option | Purpose | Example |
|--------|---------|---------|
| `-s N` | Use N parallel streams | `bbcpr -s 8 file.dat server:/dest/` |
| `-c N` | Compression level 1-9 | `bbcpr -c 4 data/ server:/backup/` |
| `-r` | Recursive (directories) | `bbcpr -r dir/ server:/dest/` |
| `-p` | Preserve attributes | `bbcpr -r -p /important/ server:/backup/` |
| `-e` | Checksum verification | `bbcpr -e critical.data server:/safe/` |
| `-P N` | Progress every N seconds | `bbcpr -P 5 file.dat server:/dest/` |
| `-v` | Verbose output | `bbcpr -vv file.dat server:/dest/` |
| `-q` | Quiet mode | `bbcpr -q file.dat server:/dest/` |

## 🎯 Performance Tips

### Choose the Right Stream Count
```bash
# Standard networks (1Gbps)
bbcpr -s 4 file.dat server:/dest/

# High-speed networks (10Gbps+)
bbcpr -s 16 file.dat server:/dest/

# Slow/unreliable networks
bbcpr -s 2 file.dat server:/dest/
```

### Optimize for File Type
```bash
# Large single files
bbcpr -s 16 --buffer 4MB movie.mkv server:/media/

# Many small files (archive first)
tar -czf archive.tar.gz small_files/
bbcpr -s 8 archive.tar.gz server:/dest/

# Database dumps
bbcpr -s 8 -c 4 -e database.dump server:/backups/
```

## 🔧 Configuration

### SSH Config for Easier Access
Create `~/.ssh/config`:
```
Host backup-server
    HostName backup.example.com
    User backup-user
    Port 2022
    IdentityFile ~/.ssh/backup_key
```

Then use simplified commands:
```bash
bbcpr -s 8 data/ backup-server:/storage/
```

### Environment Variables
```bash
export BBCPR_STREAMS=8           # Default 8 streams
export BBCPR_COMPRESS=4          # Default compression level 4
export BBCPR_PROGRESS=5          # Show progress every 5 seconds

# Now bbcpr uses these defaults
bbcpr file.dat server:/dest/
```

## 🚨 Common Mistakes to Avoid

### ❌ Don't Do This
```bash
# Too many streams for small files
bbcpr -s 64 small.txt server:/dest/

# Compressing already compressed files
bbcpr -c 9 movie.mp4 server:/media/

# No progress monitoring for large transfers
bbcpr hugefile.dat server:/dest/  # You won't know when it finishes
```

### ✅ Do This Instead
```bash
# Appropriate streams for file size
bbcpr -s 4 small.txt server:/dest/

# Skip compression for compressed files
bbcpr -s 16 movie.mp4 server:/media/

# Always monitor large transfers
bbcpr -s 16 -P 5 hugefile.dat server:/dest/
```

## 🔍 Troubleshooting Quick Fixes

### Connection Issues
```bash
# Test basic connectivity
ssh user@server echo "Connection OK"

# Use custom SSH port
bbcpr --port 2022 file.dat user@server:/dest/

# Increase timeout for slow networks
bbcpr --timeout 60 file.dat user@server:/dest/
```

### Performance Issues
```bash
# Monitor transfer in real-time
bbcpr -vv -P 1 file.dat server:/dest/

# Reduce streams if network is saturated
bbcpr -s 2 file.dat server:/dest/

# Test without compression
bbcpr -s 8 file.dat server:/dest/
```

### Permission Issues
```bash
# Ensure SSH key permissions are correct
chmod 600 ~/.ssh/id_rsa

# Test SSH access first
ssh user@server "ls -la /dest/"

# Use specific SSH key
bbcpr -i ~/.ssh/specific_key file.dat server:/dest/
```

## 🎓 Next Steps

Now that you have the basics down:

1. **[Performance Guide](Performance-Guide)** - Optimize for your network
2. **[Command Reference](Command-Reference)** - Complete option list
3. **[SSH Configuration](SSH-Configuration)** - Advanced remote setup
4. **[Troubleshooting](Troubleshooting)** - Solve common issues

## 📋 Cheat Sheet

Save this for quick reference:

```bash
# Basic copy
bbcpr file.txt user@server:/dest/

# High performance
bbcpr -s 16 -P 5 file.dat user@server:/dest/

# Slow network
bbcpr -s 4 -c 4 file.dat user@server:/dest/

# Directory backup
bbcpr -r -p -c 4 /data/ backup-server:/daily/

# Critical data
bbcpr -s 8 -e -P 10 important.dat secure-server:/vault/

# Check what's happening
bbcpr -vv -P 2 file.dat server:/dest/
```

---

**Questions?** Check the [FAQ](FAQ) or [ask for help](https://github.com/88plug/bbcpr/discussions)!
# Troubleshooting Guide

Solutions for common issues and problems when using bbcpr.

## 🚨 Quick Diagnosis

### Test Basic Functionality
```bash
# 1. Check bbcpr is installed and working
bbcpr --version

# 2. Test local copy
echo "test" > test.txt
bbcpr test.txt test_copy.txt
rm test.txt test_copy.txt

# 3. Test SSH connectivity (for remote transfers)
ssh user@server echo "SSH OK"

# 4. Test basic remote copy
echo "test" > test.txt
bbcpr test.txt user@server:/tmp/
ssh user@server "cat /tmp/test.txt && rm /tmp/test.txt"
rm test.txt
```

## 🔐 Authentication Issues

### "Permission denied (publickey)"

**Cause**: SSH authentication failure

**Solutions**:
```bash
# 1. Test SSH access first
ssh user@server

# 2. Check SSH key permissions
chmod 600 ~/.ssh/id_rsa
chmod 644 ~/.ssh/id_rsa.pub
chmod 700 ~/.ssh

# 3. Use specific SSH key
bbcpr -i ~/.ssh/specific_key file.txt server:/dest/

# 4. Check SSH agent
ssh-add -l
ssh-add ~/.ssh/id_rsa

# 5. Use password authentication (if enabled)
ssh -o PasswordAuthentication=yes user@server
```

### "Host key verification failed"

**Cause**: SSH host key not recognized

**Solutions**:
```bash
# 1. Add host key to known_hosts
ssh-keyscan server >> ~/.ssh/known_hosts

# 2. Or connect via SSH first to accept key
ssh user@server

# 3. Remove old host key if server changed
ssh-keygen -R server
```

### "Authentication timeout"

**Cause**: SSH authentication taking too long

**Solutions**:
```bash
# 1. Increase timeout
bbcpr --timeout 120 file.txt server:/dest/

# 2. Check SSH configuration
ssh -v user@server  # Verbose SSH debug

# 3. Use SSH connection multiplexing
# Add to ~/.ssh/config:
Host server
    ControlMaster auto
    ControlPath ~/.ssh/sockets/%r@%h-%p
    ControlPersist 600
```

## 🌐 Network Issues

### "Connection refused"

**Cause**: Cannot connect to remote server

**Solutions**:
```bash
# 1. Check server is reachable
ping server

# 2. Check SSH port is open
telnet server 22
# Or for custom port:
telnet server 2022

# 3. Use correct port
bbcpr --port 2022 file.txt user@server:/dest/

# 4. Check firewall settings
# On server: sudo ufw status
# On client: check local firewall
```

### "Connection timeout"

**Cause**: Network connectivity issues

**Solutions**:
```bash
# 1. Increase timeout
bbcpr --timeout 300 file.txt server:/dest/

# 2. Reduce parallel streams
bbcpr -s 2 file.txt server:/dest/

# 3. Test network stability
ping -c 10 server
mtr server  # If available

# 4. Use smaller buffer size
bbcpr --buffer 512KB file.txt server:/dest/
```

### "Transfer stalls/hangs"

**Cause**: Network congestion or instability

**Solutions**:
```bash
# 1. Reduce streams and buffer size
bbcpr -s 2 --buffer 256KB file.txt server:/dest/

# 2. Monitor in real-time
bbcpr -vv -P 1 file.txt server:/dest/

# 3. Disable compression (if enabled)
bbcpr -s 4 file.txt server:/dest/  # No -c flag

# 4. Test with single stream
bbcpr -s 1 file.txt server:/dest/
```

## 📁 File System Issues

### "No such file or directory"

**Cause**: Source file/directory doesn't exist or path is wrong

**Solutions**:
```bash
# 1. Check source path
ls -la /path/to/source

# 2. Check destination path exists on remote
ssh user@server "ls -la /path/to/dest/"

# 3. Create destination directory
ssh user@server "mkdir -p /path/to/dest"

# 4. Use absolute paths
bbcpr /full/path/to/file.txt user@server:/full/path/dest/
```

### "Permission denied" (file access)

**Cause**: Insufficient permissions to read source or write destination

**Solutions**:
```bash
# 1. Check source file permissions
ls -la source.txt

# 2. Check destination directory permissions
ssh user@server "ls -la /path/to/dest/"

# 3. Fix permissions if you own the files
chmod 644 source.txt
ssh user@server "chmod 755 /path/to/dest/"

# 4. Check if you're the file owner
stat source.txt
ssh user@server "stat /path/to/dest/"
```

### "Disk full" or "No space left on device"

**Cause**: Insufficient disk space on destination

**Solutions**:
```bash
# 1. Check disk space on destination
ssh user@server "df -h /path/to/dest/"

# 2. Clean up space
ssh user@server "du -sh /path/to/dest/*"

# 3. Use compression to reduce size
bbcpr -c 6 large_file.txt server:/dest/

# 4. Transfer to different location
bbcpr file.txt server:/alternative/path/
```

## ⚡ Performance Issues

### "Transfer too slow"

**Cause**: Suboptimal configuration for network conditions

**Solutions**:
```bash
# 1. Increase parallel streams
bbcpr -s 16 file.txt server:/dest/

# 2. Increase buffer size
bbcpr --buffer 4MB file.txt server:/dest/

# 3. Monitor performance
bbcpr -vv -P 2 file.txt server:/dest/

# 4. Test network baseline
iperf3 -c server  # If available

# 5. Check if compression helps
bbcpr -c 4 file.txt server:/dest/
```

### "High CPU usage"

**Cause**: Too much compression or too many streams

**Solutions**:
```bash
# 1. Reduce compression level
bbcpr -c 1 file.txt server:/dest/  # Was -c 9

# 2. Reduce streams
bbcpr -s 4 file.txt server:/dest/  # Was -s 32

# 3. Disable compression for pre-compressed files
bbcpr -s 8 video.mp4 server:/dest/  # No -c flag

# 4. Monitor system resources
top -p $(pgrep bbcpr)
```

### "High memory usage"

**Cause**: Large buffers or too many streams

**Solutions**:
```bash
# 1. Reduce buffer size
bbcpr --buffer 512KB file.txt server:/dest/

# 2. Reduce streams
bbcpr -s 4 file.txt server:/dest/

# 3. Monitor memory usage
ps aux | grep bbcpr
```

## 🔍 Data Integrity Issues

### "Checksum verification failed"

**Cause**: Data corruption during transfer

**Solutions**:
```bash
# 1. Retry the transfer
bbcpr -e file.txt server:/dest/

# 2. Use fewer streams to reduce network stress
bbcpr -s 2 -e file.txt server:/dest/

# 3. Check network quality
ping -c 100 server  # Look for packet loss

# 4. Use smaller buffer size
bbcpr --buffer 256KB -e file.txt server:/dest/

# 5. Verify source file integrity
md5sum source.txt
```

### "File sizes don't match"

**Cause**: Incomplete transfer or corruption

**Solutions**:
```bash
# 1. Compare file sizes
ls -la source.txt
ssh user@server "ls -la /dest/source.txt"

# 2. Delete partial file and retry
ssh user@server "rm /dest/source.txt"
bbcpr -e source.txt server:/dest/

# 3. Check disk space during transfer
bbcpr -P 5 source.txt server:/dest/  # Monitor progress
```

## 🔧 Configuration Issues

### "Command not found"

**Cause**: bbcpr not in PATH or not installed

**Solutions**:
```bash
# 1. Check if bbcpr exists
which bbcpr
ls -la ~/.local/bin/bbcpr

# 2. Add to PATH
export PATH="$HOME/.local/bin:$PATH"
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc

# 3. Reinstall bbcpr
curl -sSL https://raw.githubusercontent.com/88plug/bbcpr/rust-rewrite/install.sh | bash

# 4. Use full path
~/.local/bin/bbcpr file.txt server:/dest/
```

### SSH configuration problems

**Cause**: SSH config issues affecting bbcpr

**Solutions**:
```bash
# 1. Test SSH configuration
ssh -F ~/.ssh/config user@server

# 2. Bypass SSH config
ssh -F /dev/null user@server

# 3. Debug SSH configuration
ssh -v user@server

# 4. Check SSH config syntax
ssh -T user@server
```

## 🐛 Debug Mode

### Enable maximum debugging
```bash
# Most verbose output
bbcpr -vvv -P 1 file.txt server:/dest/

# SSH debugging
ssh -vvv user@server echo "test"

# Network debugging
bbcpr -vvv -s 1 --timeout 300 file.txt server:/dest/
```

### Collect debug information
```bash
# System information
uname -a
bbcpr --version

# Network information
ip route get 8.8.8.8
ss -tuln | grep :22

# SSH information
ssh -V
ssh -T user@server
```

## 🔄 Recovery Procedures

### Interrupted transfer recovery
```bash
# 1. Check what was transferred
ls -la dest/
ssh user@server "ls -la /dest/"

# 2. Resume (bbcpr doesn't have built-in resume, so restart)
bbcpr -e file.txt server:/dest/  # Will overwrite

# 3. For large files, consider splitting
split -b 1G large_file.txt part_
for part in part_*; do
    bbcpr -e "$part" server:/dest/
done
```

### Cleanup after failed transfers
```bash
# Remove partial files
ssh user@server "rm /dest/partial_file.txt"

# Check disk space was freed
ssh user@server "df -h /dest/"

# Clear any temporary files
rm /tmp/bbcpr_*  # If any exist
```

## 📊 Performance Testing

### Benchmark your setup
```bash
#!/bin/bash
# Create test file
dd if=/dev/zero of=test1gb.dat bs=1M count=1024

# Test different configurations
echo "Testing 1 stream:"
time bbcpr -s 1 test1gb.dat server:/tmp/
echo "Testing 4 streams:"
time bbcpr -s 4 test1gb.dat server:/tmp/
echo "Testing 8 streams:"
time bbcpr -s 8 test1gb.dat server:/tmp/
echo "Testing 16 streams:"
time bbcpr -s 16 test1gb.dat server:/tmp/

# Cleanup
rm test1gb.dat
ssh server "rm /tmp/test1gb.dat"
```

## 📞 Getting Help

### Before asking for help, collect:
```bash
# 1. Version information
bbcpr --version

# 2. System information
uname -a
lsb_release -a  # Linux
sw_vers         # macOS

# 3. Error output
bbcpr -vvv [your command] 2>&1 | tee debug.log

# 4. SSH test
ssh -vvv user@server echo "test" 2>&1 | tee ssh_debug.log

# 5. Network test
ping -c 10 server
traceroute server
```

### Where to get help:
- **GitHub Issues**: [Report bugs](https://github.com/88plug/bbcpr/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/88plug/bbcpr/discussions)
- **Wiki**: Check other wiki pages for specific topics
- **Email**: security@88plug.com (security issues only)

### When reporting issues, include:
1. bbcpr version and platform
2. Exact command that failed
3. Complete error message
4. Network setup (local/remote/etc.)
5. Any relevant logs (use `-vvv`)

---

**Related Pages:**
- [Performance Guide](Performance-Guide) - Optimization tips
- [SSH Configuration](SSH-Configuration) - Remote access setup
- [Command Reference](Command-Reference) - All available options
- [FAQ](FAQ) - Frequently asked questions
# Performance Optimization Guide

This guide covers how to achieve maximum performance with bbcpr for different scenarios and network conditions.

## 🎯 Quick Performance Tips

### Immediate Improvements
```bash
# Use more streams for large files
bbcpr -s 16 largefile.dat server:/dest/

# Enable compression for slow networks
bbcpr -c 4 data/ server:/backup/

# Monitor performance in real-time
bbcpr -vv -P 2 files/ server:/dest/
```

## 🚀 Stream Configuration

### Understanding Streams
Parallel streams allow bbcpr to utilize full network bandwidth by transferring data simultaneously across multiple connections.

### Stream Count Guidelines

| Network Type | Recommended Streams | Use Case |
|--------------|-------------------|----------|
| **Local Network (1Gbps)** | 4-8 streams | Standard office networks |
| **High-Speed LAN (10Gbps)** | 8-16 streams | Data center networks |
| **WAN/Internet** | 2-8 streams | Remote transfers |
| **Dedicated Links (40Gbps+)** | 16-32 streams | HPC clusters |
| **Satellite/High Latency** | 16-64 streams | Compensate for latency |

### Optimal Stream Configuration
```bash
# Start with network bandwidth / 100Mbps
# For 10Gbps network: 10000/100 = 100, but diminishing returns after 32

# Test different stream counts
bbcpr -s 4 test.dat server:/tmp/    # Baseline
bbcpr -s 8 test.dat server:/tmp/    # Double streams
bbcpr -s 16 test.dat server:/tmp/   # Quad streams
bbcpr -s 32 test.dat server:/tmp/   # High performance
```

### Stream Limitations
- **CPU overhead**: More streams = more CPU usage
- **Memory usage**: Each stream uses buffer memory
- **Diminishing returns**: Beyond optimal point, performance degrades
- **Network congestion**: Too many streams can cause packet loss

## 🗜️ Compression Optimization

### When to Use Compression
- **Slow networks** (< 100Mbps)
- **CPU abundant, bandwidth limited** scenarios
- **Text/log files** (high compression ratio)
- **Uncompressed media** files

### When NOT to Use Compression
- **High-speed networks** (> 1Gbps)
- **Pre-compressed files** (videos, archives)
- **CPU-limited systems**
- **Real-time transfers** requiring minimum latency

### Compression Level Guide
```bash
# Level 1-3: Fast compression, low CPU
bbcpr -c 1 logs/ server:/backup/     # Fastest
bbcpr -c 3 docs/ server:/archive/    # Balanced

# Level 4-6: Balanced compression (recommended)
bbcpr -c 4 data/ server:/storage/    # Good balance
bbcpr -c 6 backups/ server:/vault/   # Better compression

# Level 7-9: Maximum compression, high CPU
bbcpr -c 7 archives/ server:/cold/   # High compression
bbcpr -c 9 critical/ server:/safe/   # Maximum (slow)
```

### Compression Benchmarks
| Level | Speed | Ratio | CPU Usage | Best For |
|-------|-------|-------|-----------|----------|
| 1 | Fastest | 2:1 | Low | Real-time logs |
| 3 | Fast | 3:1 | Low | General use |
| 4 | Good | 4:1 | Medium | **Recommended** |
| 6 | Moderate | 5:1 | Medium | Important data |
| 9 | Slow | 7:1 | High | Critical archives |

## 🌐 Network Optimization

### Buffer Size Tuning
```bash
# Default buffer (1MB) - good for most cases
bbcpr file.dat server:/dest/

# Large buffers for high-bandwidth networks
bbcpr --buffer 4MB -s 16 file.dat server:/dest/
bbcpr --buffer 8MB -s 32 file.dat server:/dest/

# Small buffers for low-memory systems
bbcpr --buffer 256KB -s 4 file.dat server:/dest/
```

### Network-Specific Optimizations

#### High-Bandwidth, Low-Latency (Data Center)
```bash
bbcpr -s 16 --buffer 4MB data/ server:/storage/
```

#### High-Bandwidth, High-Latency (WAN)
```bash
bbcpr -s 32 --buffer 8MB --timeout 60 data/ remote:/backup/
```

#### Low-Bandwidth (Internet)
```bash
bbcpr -s 4 -c 4 --buffer 1MB data/ server:/backup/
```

#### Unreliable Networks
```bash
bbcpr -s 8 --timeout 30 --buffer 512KB data/ unstable:/dest/
```

### TCP Tuning (Advanced)
For maximum performance on Linux systems:

```bash
# Increase TCP buffer sizes (requires root)
echo 'net.core.rmem_max = 134217728' >> /etc/sysctl.conf
echo 'net.core.wmem_max = 134217728' >> /etc/sysctl.conf
echo 'net.ipv4.tcp_rmem = 4096 87380 134217728' >> /etc/sysctl.conf
echo 'net.ipv4.tcp_wmem = 4096 65536 134217728' >> /etc/sysctl.conf
sysctl -p
```

## 📊 Performance Monitoring

### Real-Time Monitoring
```bash
# Basic progress every 5 seconds
bbcpr -P 5 file.dat server:/dest/

# Detailed statistics every 2 seconds
bbcpr -vv -P 2 file.dat server:/dest/

# Verbose with network details
bbcpr -vvv -P 1 file.dat server:/dest/
```

### Performance Metrics
Monitor these key metrics:
- **Throughput**: MB/s transfer rate
- **Efficiency**: % of theoretical bandwidth used
- **CPU usage**: System load during transfer
- **Memory usage**: RAM consumption
- **Network utilization**: Interface statistics

### Benchmarking
```bash
# Create test file
dd if=/dev/zero of=test1gb.dat bs=1M count=1024

# Benchmark different configurations
time bbcpr -s 4 test1gb.dat server:/tmp/
time bbcpr -s 8 test1gb.dat server:/tmp/
time bbcpr -s 16 test1gb.dat server:/tmp/

# Clean up
rm test1gb.dat
ssh server 'rm /tmp/test1gb.dat'
```

## 🏆 Optimization Scenarios

### Large File Transfers (>1GB)
```bash
# High-performance configuration
bbcpr -s 16 --buffer 4MB -P 5 \
  largefile.iso server:/storage/

# With compression for slower networks
bbcpr -s 8 -c 4 --buffer 2MB -P 10 \
  largefile.iso server:/backup/
```

### Many Small Files
```bash
# Archive first, then transfer
tar -czf archive.tar.gz files/
bbcpr -s 8 archive.tar.gz server:/dest/
ssh server 'cd /dest && tar -xzf archive.tar.gz'

# Or use recursive copy with compression
bbcpr -r -c 3 -s 4 files/ server:/dest/
```

### Database Backups
```bash
# Live database dump with compression
mysqldump database | bbcpr -c 6 -s 4 - server:/backups/db.sql

# Large database files
bbcpr -s 8 -e --buffer 2MB database.dump server:/secure/
```

### Media Files
```bash
# Raw video (uncompressed)
bbcpr -s 32 --buffer 8MB raw_video.mov server:/media/

# Compressed media (don't double-compress)
bbcpr -s 16 --buffer 4MB movie.mp4 server:/movies/
```

### Scientific Data
```bash
# Research datasets with verification
bbcpr -s 16 -e -P 10 --buffer 4MB \
  research_data.hdf5 cluster:/projects/

# Compressed archives
bbcpr -s 8 -c 4 simulation_results/ cluster:/archive/
```

## 🔧 System Optimization

### SSH Configuration
Optimize SSH for better performance:

```bash
# Add to ~/.ssh/config
Host fast-server
    HostName server.example.com
    User username
    Port 22
    Compression yes
    CompressionLevel 4
    TCPKeepAlive yes
    ServerAliveInterval 60
    ControlMaster auto
    ControlPath ~/.ssh/sockets/%r@%h-%p
    ControlPersist 600
```

### Environment Variables
```bash
# Set performance defaults
export BBCPR_STREAMS=16
export BBCPR_BUFFER=4MB
export BBCPR_PROGRESS=5
export BBCPR_COMPRESS=4

# Use in scripts
bbcpr large_dataset/ server:/storage/
```

### System Limits
```bash
# Increase file descriptor limits
ulimit -n 65536

# For permanent changes, edit /etc/security/limits.conf
echo "* soft nofile 65536" >> /etc/security/limits.conf
echo "* hard nofile 65536" >> /etc/security/limits.conf
```

## 📈 Performance Testing

### Automated Benchmarking Script
```bash
#!/bin/bash
# performance_test.sh

SERVER="testserver:/tmp/"
TESTFILE="benchmark.dat"

# Create 1GB test file
echo "Creating test file..."
dd if=/dev/zero of=$TESTFILE bs=1M count=1024

echo "Testing different stream configurations..."
for streams in 1 2 4 8 16 32; do
    echo "Testing $streams streams..."
    time bbcpr -s $streams $TESTFILE $SERVER
    sleep 2
done

# Clean up
rm $TESTFILE
ssh testserver "rm /tmp/$TESTFILE"
```

### Network Speed Test
```bash
# Test network baseline
iperf3 -c server.example.com -t 30

# Test bbcpr performance
bbcpr -s 16 -vv -P 1 testfile.dat server:/tmp/
```

## 🎯 Best Practices Summary

### General Rules
1. **Start with defaults**, then optimize
2. **Monitor performance** with `-vv -P 2`
3. **Test different stream counts** for your network
4. **Use compression wisely** based on content and network speed
5. **Consider file size** - different strategies for large vs small files

### Quick Reference
```bash
# Standard performance
bbcpr -s 8 -P 5 file.dat server:/dest/

# High performance
bbcpr -s 16 --buffer 4MB -P 2 file.dat server:/dest/

# Slow network
bbcpr -s 4 -c 4 --buffer 1MB file.dat server:/dest/

# Maximum performance (fast network)
bbcpr -s 32 --buffer 8MB -P 1 file.dat server:/dest/
```

### Troubleshooting Performance Issues
- **Slow transfers**: Increase streams, check network bandwidth
- **High CPU**: Reduce compression level or streams
- **High memory**: Reduce buffer size or streams
- **Timeouts**: Increase timeout, reduce streams
- **Stalls**: Check network stability, reduce buffer size

---

**Related Pages:**
- [Network Tuning](Network-Tuning) - Advanced network configuration
- [Benchmarks](Benchmarks) - Performance test results
- [Troubleshooting](Troubleshooting) - Solving performance issues
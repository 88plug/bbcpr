#!/bin/bash
# Quick test script to verify bbcpr installation

echo "🧪 Testing bbcpr installation..."
echo ""

# Check if bbcpr is installed
if ! command -v bbcpr &> /dev/null; then
    echo "❌ bbcpr not found in PATH"
    echo "   Please run: ./install.sh"
    exit 1
fi

echo "✅ bbcpr found at: $(which bbcpr)"
echo "✅ Version: $(bbcpr --version | head -1)"
echo ""

# Create test file
TEST_FILE="/tmp/bbcpr_test_$$.txt"
TEST_DEST="/tmp/bbcpr_test_copy_$$.txt"

echo "Creating test file..."
echo "Hello from bbcpr! Testing transfer at $(date)" > "$TEST_FILE"
echo "This is a test of the bbcpr file transfer utility." >> "$TEST_FILE"
echo "If you can read this, the transfer worked!" >> "$TEST_FILE"

# Test local copy
echo "Testing local file copy..."
if bbcpr "$TEST_FILE" "$TEST_DEST"; then
    echo "✅ Local copy successful"
    
    # Verify content
    if diff -q "$TEST_FILE" "$TEST_DEST" > /dev/null; then
        echo "✅ File content verified"
    else
        echo "❌ File content mismatch"
    fi
else
    echo "❌ Local copy failed"
fi

# Cleanup
rm -f "$TEST_FILE" "$TEST_DEST"

echo ""
echo "🎉 Basic tests completed!"
echo ""
echo "Next steps:"
echo "  - Try copying to a remote host: bbcpr file.txt user@host:/path/"
echo "  - Use multiple streams: bbcpr -s 8 largefile.zip /backup/"
echo "  - Enable progress: bbcpr -P 5 bigfile.iso /data/"
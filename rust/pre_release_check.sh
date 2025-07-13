#!/bin/bash
# Pre-release verification script

echo "=== bbcpr Pre-Release Verification ==="
echo "Checking project structure and files..."

# Check essential files exist
FILES_TO_CHECK=(
    "Cargo.toml"
    "src/main.rs" 
    "src/lib.rs"
    "src/cli.rs"
    "src/error.rs"
    "src/checksum/mod.rs"
    "src/network/mod.rs" 
    "src/transfer/mod.rs"
    "src/platform/mod.rs"
    "../.github/workflows/rust-ci.yml"
)

echo "Checking for required files..."
for file in "${FILES_TO_CHECK[@]}"; do
    if [[ -f "$file" ]]; then
        echo "✅ $file exists"
    else
        echo "❌ $file missing"
        exit 1
    fi
done

echo ""
echo "Checking Cargo.toml configuration..."
if grep -q 'name = "bbcpr"' Cargo.toml; then
    echo "✅ Project name correctly set to bbcpr"
else
    echo "❌ Project name not set correctly"
    exit 1
fi

if grep -q 'version = "0.1.0"' Cargo.toml; then
    echo "✅ Version set to 0.1.0"
else
    echo "❌ Version not set to 0.1.0"
    exit 1
fi

echo ""
echo "Checking source files structure..."
RUST_FILES=$(find src -name "*.rs" | wc -l)
echo "✅ Found $RUST_FILES Rust source files"

echo ""
echo "Checking GitHub Actions workflow..."
if grep -q "cargo build --release" ../.github/workflows/rust-ci.yml; then
    echo "✅ Release build configured in CI/CD"
else
    echo "❌ Release build not found in CI/CD"
    exit 1
fi

echo ""
echo "=== Pre-Release Check PASSED ==="
echo "Project is ready for release build!"
echo ""
echo "To create release:"
echo "1. git tag v0.1.0"
echo "2. git push origin v0.1.0"
echo "3. GitHub Actions will automatically build and create release"
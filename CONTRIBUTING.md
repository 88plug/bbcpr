# Contributing to bbcpr

Thank you for your interest in contributing to bbcpr! This document provides guidelines and information for contributors.

## 🚀 Quick Start for Contributors

1. **Fork** the repository on GitHub
2. **Clone** your fork locally
3. **Create** a feature branch
4. **Make** your changes
5. **Test** your changes thoroughly
6. **Submit** a pull request

## 📋 Development Setup

### Prerequisites
- **Rust 1.70+** ([Install Rust](https://rustup.rs/))
- **Git** for version control
- **OpenSSL development libraries**

### Local Development Environment
```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/bbcpr.git
cd bbcpr

# Set up upstream remote
git remote add upstream https://github.com/88plug/bbcpr.git

# Install development dependencies
cd rust
cargo build

# Run tests
cargo test

# Install development tools
rustup component add rustfmt clippy
```

## 🔧 Development Workflow

### 1. Create Feature Branch
```bash
# Update main branch
git checkout master
git pull upstream master

# Create feature branch
git checkout -b feature/your-feature-name
```

### 2. Make Changes
- Write clean, well-documented code
- Follow Rust naming conventions
- Add tests for new functionality
- Update documentation as needed

### 3. Code Quality Checks
```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy -- -D warnings

# Run tests
cargo test

# Check documentation
cargo doc --no-deps --open
```

### 4. Commit Guidelines
Use conventional commit messages:
```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (no logic changes)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(transfer): add progress callback support

Add callback mechanism for real-time progress updates
to enable better integration with GUI applications.

Closes #123
```

### 5. Submit Pull Request
```bash
# Push feature branch
git push origin feature/your-feature-name

# Create pull request on GitHub
# Fill out the pull request template
```

## 🧪 Testing

### Running Tests
```bash
# All tests
cargo test

# Specific test module
cargo test test_transfer

# Integration tests
cargo test --test integration

# With output
cargo test -- --nocapture
```

### Test Categories
- **Unit tests**: Test individual functions/modules
- **Integration tests**: Test component interactions
- **Performance tests**: Benchmark critical paths
- **CLI tests**: Test command-line interface

### Writing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected);
    }
    
    #[tokio::test]
    async fn test_async_feature() {
        // Test async functionality
    }
}
```

## 📝 Documentation

### Code Documentation
- Document all public APIs with `///` comments
- Include examples in documentation
- Document complex algorithms and data structures

```rust
/// Transfers files using parallel streams.
///
/// # Arguments
/// * `source` - Source file path
/// * `destination` - Destination file path
/// * `streams` - Number of parallel streams to use
///
/// # Examples
/// ```
/// let result = transfer_file("src.txt", "dst.txt", 4).await?;
/// ```
pub async fn transfer_file(source: &str, destination: &str, streams: usize) -> Result<()> {
    // Implementation
}
```

### Wiki Documentation
When adding features that affect user experience:
1. Update relevant wiki pages
2. Add examples to documentation
3. Update command reference if needed

### README Updates
For significant changes:
- Update feature lists
- Add new examples
- Update installation instructions if needed

## 🐛 Bug Reports

### Before Reporting
1. Search existing issues
2. Test with latest version
3. Reproduce with minimal example

### Bug Report Template
```markdown
**Environment:**
- bbcpr version: 
- Operating system: 
- Rust version: 

**Description:**
Brief description of the bug.

**Steps to Reproduce:**
1. 
2. 
3. 

**Expected Behavior:**
What should happen.

**Actual Behavior:**
What actually happens.

**Additional Context:**
Any other relevant information.
```

## ✨ Feature Requests

### Before Requesting
1. Check if feature already exists
2. Search existing feature requests
3. Consider if it fits bbcpr's scope

### Feature Request Template
```markdown
**Is your feature request related to a problem?**
Clear description of the problem.

**Describe the solution you'd like**
Clear description of what you want to happen.

**Describe alternatives you've considered**
Alternative solutions or features.

**Additional context**
Any other context about the feature request.
```

## 🔒 Security Issues

**Do NOT file public issues for security vulnerabilities.**

Instead:
- Email: security@88plug.com
- Provide detailed description
- Include reproduction steps
- Allow time for fix before disclosure

## 📜 Code Style

### Rust Style Guidelines
- Follow standard Rust naming conventions
- Use `cargo fmt` for formatting
- Address all `cargo clippy` warnings
- Prefer explicit types when it improves clarity

### Error Handling
```rust
use anyhow::{Context, Result};

fn function() -> Result<String> {
    let data = read_file("config.txt")
        .context("Failed to read configuration file")?;
    
    Ok(data)
}
```

### Async Code
```rust
use tokio;

async fn async_function() -> Result<()> {
    // Use tokio for async operations
    let result = tokio::fs::read("file.txt").await?;
    Ok(())
}
```

## 🏗️ Project Structure

```
bbcpr/
├── rust/                   # Rust implementation
│   ├── src/
│   │   ├── main.rs        # CLI entry point
│   │   ├── lib.rs         # Library root
│   │   ├── cli.rs         # Command-line interface
│   │   ├── transfer/      # Transfer engine
│   │   ├── network/       # Network protocols
│   │   ├── checksum/      # Checksum algorithms
│   │   └── platform/      # Platform-specific code
│   ├── tests/             # Integration tests
│   └── benches/           # Benchmarks
├── packaging/             # Package manager configurations
├── wiki/                  # Documentation
├── .github/               # GitHub workflows and templates
└── README.md
```

## 🎯 Areas for Contribution

### High Priority
- **Performance optimizations**
- **Platform-specific improvements**
- **Error handling improvements**
- **Documentation enhancements**

### Medium Priority
- **Additional checksum algorithms**
- **Configuration file support**
- **Logging improvements**
- **Test coverage expansion**

### Future Features
- **GUI interface**
- **Plugin system**
- **Cloud storage integration**
- **Resume capability**

## 📊 Performance Considerations

### Benchmarking
```bash
# Create benchmark
cd rust
cargo bench

# Profile performance
cargo build --release
perf record target/release/bbcpr large_file.dat server:/dest/
perf report
```

### Memory Usage
- Use `cargo instruments` on macOS
- Use `valgrind` on Linux
- Profile memory allocations
- Optimize hot paths

## 🤝 Community Guidelines

### Be Respectful
- Treat all contributors with respect
- Be constructive in feedback
- Help newcomers learn

### Communication
- Use clear, concise language
- Provide context for changes
- Ask questions when unsure

### Collaboration
- Review others' pull requests
- Share knowledge and experience
- Celebrate contributions

## 📅 Release Process

### Version Numbering
We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist
1. Update version numbers
2. Update CHANGELOG.md
3. Run full test suite
4. Update documentation
5. Create release notes
6. Tag release
7. Build and publish binaries

## 📞 Getting Help

### Development Questions
- **GitHub Discussions**: [Ask questions](https://github.com/88plug/bbcpr/discussions)
- **Issue Comments**: Comment on relevant issues
- **Wiki**: Check existing documentation

### Code Review
- All code changes require review
- Be patient with the review process
- Address feedback constructively
- Learn from suggestions

---

## 🎉 Recognition

Contributors are recognized in:
- **README.md**: Major contributors
- **Release notes**: Feature contributors
- **GitHub**: All contributors visible in insights

Thank you for contributing to bbcpr! Your efforts help make file transfers faster, safer, and more reliable for everyone.

---

**Quick Links:**
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Development Setup](#development-setup)
- [Testing Guidelines](#testing)
- [Issue Templates](.github/ISSUE_TEMPLATE/)
- [Pull Request Template](.github/PULL_REQUEST_TEMPLATE.md)
# VoxAI Development Scripts

This directory contains scripts to help set up the development environment and validate code quality.

## Scripts

### `setup-dev.sh` - Development Environment Setup

Automatically installs all required dependencies and tools for VoxAI development.

**What it does:**
- Detects your operating system (Linux, macOS, Windows)
- Installs Rust toolchain (if not already installed)
- Installs system dependencies (GTK, ALSA, etc.)
- Installs GitHub CLI for CI integration
- Verifies that the project compiles
- Runs initial code quality checks

**Usage:**

```bash
# Make executable (first time only)
chmod +x scripts/setup-dev.sh

# Run the setup
./scripts/setup-dev.sh
```

**Supported platforms:**
- **Linux**: Ubuntu/Debian, Fedora/RHEL/CentOS, Arch/Manjaro
- **macOS**: Intel and Apple Silicon
- **Windows**: Manual setup guidance provided

**After setup, you'll need to:**
1. Authenticate with GitHub CLI: `gh auth login`
2. Run tests: `cargo test`
3. Validate CI checks: `./scripts/check-ci.sh`

---

### `check-ci.sh` - Local CI Validation

Runs the same checks as the GitHub Actions CI pipeline locally, so you can catch issues before pushing.

**What it checks:**
1. **Code formatting** - Ensures code follows Rust style guidelines
2. **Clippy linter** - Catches common mistakes and suggests improvements
3. **Build** - Verifies the project compiles successfully
4. **Tests** - Runs the entire test suite

**Usage:**

```bash
# Make executable (first time only)
chmod +x scripts/check-ci.sh

# Run all CI checks
./scripts/check-ci.sh

# Run specific checks
./scripts/check-ci.sh --format-only
./scripts/check-ci.sh --clippy-only
./scripts/check-ci.sh --build-only
./scripts/check-ci.sh --test-only

# Build with specific features
./scripts/check-ci.sh --feature cuda
./scripts/check-ci.sh --feature metal
./scripts/check-ci.sh --feature opencl

# Show help
./scripts/check-ci.sh --help
```

**When to use:**
- Before committing code
- Before creating a pull request
- After making significant changes
- When debugging CI failures

**Exit codes:**
- `0` - All checks passed
- `1` - One or more checks failed

---

## Recommended Workflow

### Initial Setup (One-time)

1. **Run the setup script:**
   ```bash
   ./scripts/setup-dev.sh
   ```

2. **Authenticate with GitHub:**
   ```bash
   gh auth login
   ```

3. **Verify everything works:**
   ```bash
   ./scripts/check-ci.sh
   ```

### Daily Development

1. **Before starting work:**
   ```bash
   git pull origin main
   cargo build
   ```

2. **While developing:**
   ```bash
   # Format your code automatically
   cargo fmt --all

   # Check for warnings
   cargo clippy --all-targets --all-features
   ```

3. **Before committing:**
   ```bash
   # Run all CI checks locally
   ./scripts/check-ci.sh

   # If all checks pass, commit and push
   git add .
   git commit -m "your commit message"
   git push
   ```

4. **View CI results:**
   ```bash
   # List recent CI runs
   gh run list

   # View logs of the latest run
   gh run view --log

   # Watch a run in real-time
   gh run watch
   ```

---

## Troubleshooting

### Setup Script Issues

**Problem:** `sudo: command not found` or permission denied
- **Solution:** You may need to run as administrator or contact your system administrator

**Problem:** Unsupported Linux distribution
- **Solution:** Manually install the dependencies listed in `.github/workflows/ci.yml`

**Problem:** Compilation fails during setup
- **Solution:** Check the error messages and ensure all system dependencies are installed

### Check Script Issues

**Problem:** `cargo: command not found`
- **Solution:** Run `source $HOME/.cargo/env` or restart your terminal

**Problem:** Formatting check fails
- **Solution:** Run `cargo fmt --all` to auto-format your code

**Problem:** Clippy warnings
- **Solution:** Fix the warnings manually (clippy usually provides suggestions)

**Problem:** Build fails
- **Solution:** Check the error messages and ensure all dependencies are up to date

### GitHub CLI Issues

**Problem:** `gh: command not found`
- **Solution:** Install manually: https://cli.github.com/manual/installation

**Problem:** `gh auth status` shows "not authenticated"
- **Solution:** Run `gh auth login` and follow the prompts

**Problem:** Cannot view CI logs
- **Solution:** Ensure you're authenticated and have access to the repository

---

## Additional Tools

### Useful Cargo Commands

```bash
# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated

# Security audit
cargo audit

# Generate documentation
cargo doc --open

# Run benchmarks
cargo bench

# Clean build artifacts
cargo clean
```

### Useful GitHub CLI Commands

```bash
# View CI runs for current branch
gh run list --branch $(git branch --show-current)

# View specific run logs
gh run view <run-id> --log

# Re-run a failed job
gh run rerun <run-id>

# Create a pull request
gh pr create

# View pull request status
gh pr status
```

---

## Contributing

When adding new scripts:

1. Make them executable: `chmod +x scripts/your-script.sh`
2. Add proper error handling (`set -e`, check return codes)
3. Use colored output for better UX
4. Document the script in this README
5. Test on multiple platforms if possible

---

## See Also

- [CLAUDE.md](../CLAUDE.md) - AI assistant guidelines for this project
- [.github/workflows/](../.github/workflows/) - GitHub Actions CI/CD configuration
- [Cargo.toml](../Cargo.toml) - Project dependencies and configuration

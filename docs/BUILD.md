# Building VoxAI

This document describes how to build VoxAI from source on different platforms.

## Prerequisites

### All Platforms

- **Rust**: 1.70 or later (install from [rustup.rs](https://rustup.rs))
- **CMake**: 3.15 or later (for building Whisper.cpp)
- **Git**: For cloning the repository

### Windows

- **Visual Studio Build Tools 2019 or later** OR **Windows SDK**
  - Install from: https://visualstudio.microsoft.com/downloads/
  - Select "Desktop development with C++" workload
- **CMake**: Download from https://cmake.org/download/

### macOS

- **Xcode Command Line Tools**:
  ```bash
  xcode-select --install
  ```
- **CMake** (via Homebrew):
  ```bash
  brew install cmake
  ```

### Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libasound2-dev
```

### Linux (Fedora/RHEL)

```bash
sudo dnf install -y \
    gcc \
    gcc-c++ \
    cmake \
    pkg-config \
    alsa-lib-devel
```

## Building

### Quick Build (Current Platform)

```bash
# Clone the repository
git clone https://github.com/theo-stocchetti/voxai.git
cd voxai

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Run the application
cargo run --release
```

### Platform-Specific Builds

#### Windows

```bash
# Using the build script
./scripts/build-windows.sh

# Or manually
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

Output: `target/x86_64-pc-windows-msvc/release/voxai.exe`

#### macOS (Intel)

```bash
# Using the build script
./scripts/build-macos.sh

# Or manually
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

Output: `target/x86_64-apple-darwin/release/voxai`

#### macOS (Apple Silicon)

```bash
# Using the build script (auto-detects architecture)
./scripts/build-macos.sh

# Or manually
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

Output: `target/aarch64-apple-darwin/release/voxai`

#### macOS Universal Binary

```bash
# Build for both Intel and Apple Silicon
./scripts/build-macos.sh --universal
```

Output: `target/universal/release/voxai`

#### Linux

```bash
# Using the build script
./scripts/build-linux.sh

# Or manually
cargo build --release --target x86_64-unknown-linux-gnu
```

Output: `target/x86_64-unknown-linux-gnu/release/voxai`

### Cross-Compilation

You can use the `cross` tool for cross-compilation:

```bash
# Install cross
cargo install cross

# Build for Windows from Linux/macOS
cross build --release --target x86_64-pc-windows-msvc

# Build for Linux from Windows/macOS
cross build --release --target x86_64-unknown-linux-gnu
```

## Cargo Aliases

The following aliases are defined in `.cargo/config.toml`:

```bash
cargo build-win         # Build for Windows
cargo build-mac-intel   # Build for macOS Intel
cargo build-mac-arm     # Build for macOS ARM
cargo build-linux       # Build for Linux
```

## Build Profiles

### Debug Profile (default)

- No optimizations
- Debug symbols included
- Faster compilation
- Larger binary size
- Use for development

```bash
cargo build
```

### Release Profile

- Maximum optimizations (opt-level = 3)
- Link-time optimization (LTO)
- Debug symbols stripped
- Smaller binary size
- Use for distribution

```bash
cargo build --release
```

## Troubleshooting

### Windows

**Error**: "link.exe not found"
- **Solution**: Install Visual Studio Build Tools with C++ workload

**Error**: "CMake not found"
- **Solution**: Install CMake and add it to PATH

### macOS

**Error**: "xcrun: error: invalid active developer path"
- **Solution**: Install Xcode Command Line Tools:
  ```bash
  xcode-select --install
  ```

**Error**: "ld: framework not found Cocoa"
- **Solution**: Update Xcode Command Line Tools:
  ```bash
  sudo rm -rf /Library/Developer/CommandLineTools
  xcode-select --install
  ```

### Linux

**Error**: "Could not find ALSA development files"
- **Solution**: Install ALSA development package:
  ```bash
  sudo apt-get install libasound2-dev
  ```

**Error**: "pkg-config not found"
- **Solution**: Install pkg-config:
  ```bash
  sudo apt-get install pkg-config
  ```

## Binary Size Optimization

To further reduce binary size:

```bash
# Strip symbols manually
strip target/release/voxai

# Or use cargo-strip
cargo install cargo-strip
cargo strip --release

# Use UPX compression (optional)
upx --best --lzma target/release/voxai
```

## Development Builds

For faster iteration during development:

```bash
# Build with debug symbols but some optimizations
cargo build --profile dev-opt

# Watch for changes and rebuild automatically
cargo install cargo-watch
cargo watch -x build
```

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run tests for specific target
cargo test --target x86_64-unknown-linux-gnu
```

## Benchmarks

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench benchmark_name
```

## Documentation

```bash
# Build documentation
cargo doc

# Build and open documentation
cargo doc --open

# Build documentation for all dependencies
cargo doc --no-deps
```

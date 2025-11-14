# VoxAI Developer Documentation

**Last Updated**: 2025-11-14
**Version**: 0.1.0

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Architecture Overview](#architecture-overview)
3. [Build Instructions](#build-instructions)
4. [Development Workflow](#development-workflow)
5. [Testing Guide](#testing-guide)
6. [Contributing](#contributing)
7. [Troubleshooting](#troubleshooting)

---

## Getting Started

### Prerequisites

- **Rust**: 1.75 or later
- **CMake**: For Whisper.cpp compilation
- **Platform-specific tools**:
  - Windows: Visual Studio Build Tools
  - macOS: Xcode Command Line Tools
  - Linux: build-essential, pkg-config, libasound2-dev

### Quick Start

```bash
# Clone the repository
git clone https://github.com/theo-stocchetti/voxai
cd voxai

# Build the project
cargo build

# Run tests
cargo test

# Run the application
cargo run
```

---

## Architecture Overview

VoxAI follows a modular architecture:

```
┌─────────────────────────────────────────────┐
│              Main Application               │
├─────────────────────────────────────────────┤
│  Config  │  UI  │ Audio │ Transcription │  Output │
├──────────┴──────┴───────┴───────────────┴─────────┤
│               Platform Layer                │
│  (Windows / macOS / Linux specific code)    │
└─────────────────────────────────────────────┘
```

### Core Modules

#### 1. Configuration (`src/config/`)
- **Purpose**: Manage persistent configuration
- **Key Files**:
  - `mod.rs`: Load/save configuration
  - `schema.rs`: Config data structures
  - `language.rs`: Multi-language support

#### 2. Audio (`src/audio/`)
- **Purpose**: Audio capture and preprocessing
- **Components**:
  - Device enumeration
  - Real-time capture (CPAL)
  - Noise reduction (RNNoise)
  - Voice Activity Detection (VAD)

#### 3. Transcription (`src/transcription/`)
- **Purpose**: Speech-to-text conversion
- **Components**:
  - Whisper model management
  - Real-time pipeline
  - Chunking and buffering

#### 4. UI (`src/ui/`)
- **Purpose**: User interface
- **Components**:
  - System tray (platform-specific)
  - Settings window (egui)
  - Visual feedback

#### 5. Hotkeys (`src/hotkeys/`)
- **Purpose**: Global keyboard shortcuts
- **Platform-specific**: Windows, macOS, Linux implementations

#### 6. Output (`src/output/`)
- **Purpose**: Text injection into active applications
- **Components**:
  - Clipboard management
  - Text injection (platform-specific)
  - Post-processing

---

## Build Instructions

### Development Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Platform-Specific Builds

**Windows**:
```bash
./scripts/build-windows.sh
```

**macOS**:
```bash
./scripts/build-macos.sh
```

**Linux**:
```bash
./scripts/build-linux.sh
```

### Cross-Compilation

```bash
# Install cross
cargo install cross

# Build for Windows from Linux
cross build --target x86_64-pc-windows-msvc

# Build for macOS from Linux
cross build --target x86_64-apple-darwin
```

---

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/my-feature
```

### 2. Make Changes

Follow the code conventions in `CLAUDE.md`.

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### 4. Format Code

```bash
cargo fmt
```

### 5. Run Linter

```bash
cargo clippy
```

### 6. Commit Changes

```bash
git add .
git commit -m "feat: add my feature"
```

### 7. Push and Create PR

```bash
git push -u origin feature/my-feature
# Create PR on GitHub
```

---

## Testing Guide

### Unit Tests

Located alongside source code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Tests

Located in `tests/`:

```rust
// tests/my_test.rs
use voxai::config::Config;

#[test]
fn test_integration() {
    let config = Config::default();
    assert_eq!(config.version, "1.0.0");
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test file
cargo test --test config_tests

# With coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

---

## Contributing

See `CLAUDE.md` for detailed contribution guidelines.

### Code Style

- Follow Rust conventions
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Write tests for new features
- Document public APIs

### Pull Request Process

1. Create feature branch
2. Make changes
3. Run tests
4. Create PR with description
5. Wait for review
6. Address feedback
7. Merge when approved

---

## Troubleshooting

### Build Errors

**CMake not found**:
```bash
# Ubuntu/Debian
sudo apt-get install cmake

# macOS
brew install cmake

# Windows
# Install CMake from https://cmake.org/download/
```

**Whisper.cpp compilation fails**:
- Ensure CMake is installed
- Check that C++ compiler is available
- See `docs/BUILD.md` for platform-specific instructions

### Runtime Errors

**Config file not found**:
- Config is created automatically on first run
- Check permissions on config directory

**Audio device not found**:
- Check audio device is connected
- Verify permissions (may require sudo on Linux)

---

## API Documentation

Generate and view API documentation:

```bash
cargo doc --no-deps --open
```

---

## Performance Profiling

### CPU Profiling

```bash
cargo install flamegraph
cargo flamegraph
```

### Memory Profiling

```bash
cargo install heaptrack
heaptrack ./target/release/voxai
```

### Benchmarks

```bash
cargo bench
```

---

## FAQ

### How do I add a new Whisper model?

Edit `src/transcription/models.rs` and add the model metadata.

### How do I add support for a new language?

Edit `src/config/language.rs` and add the language to the `Language` enum.

### How do I debug platform-specific code?

Use conditional compilation:
```rust
#[cfg(target_os = "windows")]
fn windows_specific() { }

#[cfg(target_os = "macos")]
fn macos_specific() { }

#[cfg(target_os = "linux")]
fn linux_specific() { }
```

---

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Whisper.cpp](https://github.com/ggerganov/whisper.cpp)
- [CPAL Documentation](https://docs.rs/cpal/)
- [egui Documentation](https://docs.rs/egui/)

---

**Happy Coding! 🚀**

# VoxAI System Dependencies

This document lists all system dependencies required to build and run VoxAI on different platforms.

---

## Linux

### Ubuntu / Debian

```bash
# Core development tools
sudo apt update
sudo apt install -y build-essential pkg-config

# GTK3 (for system tray)
sudo apt install -y libgtk-3-dev libgdk-pixbuf2.0-dev libpango1.0-dev libcairo2-dev

# X11 (for hotkeys and text injection)
sudo apt install -y libx11-dev libxtst-dev libxinerama-dev libxkbcommon-dev

# Audio (CPAL)
sudo apt install -y libasound2-dev

# Optional: Wayland support
sudo apt install -y libwayland-dev
```

### Fedora / RHEL / CentOS

```bash
# Core development tools
sudo dnf groupinstall "Development Tools"
sudo dnf install pkg-config

# GTK3
sudo dnf install gtk3-devel gdk-pixbuf2-devel pango-devel cairo-devel

# X11
sudo dnf install libX11-devel libXtst-devel libXinerama-devel libxkbcommon-devel

# Audio
sudo dnf install alsa-lib-devel

# Optional: Wayland
sudo dnf install wayland-devel
```

### Arch Linux

```bash
# Core development tools
sudo pacman -S base-devel

# GTK3
sudo pacman -S gtk3 gdk-pixbuf2 pango cairo

# X11
sudo pacman -S libx11 libxtst libxinerama libxkbcommon

# Audio
sudo pacman -S alsa-lib

# Optional: Wayland
sudo pacman -S wayland
```

---

## macOS

### Xcode Command Line Tools

```bash
xcode-select --install
```

### Homebrew (for optional dependencies)

```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Optional: PortAudio for advanced audio features
brew install portaudio
```

### Additional Requirements

- macOS 10.15 (Catalina) or later
- Xcode 12.0 or later (for building)
- **Permissions**: You'll need to grant the following permissions when running:
  - Accessibility (for text injection)
  - Input Monitoring (for global hotkeys)
  - Microphone (for audio capture)

---

## Windows

### Visual Studio Build Tools

Download and install from: https://visualstudio.microsoft.com/visual-cpp-build-tools/

Required components:
- Windows SDK (10.0.19041.0 or later)
- MSVC v142 - VS 2019 C++ x64/x86 build tools
- C++ CMake tools for Windows

### Rust Target

```powershell
rustup target add x86_64-pc-windows-msvc
```

### Optional: CMake

For Whisper.cpp integration:
- Download from https://cmake.org/download/
- Add to PATH

---

## Rust Toolchain

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Required Rust Version

- Minimum: 1.75.0
- Recommended: Latest stable

### Useful Tools

```bash
# Code formatting
rustup component add rustfmt

# Linting
rustup component add clippy

# Cross-compilation (optional)
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

---

## Optional Dependencies

### For Icon Generation

**Inkscape** (SVG to PNG conversion):
```bash
# Ubuntu/Debian
sudo apt install inkscape

# macOS
brew install inkscape

# Windows
# Download from https://inkscape.org/
```

**ImageMagick** (Image manipulation):
```bash
# Ubuntu/Debian
sudo apt install imagemagick

# macOS
brew install imagemagick

# Windows
# Download from https://imagemagick.org/
```

---

## Troubleshooting

### Linux: "pkg-config not found"

```bash
sudo apt install pkg-config
```

### Linux: "gdk-pixbuf-2.0 not found"

```bash
sudo apt install libgdk-pixbuf2.0-dev
```

### macOS: "xcrun: error: unable to find utility"

```bash
xcode-select --install
```

### Windows: "link.exe not found"

Install Visual Studio Build Tools (see above) or full Visual Studio.

---

## Verification

After installing dependencies, verify your setup:

```bash
# Check Rust installation
rustc --version
cargo --version

# Check pkg-config (Linux only)
pkg-config --version

# Test build
cd voxai
cargo check
```

---

## Docker Alternative (Linux Development)

If you don't want to install system dependencies, you can use Docker:

```bash
# Build Docker image (TODO: Create Dockerfile)
docker build -t voxai-dev .

# Run development environment
docker run -it --rm -v $(pwd):/workspace voxai-dev
```

---

## CI/CD

See `.github/workflows/ci.yml` for automated build configuration on GitHub Actions.

---

**Last Updated**: 2025-11-14
**Maintained by**: Team 3 - UI/Platform

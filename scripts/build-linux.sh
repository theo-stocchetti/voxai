#!/bin/bash
# Build script for Linux (x86_64)

set -e

echo "==================================="
echo "Building VoxAI for Linux x64"
echo "==================================="

# Check prerequisites
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed"
    exit 1
fi

# Check for required system dependencies
echo "Checking system dependencies..."

MISSING_DEPS=()

if ! pkg-config --exists alsa; then
    MISSING_DEPS+=("libasound2-dev")
fi

if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
    echo "Warning: Missing system dependencies:"
    for dep in "${MISSING_DEPS[@]}"; do
        echo "  - $dep"
    done
    echo ""
    echo "Install them with:"
    echo "  sudo apt-get install ${MISSING_DEPS[*]}"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Add target if not already added
rustup target add x86_64-unknown-linux-gnu 2>/dev/null || true

# Build for Linux
echo "Building release binary..."
cargo build --release --target x86_64-unknown-linux-gnu

echo ""
echo "✓ Build completed successfully!"
echo "Binary location: target/x86_64-unknown-linux-gnu/release/voxai"

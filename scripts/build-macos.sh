#!/bin/bash
# Build script for macOS (both Intel and ARM)

set -e

echo "==================================="
echo "Building VoxAI for macOS"
echo "==================================="

# Check prerequisites
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed"
    exit 1
fi

# Detect current architecture
ARCH=$(uname -m)

if [ "$ARCH" == "arm64" ]; then
    TARGET="aarch64-apple-darwin"
    ARCH_NAME="Apple Silicon (ARM64)"
else
    TARGET="x86_64-apple-darwin"
    ARCH_NAME="Intel (x86_64)"
fi

# Add target if not already added
rustup target add "$TARGET" 2>/dev/null || true

echo "Building for $ARCH_NAME..."
cargo build --release --target "$TARGET"

echo ""
echo "✓ Build completed successfully!"
echo "Binary location: target/$TARGET/release/voxai"

# Optional: Build universal binary for both architectures
if [ "$1" == "--universal" ]; then
    echo ""
    echo "Building universal binary..."

    rustup target add x86_64-apple-darwin 2>/dev/null || true
    rustup target add aarch64-apple-darwin 2>/dev/null || true

    cargo build --release --target x86_64-apple-darwin
    cargo build --release --target aarch64-apple-darwin

    # Create universal binary with lipo
    mkdir -p target/universal/release
    lipo -create \
        target/x86_64-apple-darwin/release/voxai \
        target/aarch64-apple-darwin/release/voxai \
        -output target/universal/release/voxai

    echo "✓ Universal binary created: target/universal/release/voxai"
fi

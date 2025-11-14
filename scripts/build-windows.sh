#!/bin/bash
# Build script for Windows (x86_64)

set -e

echo "==================================="
echo "Building VoxAI for Windows x64"
echo "==================================="

# Check prerequisites
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed"
    exit 1
fi

# Add target if not already added
rustup target add x86_64-pc-windows-msvc 2>/dev/null || true

# Build for Windows
echo "Building release binary..."
cargo build --release --target x86_64-pc-windows-msvc

echo ""
echo "✓ Build completed successfully!"
echo "Binary location: target/x86_64-pc-windows-msvc/release/voxai.exe"

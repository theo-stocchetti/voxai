#!/bin/bash
#
# Generate PNG icons from SVG for Linux
# Requires: inkscape or imagemagick
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LINUX_DIR="$SCRIPT_DIR/linux"
PNG_DIR="$LINUX_DIR/png"

# Create PNG directory
mkdir -p "$PNG_DIR"

# Sizes to generate
SIZES=(16 22 24 32 48 256)

# Icons to generate
ICONS=(idle recording processing)

# Check if inkscape is available
if command -v inkscape &> /dev/null; then
    echo "Using Inkscape for PNG generation..."
    TOOL="inkscape"
elif command -v convert &> /dev/null; then
    echo "Using ImageMagick for PNG generation..."
    TOOL="convert"
else
    echo "Error: Neither Inkscape nor ImageMagick found."
    echo "Please install one of them:"
    echo "  Ubuntu/Debian: sudo apt install inkscape"
    echo "  macOS: brew install inkscape"
    exit 1
fi

# Generate PNGs
for icon in "${ICONS[@]}"; do
    echo "Generating PNG files for $icon..."

    for size in "${SIZES[@]}"; do
        input="$LINUX_DIR/${icon}.svg"
        output="$PNG_DIR/${icon}-${size}.png"

        if [ "$TOOL" = "inkscape" ]; then
            inkscape --export-type=png \
                     --export-width=$size \
                     --export-height=$size \
                     "$input" \
                     -o "$output" \
                     2>/dev/null
        else
            convert -background none \
                    "$input" \
                    -resize ${size}x${size} \
                    "$output"
        fi

        echo "  Created: ${icon}-${size}.png"
    done
done

echo ""
echo "✅ PNG generation complete!"
echo "Output directory: $PNG_DIR"
echo ""
echo "Files generated:"
ls -lh "$PNG_DIR"

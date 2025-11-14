# Windows Icons - Generation Instructions

## Status: TODO

This directory should contain Windows .ico files for VoxAI system tray.

## Required Files

- `idle.ico` - Application at rest
- `recording.ico` - Recording audio
- `processing.ico` - Processing transcription

## Generation Steps

1. **Prerequisites**: Install ImageMagick
   ```bash
   # Ubuntu/Debian
   sudo apt install imagemagick

   # macOS
   brew install imagemagick

   # Windows
   # Download from https://imagemagick.org/
   ```

2. **Generate .ico files** from the Linux SVG icons:
   ```bash
   cd assets/icons/linux

   # For idle.ico
   for size in 16 20 24 32 40 48 64 256; do
     convert -background none idle.svg -resize ${size}x${size} /tmp/idle-${size}.png
   done
   convert /tmp/idle-*.png ../windows/idle.ico
   rm /tmp/idle-*.png

   # Repeat for recording.ico and processing.ico
   ```

3. **Alternative**: Use online converter
   - Upload SVG to https://convertio.co/svg-ico/
   - Select all sizes (16, 20, 24, 32, 40, 48, 64, 256)
   - Download .ico file

4. **Test** the .ico file:
   - Right-click in Windows Explorer
   - Check "Properties" shows all embedded sizes
   - Verify it displays correctly in system tray

## Automation Script

TODO: Create a script `generate-windows-icons.sh` to automate this process.

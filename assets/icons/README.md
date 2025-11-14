# VoxAI System Tray Icons

**Created**: 2025-11-14
**Status**: Initial SVG prototypes created
**Issue**: #005-4

---

## Overview

This directory contains system tray icons for VoxAI across all supported platforms.

### Icon States

1. **Idle** (`idle.*`) - Application at rest, ready to record
2. **Recording** (`recording.*`) - Actively capturing audio
3. **Processing** (`processing.*`) - Transcribing audio with Whisper

---

## Directory Structure

```
assets/icons/
├── linux/          # Linux icons (SVG + PNG)
│   ├── idle.svg
│   ├── recording.svg
│   ├── processing.svg
│   └── png/        # TODO: Export PNG in multiple sizes
├── windows/        # Windows icons (.ico)
│   └── TODO: Generate .ico files from SVG
├── macos/          # macOS template icons
│   └── TODO: Create @1x and @2x template PNGs
└── README.md       # This file
```

---

## Current Status

### ✅ Completed
- [x] Linux SVG icons created (idle, recording, processing)
- [x] Directory structure created

### 🚧 TODO
- [ ] Export Linux PNG files (16x16, 22x22, 24x24, 32x32, 48x48, 256x256)
- [ ] Create Windows .ico files (all sizes embedded)
- [ ] Create macOS template icons (@1x and @2x)
- [ ] Test icons on all platforms
- [ ] Optimize file sizes

---

## Linux Icons (SVG)

**Format**: SVG (Scalable Vector Graphics)
**Location**: `linux/*.svg`
**Status**: ✅ Created

The SVG icons follow the symbolic icon style and are designed to:
- Scale cleanly to any size
- Work in both light and dark themes
- Be clear at small sizes (16x16, 22x22)
- Follow freedesktop.org icon theme specification

### Generating PNG from SVG

To generate PNG files at various sizes, use one of these tools:

**Using Inkscape (recommended)**:
```bash
inkscape --export-type=png --export-width=16 --export-height=16 idle.svg -o png/idle-16.png
inkscape --export-type=png --export-width=22 --export-height=22 idle.svg -o png/idle-22.png
inkscape --export-type=png --export-width=24 --export-height=24 idle.svg -o png/idle-24.png
inkscape --export-type=png --export-width=32 --export-height=32 idle.svg -o png/idle-32.png
inkscape --export-type=png --export-width=48 --export-height=48 idle.svg -o png/idle-48.png
inkscape --export-type=png --export-width=256 --export-height=256 idle.svg -o png/idle-256.png
```

**Using ImageMagick**:
```bash
convert -background none idle.svg -resize 16x16 png/idle-16.png
convert -background none idle.svg -resize 22x22 png/idle-22.png
# ... repeat for other sizes
```

**Using rsvg-convert**:
```bash
rsvg-convert -w 16 -h 16 idle.svg -o png/idle-16.png
rsvg-convert -w 22 -h 22 idle.svg -o png/idle-22.png
# ... repeat for other sizes
```

---

## Windows Icons (.ico)

**Format**: ICO (Windows Icon)
**Location**: `windows/*.ico`
**Status**: ⏳ TODO

Windows .ico files must contain multiple sizes embedded in a single file.

### Required Sizes
- 16x16
- 20x20
- 24x24
- 32x32
- 40x40
- 48x48
- 64x64
- 256x256

### Generating .ico from SVG

**Option 1: Using ImageMagick** (recommended):
```bash
# First, generate PNGs at all required sizes
for size in 16 20 24 32 40 48 64 256; do
  convert -background none idle.svg -resize ${size}x${size} idle-${size}.png
done

# Then combine into .ico
convert idle-16.png idle-20.png idle-24.png idle-32.png \
        idle-40.png idle-48.png idle-64.png idle-256.png \
        windows/idle.ico

# Repeat for recording.svg and processing.svg
```

**Option 2: Using Online Tools**:
- https://convertio.co/svg-ico/
- https://www.icoconverter.com/
- Upload SVG, select all sizes, download .ico

**Option 3: Using Rust crate** (for integration):
```rust
// Use the `ico` crate to generate .ico files from PNG
use ico::{IconDir, IconDirEntry, IconImage};
```

---

## macOS Icons (Template Icons)

**Format**: PNG @1x and @2x (or PDF vector)
**Location**: `macos/*.png` or `macos/*.pdf`
**Status**: ⏳ TODO

macOS menu bar icons should be **template icons** - monochrome images that adapt to light/dark mode.

### Requirements
- **Size**: 22x22pt (@1x), 44x44pt (@2x)
- **Color**: Black on transparent background
- **Style**: Flat, simple, minimal
- **Naming**: `idle.png`, `idle@2x.png`, or `idle.pdf`

### Generating Template Icons

**From SVG**:
1. Convert SVG to monochrome (black on transparent)
2. Export at @1x (22x22) and @2x (44x44) sizes

**Using Inkscape**:
```bash
# Export @1x
inkscape --export-type=png --export-width=22 --export-height=22 idle.svg -o macos/idle.png

# Export @2x
inkscape --export-type=png --export-width=44 --export-height=44 idle.svg -o macos/idle@2x.png
```

**Alternatively, export as PDF** (vector):
```bash
inkscape --export-type=pdf idle.svg -o macos/idle.pdf
```

### macOS Template Icon Naming Convention

In code, specify template mode:
```swift
statusItem.button?.image?.isTemplate = true
```

Or in Rust with Cocoa bindings:
```rust
icon.set_as_template(true);
```

---

## Design Specifications

### Color Palette

**Idle State**:
- Gray: `#6B6B6B` (neutral)

**Recording State**:
- Red: `#E74C3C` (active, recording)
- Accent: `#FF0000` (pulsing indicator)

**Processing State**:
- Blue: `#3498DB` (processing, thinking)

### Icon Style

- **Minimalist**: Simple microphone shape
- **Clear**: Recognizable at 16x16 pixels
- **Consistent**: All three states use same base shape
- **Symbolic**: Works in light and dark themes

### Animation

The SVG files include embedded CSS animations:
- **Recording**: Pulsing red dot
- **Processing**: Rotating spinner

For runtime animation, update icons periodically (frame-by-frame).

---

## Usage in Code

### Linux (tray-icon crate)

```rust
use tray_icon::{Icon, TrayIconBuilder};

let icon_data = include_bytes!("../assets/icons/linux/png/idle-32.png");
let icon = Icon::from_rgba(icon_data.to_vec(), 32, 32)?;

let tray = TrayIconBuilder::new()
    .with_icon(icon)
    .build()?;
```

### Windows (tray-icon crate)

```rust
let icon = Icon::from_path("assets/icons/windows/idle.ico", None)?;
let tray = TrayIconBuilder::new()
    .with_icon(icon)
    .build()?;
```

### macOS (Cocoa)

```rust
// Using raw Cocoa bindings
let image = NSImage::alloc()
    .initWithContentsOfFile(ns_string!("assets/icons/macos/idle.png"));
image.setTemplate(true); // Enable template mode

status_item.button().setImage(image);
```

---

## Testing

### Visual Testing Checklist
- [ ] Icons clear at 16x16 (smallest size)
- [ ] Icons clear at 22x22 (macOS menu bar)
- [ ] Icons scale well to 256x256
- [ ] Light theme: good contrast
- [ ] Dark theme: good contrast
- [ ] HiDPI/Retina: sharp and crisp

### Platform Testing
- [ ] Windows 10/11 system tray
- [ ] Windows light theme
- [ ] Windows dark theme
- [ ] macOS menu bar
- [ ] macOS light mode
- [ ] macOS dark mode
- [ ] Linux GNOME
- [ ] Linux KDE
- [ ] Linux XFCE

---

## Future Improvements

- [ ] Add error state icon (red X or warning)
- [ ] Add disabled state icon (grayed out)
- [ ] Optimize PNG file sizes (pngcrush, optipng)
- [ ] Create animated .ico for Windows (multi-frame)
- [ ] App icon for launcher (not just tray)
- [ ] Custom color themes

---

## Tools & Resources

### Design Tools
- **Inkscape** (free, open-source): https://inkscape.org/
- **Figma** (free for individuals): https://figma.com/
- **GIMP** (free): https://www.gimp.org/

### Icon Conversion Tools
- **ImageMagick**: https://imagemagick.org/
- **rsvg-convert** (librsvg): Part of librsvg package
- **Inkscape CLI**: Built-in with Inkscape

### Platform Guidelines
- **Windows**: https://docs.microsoft.com/en-us/windows/win32/uxguide/vis-icons
- **macOS**: https://developer.apple.com/design/human-interface-guidelines/macos/icons-and-images/system-icons/
- **Linux**: https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html

### Inspiration
- SF Symbols (macOS): `mic`, `mic.fill`, `mic.circle`
- Material Design: https://fonts.google.com/icons?icon.query=microphone
- macOS system tray apps: Look at Slack, Discord, Zoom

---

## Contributors

- Team 3 - UI/Platform
- Issue #005-4

---

## License

Part of the VoxAI project. See LICENSE file in root directory.

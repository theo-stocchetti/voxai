# macOS Icons - Generation Instructions

## Status: TODO

This directory should contain macOS template icons for VoxAI menu bar.

## Required Files

**Option 1: PNG @1x/@2x**
- `idle.png` (22x22)
- `idle@2x.png` (44x44)
- `recording.png` (22x22)
- `recording@2x.png` (44x44)
- `processing.png` (22x22)
- `processing@2x.png` (44x44)

**Option 2: PDF (vector, preferred)**
- `idle.pdf`
- `recording.pdf`
- `processing.pdf`

## Generation Steps

### Method 1: Export PNG from SVG

1. **Prerequisites**: Install Inkscape
   ```bash
   brew install inkscape
   ```

2. **Export @1x (22x22)**:
   ```bash
   cd assets/icons/linux
   inkscape --export-type=png --export-width=22 --export-height=22 \
            idle.svg -o ../macos/idle.png
   ```

3. **Export @2x (44x44)**:
   ```bash
   inkscape --export-type=png --export-width=44 --export-height=44 \
            idle.svg -o ../macos/idle@2x.png
   ```

4. **Repeat for recording and processing**

### Method 2: Export PDF (Recommended)

```bash
cd assets/icons/linux
inkscape --export-type=pdf idle.svg -o ../macos/idle.pdf
inkscape --export-type=pdf recording.svg -o ../macos/recording.pdf
inkscape --export-type=pdf processing.svg -o ../macos/processing.pdf
```

## Important: Template Mode

macOS menu bar icons should be **monochrome** (black on transparent) to work as template icons.

### Converting to Template Style

If your SVG has colors, convert to monochrome first:

1. Open in Inkscape
2. Select all elements
3. Set fill to black (#000000)
4. Set stroke to black (if needed)
5. Remove any color gradients
6. Export as described above

### In Code

Enable template mode in your Rust/Swift code:

```rust
// Using cocoa-rs or objc
icon.set_as_template(true);
```

This tells macOS to:
- Use system accent color for the icon
- Invert in dark mode automatically
- Apply highlight color when clicked

## Testing

1. Display icon in menu bar
2. Switch between light/dark mode - icon should adapt
3. Click icon - should highlight with system accent color
4. Test on Retina display - @2x should be crisp

## Automation Script

TODO: Create a script `generate-macos-icons.sh` to automate this process.

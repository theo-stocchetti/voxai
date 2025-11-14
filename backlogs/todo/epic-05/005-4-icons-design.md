# [ISSUE-005-4] System Tray Icons Design & Assets

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: S
**EPIC**: EPIC 5 - Interface System Tray

---

## Description

Design and create all icon assets for VoxAI system tray across all platforms (Windows, macOS, Linux). Icons should be clear, professional, and follow platform-specific design guidelines.

---

## Context

Each platform has different icon requirements and conventions. High-quality icons are essential for a polished, professional application. Icons must clearly communicate the application state (idle, recording, processing) at a glance.

---

## Acceptance Criteria

- [ ] Icons designed for all three platforms (Windows, macOS, Linux)
- [ ] Three states: idle, recording, processing
- [ ] Icons follow platform-specific design guidelines
- [ ] High-resolution support (HiDPI, Retina)
- [ ] Icons are clear and recognizable at small sizes (16x16)
- [ ] Color schemes work in light and dark modes
- [ ] All required file formats and sizes provided

---

## Technical Details

### Affected Components
- `assets/icons/windows/` - Windows ICO files
- `assets/icons/macos/` - macOS template icons (PDF or PNG @1x/@2x)
- `assets/icons/linux/` - Linux SVG and PNG files (multiple sizes)

### Dependencies
- None (this is an asset creation task)

### Icon Requirements by Platform

**Windows (.ico)**:
- Sizes: 16x16, 20x20, 24x24, 32x32, 40x40, 48x48, 64x64, 256x256
- Format: ICO file containing all sizes
- Color: Full color icons
- Files needed: `idle.ico`, `recording.ico`, `processing.ico`

**macOS (template icons)**:
- Size: 22x22pt (@1x), 44x44pt (@2x)
- Format: PDF (vector, preferred) or PNG @1x/@2x
- Color: Black on transparent (template mode for auto light/dark)
- Files needed: `idle.pdf`, `idle@2x.png`, `recording.pdf`, `recording@2x.png`, `processing.pdf`, `processing@2x.png`

**Linux (SVG + PNG)**:
- Sizes: 16x16, 22x22, 24x24, 32x32, 48x48, 256x256
- Format: SVG (scalable) + PNG at each size
- Color: Follow freedesktop.org symbolic icon spec
- Files needed: `idle.svg`, `idle-16.png`, `idle-22.png`, ..., `recording.svg`, `processing.svg` + all PNG sizes

---

## Design Specifications

### Icon Concept
**Primary Symbol**: Microphone

**Three States**:
1. **Idle**: Microphone icon, neutral/gray color
2. **Recording**: Microphone icon with red accent or filled
3. **Processing**: Microphone icon with spinner/loading indicator

### Design Guidelines

**Windows**:
- Full color icons
- 3D-style depth acceptable
- Use Windows accent colors
- Recording state: Red microphone
- Processing state: Blue/gray with animation possibility

**macOS**:
- Monochrome template icons
- Simple, flat design
- Minimal detail (clear at 22x22)
- Recording state: Filled microphone
- Processing state: Microphone with dots/spinner

**Linux**:
- Symbolic icon style (monochrome)
- Follow GNOME/Adwaita guidelines
- Simple shapes, clean lines
- Adapt to user's theme color

### Color Palette

**Idle State**:
- Windows: Gray (#6B6B6B)
- macOS: Black (auto-adapts)
- Linux: Symbolic (theme color)

**Recording State**:
- Windows: Red (#E74C3C)
- macOS: Black filled (auto-adapts)
- Linux: Symbolic with red hint

**Processing State**:
- Windows: Blue (#3498DB) with spinner
- macOS: Black with spinner (auto-adapts)
- Linux: Symbolic with activity indicator

---

## Tasks Breakdown

- [ ] Research platform icon guidelines (Windows, macOS, Linux)
- [ ] Sketch microphone icon concepts (3-5 variants)
- [ ] Choose final design direction
- [ ] Create master SVG vector file
- [ ] Design idle state icon
- [ ] Design recording state icon
- [ ] Design processing state icon (with animation frames)
- [ ] Export Windows ICO files (all sizes in one .ico)
- [ ] Export macOS template icons (@1x and @2x)
- [ ] Export Linux SVG files
- [ ] Export Linux PNG files (all sizes)
- [ ] Test icons at small sizes (16x16, 22x22)
- [ ] Test icons in light and dark modes
- [ ] Get design review/feedback
- [ ] Make revisions if needed
- [ ] Finalize and commit icon assets

---

## Test Plan

### Visual Testing
- [ ] View icons at 16x16 (smallest size)
- [ ] View icons at 22x22 (macOS menu bar)
- [ ] View icons at 256x256 (high resolution)
- [ ] Test in Windows light theme
- [ ] Test in Windows dark theme
- [ ] Test in macOS light mode
- [ ] Test in macOS dark mode
- [ ] Test in Linux GNOME (light and dark)
- [ ] Test on HiDPI display (200% scaling)
- [ ] Verify icons are clear and recognizable

### Integration Testing
- [ ] Load icons in Windows system tray
- [ ] Load icons in macOS menu bar
- [ ] Load icons in Linux system tray (GNOME, KDE, XFCE)
- [ ] Verify state changes work visually
- [ ] Check icon alignment and padding

---

## Documentation Updates

- [ ] Add icon design rationale to CLAUDE.md
- [ ] Document icon file locations
- [ ] Document icon naming conventions
- [ ] Add icon preview to README (optional)

---

## Related Issues

- Blocks: #005-1 (Windows System Tray - needs icons)
- Blocks: #005-2 (macOS Menu Bar - needs icons)
- Blocks: #005-3 (Linux System Tray - needs icons)
- Related to: #009-1 (Status Overlay - may use similar icons)

---

## Notes

**Design Tools**:
- Vector: Figma, Sketch, Adobe Illustrator, Inkscape (free)
- Icon export: IconJar, Image2Icon, or manual export
- ICO creation: ImageMagick, online ICO converter

**Icon Inspiration**:
- Look at popular apps: Discord, Slack, Zoom (tray icons)
- macOS SF Symbols: `mic`, `mic.fill`, `mic.circle`
- Material Design Icons: Microphone variants
- Keep it simple and recognizable

**Animation for Processing State**:
- Windows: Can use animated .ico (GIF-like frames)
- macOS: Update icon periodically (rotating spinner)
- Linux: Update icon periodically (pulse effect)

**Accessibility**:
- Ensure sufficient contrast
- Don't rely on color alone (use shape/fill differences)
- Test with color blindness simulators

**File Size Considerations**:
- Keep PNG files optimized (use pngcrush or similar)
- SVG files should be minimal (no unnecessary metadata)
- ICO files can be large (all sizes in one file) - acceptable

**Future Enhancements**:
- Additional states: Error, Warning, Disabled
- Animated recording icon (pulsing)
- Custom user themes
- App icon (for launcher, not just tray)

---

## Definition of Done

- [ ] All icon states designed (idle, recording, processing)
- [ ] Windows ICO files created and exported
- [ ] macOS template icons created and exported
- [ ] Linux SVG and PNG files created and exported
- [ ] Icons tested at all required sizes
- [ ] Icons tested in light and dark modes
- [ ] Icons reviewed and approved
- [ ] Icon assets committed to repository
- [ ] Documentation updated
- [ ] Issue moved to done folder

# [ISSUE-005-2] macOS Menu Bar Implementation

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 5 - Interface System Tray

---

## Description

Implement a native macOS menu bar icon and menu for VoxAI. This provides the primary user interface for macOS users, following macOS design conventions for menu bar applications.

---

## Context

macOS uses a menu bar at the top of the screen for system-level applications. VoxAI should integrate seamlessly with this pattern, providing a native macOS experience. The menu bar app should feel like a built-in macOS application.

---

## Acceptance Criteria

- [ ] Menu bar icon appears in macOS menu bar
- [ ] Icon displays current state (idle, recording, processing)
- [ ] Click opens context menu with options
- [ ] Menu includes Start/Stop, Settings, About, Quit
- [ ] Follows macOS Human Interface Guidelines
- [ ] Works on both Intel and Apple Silicon Macs
- [ ] High-resolution icons for Retina displays

---

## Technical Details

### Affected Components
- `src/ui/tray_macos.rs` - New macOS menu bar implementation
- `src/ui/mod.rs` - Platform-specific tray initialization
- `assets/icons/macos/` - macOS-specific icon assets

### Dependencies
- [ ] `tray-icon` crate with macOS support
- [ ] macOS 11.0+ (Big Sur or later)
- [ ] Related to: #006-2 (macOS Hotkeys)

### Implementation Notes

**macOS Menu Bar Design**:
- Use SF Symbols for native look (if possible)
- Template icons that adapt to light/dark mode
- Monochrome icons following macOS style
- 22x22pt base size (@1x), 44x44pt for Retina (@2x)

**Icon States**:
- Idle: Microphone icon (unfilled)
- Recording: Microphone icon (filled/red)
- Processing: Microphone with spinner animation

**Menu Structure**:
```
[Icon]
├── Start Recording (⌘⇧R)
├── Stop Recording
├── ───────────────
├── Settings... (⌘,)
├── ───────────────
├── About VoxAI
├── Check for Updates...
├── ───────────────
├── Quit VoxAI (⌘Q)
```

**Code Example**:
```rust
#[cfg(target_os = "macos")]
pub fn create_menu_bar() -> Result<TrayIcon> {
    use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem}};

    let icon = load_icon_macos("idle")?;

    let menu = Menu::new();
    menu.append(&MenuItem::new("Start Recording", true, Some("cmd+shift+r")))?;
    menu.append(&MenuItem::separator())?;
    menu.append(&MenuItem::new("Settings...", true, Some("cmd+,")))?;
    menu.append(&MenuItem::separator())?;
    menu.append(&MenuItem::new("Quit VoxAI", true, Some("cmd+q")))?;

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .with_tooltip("VoxAI - Voice Transcription")
        .build()?;

    Ok(tray)
}
```

---

## Tasks Breakdown

- [ ] Create `src/ui/tray_macos.rs` module
- [ ] Implement menu bar icon initialization
- [ ] Create menu structure with macOS conventions
- [ ] Design macOS-style icons (template icons)
- [ ] Export icons at @1x and @2x resolutions
- [ ] Implement icon state changes (idle/recording/processing)
- [ ] Add keyboard shortcuts to menu items (⌘⇧R, ⌘,, ⌘Q)
- [ ] Implement menu event handling
- [ ] Add tooltip "VoxAI - Voice Transcription"
- [ ] Support light/dark mode (template icons)
- [ ] Test on macOS 11, 12, 13, 14 (Sonoma)
- [ ] Test on both Intel and Apple Silicon

---

## Test Plan

### Unit Tests
- [ ] Test menu bar initialization
- [ ] Test menu item creation and event handling
- [ ] Test icon loading and state changes

### Integration Tests
- [ ] Test menu bar appears correctly
- [ ] Test menu opens on click
- [ ] Test all menu items trigger correct actions
- [ ] Test keyboard shortcuts work from menu

### Manual Testing
- [ ] Test on macOS 13 Ventura (Intel Mac)
- [ ] Test on macOS 14 Sonoma (Apple Silicon)
- [ ] Verify icon appears in correct position
- [ ] Verify menu looks native (font, spacing, style)
- [ ] Test light mode and dark mode
- [ ] Test with multiple monitors
- [ ] Test menu bar rearrangement (drag icon)

---

## Documentation Updates

- [ ] Update README.md with macOS menu bar features
- [ ] Update CLAUDE.md with macOS UI implementation details
- [ ] Document macOS-specific design decisions
- [ ] Add screenshots of menu bar in light/dark mode

---

## Related Issues

- Related to: #005-1 (Windows System Tray)
- Related to: #005-3 (Linux System Tray)
- Related to: #005-4 (Icons & Design)
- Related to: #006-2 (macOS Hotkeys - keyboard shortcuts)
- Blocks: #007-2 (Settings UI)

---

## Notes

**macOS Human Interface Guidelines**:
- Menu bar icons should be monochrome (template images)
- Use standard menu item ordering (Quit at bottom)
- Include keyboard shortcuts for common actions
- Use SF Symbols when possible for consistency

**Icon Design**:
- Template icons automatically adapt to light/dark mode
- File format: PDF vector (preferred) or PNG @1x/@2x
- Color: Black on transparent (will be inverted in dark mode)
- Style: Simple, clear, recognizable at small size

**macOS-Specific Behavior**:
- Right-click and left-click both open menu
- Command-drag to rearrange menu bar icons
- Option-click for alternate actions (future)

**Testing Considerations**:
- Test with various macOS versions (11, 12, 13, 14)
- Test with different screen resolutions
- Test with scaled display settings
- Test with external displays

**Future Enhancements**:
- Animated icon during recording (subtle pulse)
- Rich notifications with inline actions
- Quick actions menu (right-click for alternate menu)
- Menu bar extra (larger dropdown with stats)

---

## Definition of Done

- [ ] Menu bar icon implemented and displayed correctly
- [ ] Menu structure follows macOS conventions
- [ ] Icon states work (idle, recording, processing)
- [ ] Keyboard shortcuts functional
- [ ] Light/dark mode supported
- [ ] Tests passing on macOS (Intel and Apple Silicon)
- [ ] Visual design reviewed and approved
- [ ] Documentation updated with screenshots
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

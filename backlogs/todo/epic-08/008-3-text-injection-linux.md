# [ISSUE-008-3] Linux Text Injection Implementation

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: L
**EPIC**: EPIC 8 - Sortie et Injection du Texte

---

## Description

Implement text injection for Linux to automatically type transcribed text into the active application. Must support both X11 and Wayland display servers.

---

## Context

Linux has two main display servers (X11 and Wayland) with different APIs for keyboard simulation. X11 allows direct keyboard event injection via XTest, while Wayland has security restrictions requiring alternative approaches.

---

## Acceptance Criteria

- [ ] Text injection works on X11
- [ ] Text injection works on Wayland (with limitations or workarounds)
- [ ] Works in most applications (text editors, browsers, terminals)
- [ ] Handles special characters and Unicode correctly
- [ ] Clipboard fallback available for incompatible apps
- [ ] Graceful error handling when injection fails

---

## Technical Details

### Affected Components
- `src/output/text_injector_linux.rs` - New Linux text injection implementation
- `src/output/mod.rs` - Platform-specific text injection

### Dependencies
- [ ] X11 libraries (for X11 support): libxtst
- [ ] D-Bus (for Wayland portals)
- [ ] `enigo` crate or native X11/Wayland APIs

### Implementation Notes

**X11 Implementation** (XTest):
```rust
#[cfg(target_os = "linux")]
pub fn inject_text_x11(text: &str) -> Result<()> {
    use enigo::{Enigo, KeyboardControllable};

    let mut enigo = Enigo::new();
    enigo.key_sequence(text);

    Ok(())
}
```

**Wayland Implementation**:
- No direct keyboard injection API (security restriction)
- Options:
  1. Use ydotool (requires setup and daemon)
  2. Use D-Bus RemoteDesktop portal (limited support)
  3. Fallback to clipboard + Ctrl+V

**Display Server Detection**:
```rust
pub fn detect_display_server() -> DisplayServer {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        DisplayServer::Wayland
    } else if std::env::var("DISPLAY").is_ok() {
        DisplayServer::X11
    } else {
        DisplayServer::Unknown
    }
}
```

**Clipboard Fallback**:
```rust
pub fn inject_text_clipboard(text: &str) -> Result<()> {
    use arboard::Clipboard;

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)?;

    // Simulate Ctrl+V
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);

    Ok(())
}
```

---

## Tasks Breakdown

- [ ] Create `src/output/text_injector_linux.rs` module
- [ ] Implement display server detection (X11 vs Wayland)
- [ ] Implement X11 text injection (XTest)
- [ ] Research Wayland injection options (ydotool, portals)
- [ ] Implement Wayland fallback (clipboard method)
- [ ] Handle special characters (newlines, tabs)
- [ ] Handle Unicode characters
- [ ] Add configuration option for injection method
- [ ] Document Wayland limitations
- [ ] Test on X11 (Ubuntu, Fedora)
- [ ] Test on Wayland (GNOME, KDE Plasma)
- [ ] Test in various applications

---

## Test Plan

### Unit Tests
- [ ] Test display server detection
- [ ] Test text sanitization
- [ ] Test special character handling

### Integration Tests
- [ ] Test X11 injection in text editor
- [ ] Test Wayland clipboard fallback
- [ ] Test Unicode handling

### Manual Testing
- [ ] Test on Ubuntu 22.04 GNOME (Wayland)
- [ ] Test on Ubuntu 22.04 GNOME (X11 session)
- [ ] Test on Kubuntu (KDE Plasma)
- [ ] Test on Xubuntu (XFCE, X11)
- [ ] Test in gedit, Kate, VSCode
- [ ] Test in Firefox, Chrome
- [ ] Test in Terminal (gedit, gnome-terminal)
- [ ] Test with emojis and special characters

---

## Documentation Updates

- [ ] Update README.md with Linux text injection info
- [ ] Update CLAUDE.md with X11 vs Wayland details
- [ ] Document Wayland limitations and workarounds
- [ ] Provide setup instructions for ydotool (if used)
- [ ] Add troubleshooting section

---

## Related Issues

- Related to: #008-1 (Windows Text Injection)
- Related to: #008-2 (macOS Text Injection)
- Related to: #008-4 (Text Post-Processing)
- Related to: #005-3 (Linux System Tray)

---

## Notes

**X11 vs Wayland**:
- **X11**: Full support via XTest extension
- **Wayland**: Limited/no direct keyboard injection (security by design)

**Wayland Solutions**:
1. **ydotool**: Requires root/uinput permissions, daemon setup
2. **RemoteDesktop Portal**: Limited DE support, complex
3. **Clipboard method**: Most compatible, but visible to user

**Application Compatibility**:
- X11: Works in almost all applications
- Wayland: Depends on method used
  - Clipboard: Works everywhere
  - ydotool: Works if properly configured
  - Portal: Limited DE support

**ydotool Setup** (if implementing):
```bash
# Install ydotool
sudo apt install ydotool  # Ubuntu/Debian
sudo dnf install ydotool  # Fedora

# Add user to input group
sudo usermod -a -G input $USER

# Start ydotool daemon
ydotoold &
```

**Known Issues**:
- Wayland blocks keyboard simulation by design (security)
- ydotool requires additional setup and permissions
- Some applications block simulated input
- Clipboard method overwrites user's clipboard

**Recommendation**:
- X11: Use XTest (reliable)
- Wayland: Use clipboard method by default
- Advanced: Offer ydotool as optional (with setup guide)

**Future Enhancements**:
- Auto-detect best injection method
- Per-application injection preferences
- Smart clipboard restore
- Wayland portal integration (when more widely supported)

---

## Definition of Done

- [ ] Text injection implemented for X11
- [ ] Wayland fallback implemented (clipboard method)
- [ ] Display server detection working
- [ ] Tests passing on X11 and Wayland
- [ ] Documentation updated with platform differences
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

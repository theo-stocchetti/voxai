# [ISSUE-006-3] Linux Global Hotkeys Implementation

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: L
**EPIC**: EPIC 6 - Raccourcis Clavier

---

## Description

Implement global keyboard shortcuts for Linux to allow users to start/stop recording system-wide. Must support major desktop environments (GNOME, KDE, XFCE) and both X11 and Wayland.

---

## Context

Linux hotkey implementation is complex due to multiple display servers (X11, Wayland) and desktop environments. Each has different APIs and security models. Wayland in particular has restrictions on global hotkeys for security.

---

## Acceptance Criteria

- [ ] Global hotkey works on X11
- [ ] Hotkey works on Wayland (with limitations)
- [ ] Default hotkey: Ctrl+Shift+R
- [ ] Works on GNOME, KDE, XFCE
- [ ] Hotkey toggles recording (start/stop)
- [ ] Graceful degradation if hotkeys unavailable

---

## Technical Details

### Affected Components
- `src/hotkeys/linux.rs` - New Linux hotkey implementation
- `src/hotkeys/mod.rs` - Platform-specific hotkey initialization

### Dependencies
- [ ] `global-hotkey` crate with Linux support
- [ ] X11 libraries (for X11 support)
- [ ] D-Bus for Wayland desktop portals

### Implementation Notes

**X11 Implementation**:
- Use XGrabKey for global hotkey registration
- Direct access to X11 server
- Works on all X11-based desktops

**Wayland Implementation**:
- No direct global hotkey API (security restriction)
- Use desktop-specific D-Bus interfaces:
  - GNOME: org.gnome.Shell.Extensions
  - KDE: Custom KDE hotkey registration
- Requires desktop environment cooperation

**Code Example**:
```rust
#[cfg(target_os = "linux")]
pub fn register_hotkey() -> Result<HotkeyManager> {
    use global_hotkey::{GlobalHotKeyManager, HotKey, modifiers};

    let manager = GlobalHotKeyManager::new()?;

    let hotkey = HotKey::new(
        Some(modifiers::CONTROL | modifiers::SHIFT),
        Code::KeyR
    );

    manager.register(hotkey)?;

    Ok(manager)
}
```

**Wayland Limitations**:
- May require desktop-specific extensions
- GNOME: Needs custom extension or manual keybinding setup
- KDE: Better support via KDE hotkey system
- i3/Sway: Configure in window manager config

---

## Tasks Breakdown

- [ ] Create `src/hotkeys/linux.rs` module
- [ ] Implement X11 global hotkey registration (XGrabKey)
- [ ] Detect X11 vs Wayland at runtime
- [ ] Implement Wayland fallback (D-Bus portals if available)
- [ ] Set default hotkey: Ctrl+Shift+R
- [ ] Implement hotkey event handler
- [ ] Add error handling for failed registration
- [ ] Document Wayland limitations
- [ ] Test on X11 (Ubuntu 22.04, XFCE)
- [ ] Test on Wayland GNOME (Ubuntu 22.04, Fedora)
- [ ] Test on Wayland KDE Plasma
- [ ] Provide workaround instructions for Wayland

---

## Test Plan

### Unit Tests
- [ ] Test hotkey registration on X11
- [ ] Test display server detection (X11 vs Wayland)
- [ ] Test error handling

### Integration Tests
- [ ] Test hotkey on X11 desktop
- [ ] Test hotkey on Wayland (if supported)
- [ ] Test fallback behavior

### Manual Testing
- [ ] Test on Ubuntu 22.04 GNOME (Wayland)
- [ ] Test on Ubuntu 22.04 GNOME (X11 session)
- [ ] Test on Kubuntu (KDE Plasma, X11)
- [ ] Test on Xubuntu (XFCE, X11)
- [ ] Test on Fedora Workstation (GNOME, Wayland)
- [ ] Document which combinations work

---

## Documentation Updates

- [ ] Update README.md with Linux hotkey information
- [ ] Update CLAUDE.md with Linux implementation details
- [ ] Document X11 vs Wayland differences
- [ ] Provide Wayland workaround instructions
- [ ] Add manual keybinding setup guide

---

## Related Issues

- Related to: #006-1 (Windows Hotkeys)
- Related to: #006-2 (macOS Hotkeys)
- Related to: #006-4 (Hotkey Configuration)
- Related to: #005-3 (Linux System Tray)

---

## Notes

**X11 vs Wayland**:
- **X11**: Full global hotkey support, works reliably
- **Wayland**: Limited support, security restrictions
- Most users still on X11 (as of 2024)
- Wayland adoption increasing (Ubuntu 22.04+ default)

**Wayland Workarounds**:
1. Use desktop environment's native hotkey settings
2. User manually configures hotkey to call VoxAI CLI
3. Use D-Bus to register with desktop-specific services

**Desktop Environment Support Matrix**:
| Desktop | Display Server | Hotkey Support |
|---------|----------------|----------------|
| GNOME (X11) | X11 | Full (XGrabKey) |
| GNOME (Wayland) | Wayland | Limited (manual config) |
| KDE Plasma (X11) | X11 | Full (XGrabKey) |
| KDE Plasma (Wayland) | Wayland | Partial (KDE APIs) |
| XFCE | X11 | Full (XGrabKey) |
| i3/Sway | X11/Wayland | Manual config |

**Recommended Approach**:
- Primary: X11 implementation (works for most)
- Secondary: Document Wayland manual setup
- Future: Implement desktop-specific D-Bus integrations

**Future Enhancements**:
- GNOME Shell extension for Wayland hotkeys
- KDE KGlobalAccel integration
- Flatpak/Snap portal support

---

## Definition of Done

- [ ] Global hotkey implemented for X11
- [ ] Wayland detection and graceful handling
- [ ] Default Ctrl+Shift+R working on X11
- [ ] Tests passing on X11 desktops
- [ ] Wayland limitations documented
- [ ] Manual configuration guide provided
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

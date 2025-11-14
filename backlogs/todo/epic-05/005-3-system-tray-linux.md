# [ISSUE-005-3] Linux System Tray Implementation

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 5 - Interface System Tray

---

## Description

Implement a system tray icon and menu for Linux desktop environments. This should support major Linux desktop environments (GNOME, KDE, XFCE, etc.) and follow freedesktop.org standards.

---

## Context

Linux has various desktop environments with different system tray implementations (AppIndicator, StatusNotifier, legacy XEmbed). VoxAI should work across popular Linux desktops while providing a consistent user experience.

---

## Acceptance Criteria

- [ ] System tray icon appears in GNOME (with extension)
- [ ] System tray icon appears in KDE Plasma
- [ ] System tray icon appears in XFCE
- [ ] Icon displays current state (idle, recording, processing)
- [ ] Click opens context menu with options
- [ ] Menu includes Start/Stop, Settings, About, Quit
- [ ] Follows freedesktop.org specifications
- [ ] Supports both light and dark themes

---

## Technical Details

### Affected Components
- `src/ui/tray_linux.rs` - New Linux system tray implementation
- `src/ui/mod.rs` - Platform-specific tray initialization
- `assets/icons/linux/` - Linux-specific icon assets

### Dependencies
- [ ] `tray-icon` crate with Linux support (libappindicator or StatusNotifier)
- [ ] Linux desktop environment with system tray support

### Implementation Notes

**Linux Tray Protocols**:
1. **AppIndicator** (Ubuntu, GNOME with extension)
2. **StatusNotifier** (KDE Plasma)
3. **XEmbed** (Legacy, XFCE, older desktops)

**Icon Specifications** (freedesktop.org):
- SVG format (scalable) or PNG in multiple sizes
- Sizes: 16x16, 22x22, 24x24, 32x32, 48x48, 256x256
- Install to `~/.local/share/icons/hicolor/`
- Follow Icon Theme Specification

**Menu Structure**:
```
[Icon]
├── Start Recording
├── Stop Recording
├── ─────────────
├── Settings
├── ─────────────
├── About VoxAI
├── Quit
```

**Code Example**:
```rust
#[cfg(target_os = "linux")]
pub fn create_system_tray() -> Result<TrayIcon> {
    use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem}};

    let icon = load_icon_linux("idle")?;

    let menu = Menu::new();
    menu.append(&MenuItem::new("Start Recording", true, None))?;
    menu.append(&MenuItem::separator())?;
    menu.append(&MenuItem::new("Settings", true, None))?;
    menu.append(&MenuItem::separator())?;
    menu.append(&MenuItem::new("Quit", true, None))?;

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .with_tooltip("VoxAI - Voice Transcription")
        .build()?;

    Ok(tray)
}
```

**Desktop Environment Support**:
- **GNOME 3/4**: Requires AppIndicator extension
- **KDE Plasma**: Native StatusNotifier support
- **XFCE**: Native system tray (XEmbed)
- **MATE, Cinnamon**: AppIndicator support
- **i3, Sway**: Via i3bar/swaybar with tray

---

## Tasks Breakdown

- [ ] Create `src/ui/tray_linux.rs` module
- [ ] Implement AppIndicator support
- [ ] Implement StatusNotifier support (KDE)
- [ ] Implement XEmbed fallback (legacy)
- [ ] Design Linux-compatible icons (SVG + PNG)
- [ ] Export icons in required sizes (16, 22, 24, 32, 48, 256)
- [ ] Implement icon state changes (idle/recording/processing)
- [ ] Create menu structure
- [ ] Implement menu event handling
- [ ] Add desktop file for proper integration
- [ ] Test on GNOME (with AppIndicator extension)
- [ ] Test on KDE Plasma
- [ ] Test on XFCE
- [ ] Support light and dark themes

---

## Test Plan

### Unit Tests
- [ ] Test tray initialization
- [ ] Test menu item creation and event handling
- [ ] Test icon loading and state changes

### Integration Tests
- [ ] Test tray appears in supported desktop environments
- [ ] Test menu opens on click
- [ ] Test all menu items trigger correct actions

### Manual Testing
- [ ] Test on Ubuntu 22.04/24.04 with GNOME
- [ ] Test on Kubuntu (KDE Plasma)
- [ ] Test on Xubuntu (XFCE)
- [ ] Test on Fedora Workstation (GNOME)
- [ ] Test on Arch Linux with various DEs
- [ ] Test on Pop!_OS (GNOME fork)
- [ ] Verify icon theme compatibility (light/dark)
- [ ] Test with HiDPI displays

---

## Documentation Updates

- [ ] Update README.md with Linux system tray requirements
- [ ] Update CLAUDE.md with Linux UI implementation details
- [ ] Document desktop environment compatibility
- [ ] Document GNOME AppIndicator extension requirement
- [ ] Add troubleshooting for missing system tray

---

## Related Issues

- Related to: #005-1 (Windows System Tray)
- Related to: #005-2 (macOS Menu Bar)
- Related to: #005-4 (Icons & Design)
- Related to: #006-3 (Linux Hotkeys)
- Blocks: #007-2 (Settings UI)

---

## Notes

**Desktop Environment Compatibility**:
| Desktop Environment | Protocol | Native Support |
|---------------------|----------|----------------|
| GNOME 3/4 | AppIndicator | Extension required |
| KDE Plasma | StatusNotifier | Yes |
| XFCE | XEmbed | Yes |
| MATE | AppIndicator | Yes |
| Cinnamon | AppIndicator | Yes |
| Budgie | StatusNotifier | Yes |
| i3/Sway | Various | Via bar config |

**GNOME AppIndicator Extension**:
- GNOME removed native system tray in GNOME 3.26
- Users must install "AppIndicator and KStatusNotifierItem Support" extension
- Document this requirement clearly in README

**Icon Theme Integration**:
- Icons should follow user's icon theme
- Provide both symbolic (outline) and full-color variants
- Symbolic icons work better for system tray
- Install icons to `~/.local/share/icons/hicolor/`

**Known Issues**:
- GNOME requires manual extension installation
- Wayland may have different behavior than X11
- Some lightweight window managers don't support system tray
- Icon sizes vary by desktop environment

**Testing Matrix**:
| OS | Desktop | Protocol | Priority |
|----|---------|----------|----------|
| Ubuntu 22.04 | GNOME | AppIndicator | High |
| Ubuntu 24.04 | GNOME | AppIndicator | High |
| Kubuntu | KDE Plasma | StatusNotifier | High |
| Xubuntu | XFCE | XEmbed | Medium |
| Fedora | GNOME | AppIndicator | Medium |
| Arch | Various | All | Low |

**Future Enhancements**:
- Auto-install AppIndicator extension (if possible)
- Wayland-native protocol support
- Custom tray icon colors (user theme)
- Rich notifications with D-Bus integration

---

## Definition of Done

- [ ] System tray implemented for Linux
- [ ] AppIndicator support working
- [ ] StatusNotifier support working (KDE)
- [ ] XEmbed fallback working (legacy)
- [ ] Icons exported in all required sizes
- [ ] Menu structure implemented
- [ ] Tests passing on GNOME, KDE, XFCE
- [ ] Light/dark theme support
- [ ] Documentation updated with DE requirements
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

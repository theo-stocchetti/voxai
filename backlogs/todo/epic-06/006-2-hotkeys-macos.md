# [ISSUE-006-2] macOS Global Hotkeys Implementation

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 6 - Raccourcis Clavier

---

## Description

Implement global keyboard shortcuts for macOS to allow users to start/stop recording from anywhere in the system. Default shortcut: ⌘⇧R (Command+Shift+R).

---

## Context

macOS provides APIs for registering global hotkeys that work system-wide. This is essential for VoxAI's core functionality - users need to trigger transcription without switching to the app.

---

## Acceptance Criteria

- [ ] Global hotkey registered successfully on macOS
- [ ] Default hotkey: ⌘⇧R (Command+Shift+R)
- [ ] Hotkey works from any application
- [ ] Hotkey starts recording when idle
- [ ] Hotkey stops recording when active
- [ ] Proper error handling if hotkey registration fails
- [ ] Accessibility permissions requested if needed

---

## Technical Details

### Affected Components
- `src/hotkeys/macos.rs` - New macOS hotkey implementation
- `src/hotkeys/mod.rs` - Platform-specific hotkey initialization

### Dependencies
- [ ] `global-hotkey` crate with macOS support
- [ ] macOS 11.0+ (Big Sur or later)
- [ ] Accessibility permissions may be required

### Implementation Notes

**macOS Hotkey API**:
- Uses Carbon Event Manager or Cocoa NSEvent
- Requires Accessibility permissions for some key combinations
- System shortcuts take precedence (can't override ⌘Q, etc.)

**Code Example**:
```rust
#[cfg(target_os = "macos")]
pub fn register_hotkey() -> Result<HotkeyManager> {
    use global_hotkey::{GlobalHotKeyManager, HotKey, modifiers};

    let manager = GlobalHotKeyManager::new()?;

    let hotkey = HotKey::new(
        Some(modifiers::SUPER | modifiers::SHIFT), // Command+Shift
        Code::KeyR
    );

    manager.register(hotkey)?;

    Ok(manager)
}
```

**Accessibility Permissions**:
- May need to request in System Settings > Privacy & Security > Accessibility
- Display clear instructions if permission denied

---

## Tasks Breakdown

- [ ] Create `src/hotkeys/macos.rs` module
- [ ] Implement global hotkey registration
- [ ] Set default hotkey: ⌘⇧R
- [ ] Implement hotkey event handler (toggle recording)
- [ ] Add accessibility permission check
- [ ] Display permission request dialog if needed
- [ ] Handle hotkey registration errors gracefully
- [ ] Test on macOS 13 Ventura
- [ ] Test on macOS 14 Sonoma
- [ ] Test on Intel and Apple Silicon

---

## Test Plan

### Unit Tests
- [ ] Test hotkey registration
- [ ] Test hotkey event callback
- [ ] Test error handling

### Integration Tests
- [ ] Test hotkey triggers recording start
- [ ] Test hotkey triggers recording stop
- [ ] Test hotkey works from various applications

### Manual Testing
- [ ] Register hotkey and verify it works
- [ ] Test from Finder, Safari, TextEdit, Terminal
- [ ] Test with multiple monitors
- [ ] Test permission request flow
- [ ] Verify error messages if registration fails

---

## Documentation Updates

- [ ] Update README.md with default macOS hotkey
- [ ] Update CLAUDE.md with macOS hotkey implementation
- [ ] Document accessibility permission requirements
- [ ] Add troubleshooting for permission issues

---

## Related Issues

- Related to: #006-1 (Windows Hotkeys)
- Related to: #006-3 (Linux Hotkeys)
- Related to: #006-4 (Hotkey Configuration)
- Related to: #005-2 (macOS Menu Bar)

---

## Notes

**macOS Modifier Keys**:
- ⌘ Command (Super)
- ⇧ Shift
- ⌥ Option (Alt)
- ⌃ Control

**Accessibility Permissions**:
- Required for global hotkeys to work in all apps
- User must grant permission manually in System Settings
- Provide clear instructions with screenshots

**Future Enhancements**:
- Customizable hotkeys (006-4)
- Multiple hotkey profiles
- Push-to-talk mode (hold key to record)

---

## Definition of Done

- [ ] Global hotkey implemented for macOS
- [ ] Default ⌘⇧R hotkey working
- [ ] Accessibility permissions handled
- [ ] Tests passing on macOS
- [ ] Documentation updated
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

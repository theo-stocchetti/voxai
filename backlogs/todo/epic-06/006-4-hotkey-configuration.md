# [ISSUE-006-4] Hotkey Configuration and Customization

**Created**: 2025-11-14
**Priority**: Moyenne
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 6 - Raccourcis Clavier

---

## Description

Allow users to customize the global hotkey for starting/stopping transcription. Provide a UI to change the hotkey and validate that the chosen combination is available.

---

## Context

Different users have different preferences and workflows. Some hotkey combinations may conflict with other applications. Users should be able to customize the hotkey to fit their needs.

---

## Acceptance Criteria

- [ ] Users can change the hotkey in settings
- [ ] Hotkey customization UI is intuitive (record key press)
- [ ] Validate hotkey doesn't conflict with system shortcuts
- [ ] Custom hotkey persists across sessions
- [ ] Display current hotkey in settings and menu
- [ ] Handle invalid or unusable key combinations gracefully

---

## Technical Details

### Affected Components
- `src/config/schema.rs` - Add hotkey configuration field
- `src/ui/settings.rs` - Add hotkey customization UI
- `src/hotkeys/*.rs` - Support dynamic hotkey registration

### Dependencies
- [ ] Persistent configuration (007-1)
- [ ] Settings UI (007-2)
- [ ] Platform-specific hotkey implementations (006-1, 006-2, 006-3)

### Implementation Notes

**Configuration Schema**:
```rust
#[derive(Serialize, Deserialize)]
struct HotkeyConfig {
    modifiers: Vec<Modifier>, // Ctrl, Shift, Alt, Super
    key: String,              // Key code (e.g., "R", "Space")
}

#[derive(Serialize, Deserialize)]
struct Config {
    hotkey: HotkeyConfig,
    // ... other config
}
```

**Hotkey Recording UI**:
- Button: "Press keys to set hotkey"
- User clicks button and presses desired key combination
- Display: "Ctrl+Shift+R" (formatted)
- Validate and save

**Validation**:
- Check for valid modifier + key combination
- Warn if conflicts with common system shortcuts
- Platform-specific validation (e.g., can't use ⌘Q on macOS)

---

## Tasks Breakdown

- [ ] Add hotkey configuration to config schema
- [ ] Create hotkey recording UI component
- [ ] Implement key press capture for recording
- [ ] Format and display hotkey combination
- [ ] Validate hotkey (platform-specific rules)
- [ ] Warn about potential conflicts
- [ ] Update hotkey registration when config changes
- [ ] Display current hotkey in system tray menu
- [ ] Add "Reset to default" button
- [ ] Test hotkey customization on all platforms

---

## Test Plan

### Unit Tests
- [ ] Test hotkey parsing and formatting
- [ ] Test hotkey validation logic
- [ ] Test config serialization/deserialization

### Integration Tests
- [ ] Test hotkey registration with custom keys
- [ ] Test hotkey persistence across app restarts
- [ ] Test invalid hotkey rejection

### Manual Testing
- [ ] Change hotkey in settings UI
- [ ] Verify new hotkey works immediately
- [ ] Restart app and verify hotkey persisted
- [ ] Test various key combinations (Ctrl+X, Alt+Space, etc.)
- [ ] Try invalid combinations (key without modifier)
- [ ] Test on Windows, macOS, Linux

---

## Documentation Updates

- [ ] Update README.md with hotkey customization info
- [ ] Update CLAUDE.md with config schema
- [ ] Add screenshots of hotkey settings UI
- [ ] Document platform-specific limitations

---

## Related Issues

- Depends on: #007-1 (Persistent Configuration)
- Depends on: #007-2 (Settings UI)
- Related to: #006-1, #006-2, #006-3 (Platform hotkey implementations)

---

## Notes

**Default Hotkeys by Platform**:
- Windows: Ctrl+Shift+R
- macOS: ⌘⇧R (Command+Shift+R)
- Linux: Ctrl+Shift+R

**Modifier Keys**:
- Ctrl (Control)
- Shift
- Alt (Option on macOS)
- Super (Windows key, Command on macOS)

**Common Conflicts to Warn About**:
- Windows: Win+L (lock), Ctrl+Alt+Del
- macOS: ⌘Q (quit), ⌘W (close)
- Linux: Ctrl+Alt+T (terminal), Super+L (lock)

**UI Patterns**:
- Similar to hotkey recording in:
  - OBS Studio
  - Discord
  - Slack
  - VS Code keybindings

**Future Enhancements**:
- Multiple hotkeys for different actions
- Push-to-talk mode (hold to record)
- Hotkey profiles (work, gaming, etc.)
- Import/export hotkey configurations

---

## Definition of Done

- [ ] Hotkey configuration added to schema
- [ ] Hotkey recording UI implemented
- [ ] Hotkey validation working
- [ ] Custom hotkey registration working
- [ ] Hotkey persists across sessions
- [ ] Tests passing on all platforms
- [ ] Documentation updated
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

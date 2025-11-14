# [ISSUE-008-2] macOS Text Injection Implementation

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 8 - Sortie et Injection du Texte

---

## Description

Implement text injection for macOS to automatically type transcribed text into the active application. Use macOS accessibility APIs to simulate keyboard input.

---

## Context

After transcription completes, users expect the text to appear in their current application (text editor, email, chat, etc.). macOS provides accessibility APIs (CGEvent) for programmatic keyboard input.

---

## Acceptance Criteria

- [ ] Transcribed text is injected into active application
- [ ] Text appears as if user typed it
- [ ] Works in most macOS applications (TextEdit, Mail, Safari, etc.)
- [ ] Handles special characters and Unicode correctly
- [ ] Accessibility permissions requested if needed
- [ ] Option to use clipboard as fallback

---

## Technical Details

### Affected Components
- `src/output/text_injector_macos.rs` - New macOS text injection implementation
- `src/output/mod.rs` - Platform-specific text injection

### Dependencies
- [ ] macOS accessibility permissions
- [ ] `enigo` crate or native CGEvent APIs

### Implementation Notes

**macOS Text Injection Methods**:
1. **CGEvent** (preferred): Low-level keyboard event simulation
2. **Accessibility API**: Higher-level text insertion
3. **Clipboard + Cmd+V** (fallback): Most compatible but visible to user

**Code Example**:
```rust
#[cfg(target_os = "macos")]
pub fn inject_text(text: &str) -> Result<()> {
    use enigo::{Enigo, KeyboardControllable};

    let mut enigo = Enigo::new();

    // Type the text character by character
    enigo.key_sequence(text);

    Ok(())
}

// Alternative: Use clipboard
pub fn inject_text_clipboard(text: &str) -> Result<()> {
    use arboard::Clipboard;
    use enigo::{Enigo, Key, KeyboardControllable};

    // Save current clipboard
    let mut clipboard = Clipboard::new()?;
    let original_clipboard = clipboard.get_text().ok();

    // Set transcribed text to clipboard
    clipboard.set_text(text)?;

    // Simulate Cmd+V
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Meta); // Command key
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Meta);

    // Restore original clipboard (after delay)
    if let Some(original) = original_clipboard {
        std::thread::sleep(std::time::Duration::from_millis(100));
        clipboard.set_text(&original)?;
    }

    Ok(())
}
```

**Accessibility Permissions**:
- Required for CGEvent keyboard simulation
- User must grant in System Settings > Privacy & Security > Accessibility
- Provide clear instructions with screenshots

---

## Tasks Breakdown

- [ ] Create `src/output/text_injector_macos.rs` module
- [ ] Implement text injection using enigo or CGEvent
- [ ] Handle special characters (newlines, tabs, quotes)
- [ ] Handle Unicode/emoji
- [ ] Implement clipboard fallback method
- [ ] Add accessibility permission check
- [ ] Display permission request dialog if needed
- [ ] Add configuration option for injection method
- [ ] Test in TextEdit, Mail, Safari, Messages
- [ ] Test with long text (>1000 characters)
- [ ] Test with special characters and Unicode

---

## Test Plan

### Unit Tests
- [ ] Test text sanitization
- [ ] Test special character handling

### Integration Tests
- [ ] Test text injection in TextEdit
- [ ] Test clipboard method
- [ ] Test permission handling

### Manual Testing
- [ ] Test in TextEdit (native macOS app)
- [ ] Test in Mail.app
- [ ] Test in Safari (web forms)
- [ ] Test in Messages
- [ ] Test in Microsoft Word (if installed)
- [ ] Test in Terminal (may not work - expected)
- [ ] Test with long transcriptions
- [ ] Test with emojis and special characters
- [ ] Verify accessibility permission flow

---

## Documentation Updates

- [ ] Update README.md with macOS text injection info
- [ ] Update CLAUDE.md with implementation details
- [ ] Document accessibility permission requirements
- [ ] Add troubleshooting for apps where injection doesn't work

---

## Related Issues

- Related to: #008-1 (Windows Text Injection)
- Related to: #008-3 (Linux Text Injection)
- Related to: #008-4 (Text Post-Processing)
- Related to: #005-2 (macOS Menu Bar)

---

## Notes

**macOS Application Compatibility**:
- Works: TextEdit, Mail, Safari, Chrome, Firefox, Messages, Slack, Discord
- May not work: Terminal, some games, protected apps
- Fallback: Use clipboard method for incompatible apps

**Accessibility Permissions**:
- Required for keyboard simulation
- One-time user approval
- App must be added to Accessibility list

**Clipboard Method Trade-offs**:
- **Pros**: Works in more apps, simpler implementation
- **Cons**: Overwrites clipboard temporarily, visible paste action

**Special Characters**:
- Newlines: `\n` → Enter key
- Tabs: `\t` → Tab key
- Quotes: May need escaping depending on application

**Future Enhancements**:
- Smart paste (detect cursor position)
- Formatting preservation (bold, italic)
- Auto-capitalize sentences
- Smart quotes and punctuation
- Undo support (Cmd+Z removes injected text)

---

## Definition of Done

- [ ] Text injection implemented for macOS
- [ ] CGEvent or enigo-based injection working
- [ ] Clipboard fallback implemented
- [ ] Accessibility permissions handled
- [ ] Tests passing in major macOS applications
- [ ] Documentation updated
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

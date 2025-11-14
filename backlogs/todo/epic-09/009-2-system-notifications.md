# [ISSUE-009-2] System Notifications Implementation

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: S
**EPIC**: EPIC 9 - Indicateurs Visuels et Feedback

---

## Description

Implement native system notifications to inform users of recording start/stop, transcription completion, and errors. Use platform-specific notification APIs.

---

## Context

System notifications provide non-intrusive feedback to users. They inform users when:
- Recording has started
- Recording has stopped
- Transcription is complete
- Errors occur (e.g., no microphone, permission denied)

---

## Acceptance Criteria

- [ ] Notification on recording start
- [ ] Notification on recording stop
- [ ] Notification on transcription complete (with preview)
- [ ] Error notifications for failures
- [ ] Notifications follow platform conventions
- [ ] Notifications can be disabled in settings

---

## Technical Details

### Affected Components
- `src/ui/notifications.rs` - New notifications module

### Dependencies
- [ ] `notify-rust` crate or platform-specific APIs

### Implementation Notes

**Cross-Platform Notifications**:
```rust
use notify_rust::Notification;

pub fn notify_recording_started() -> Result<()> {
    Notification::new()
        .summary("VoxAI")
        .body("Recording started")
        .icon("microphone")
        .show()?;
    Ok(())
}

pub fn notify_transcription_complete(text: &str) -> Result<()> {
    let preview = text.chars().take(100).collect::<String>();
    Notification::new()
        .summary("Transcription Complete")
        .body(&preview)
        .icon("check")
        .show()?;
    Ok(())
}
```

---

## Tasks Breakdown

- [ ] Add notify-rust dependency
- [ ] Create notifications module
- [ ] Implement notification functions (start, stop, complete, error)
- [ ] Add notification configuration (enable/disable)
- [ ] Test notifications on Windows, macOS, Linux

---

## Test Plan

### Manual Testing
- [ ] Start recording → verify notification
- [ ] Stop recording → verify notification
- [ ] Complete transcription → verify notification with preview
- [ ] Trigger error → verify error notification
- [ ] Test on all platforms

---

## Documentation Updates

- [ ] Update README.md with notifications feature

---

## Related Issues

- Related to: #009-1 (Status Overlay)

---

## Definition of Done

- [ ] Notifications implemented for all key events
- [ ] Works on all platforms
- [ ] Configurable in settings
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

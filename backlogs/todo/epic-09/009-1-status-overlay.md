# [ISSUE-009-1] Status Overlay Implementation

**Created**: 2025-11-14
**Priority**: Moyenne
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 9 - Indicateurs Visuels et Feedback

---

## Description

Implement a semi-transparent overlay window that shows recording status, transcription progress, and current text preview while transcribing.

---

## Context

Users need visual feedback while recording to know:
- Recording is active
- Audio is being captured
- Transcription is processing
- Preview of transcribed text

An overlay provides non-intrusive, always-visible feedback.

---

## Acceptance Criteria

- [ ] Overlay appears when recording starts
- [ ] Shows recording indicator (red dot/pulse animation)
- [ ] Shows audio level visualization (waveform or bar)
- [ ] Shows transcription progress/status
- [ ] Shows preview of transcribed text (last few words)
- [ ] Overlay is semi-transparent and non-intrusive
- [ ] User can move overlay position
- [ ] Overlay auto-hides when recording stops

---

## Technical Details

### Affected Components
- `src/ui/overlay.rs` - New overlay window implementation
- Uses egui/eframe for cross-platform overlay

### Implementation Notes

**Overlay Features**:
- Always-on-top window
- Transparent background
- Draggable
- Shows: status icon, audio level, transcription preview

**Code Sketch**:
```rust
use eframe::egui;

pub struct StatusOverlay {
    recording: bool,
    audio_level: f32,
    preview_text: String,
}

impl eframe::App for StatusOverlay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("VoxAI Status")
            .title_bar(false)
            .resizable(false)
            .default_pos([10.0, 10.0])
            .show(ctx, |ui| {
                if self.recording {
                    ui.colored_label(egui::Color32::RED, "● Recording");
                    ui.add(egui::widgets::ProgressBar::new(self.audio_level));
                    ui.label(&self.preview_text);
                }
            });
    }
}
```

---

## Tasks Breakdown

- [ ] Create `src/ui/overlay.rs` module
- [ ] Implement overlay window with egui
- [ ] Add recording status indicator
- [ ] Add audio level visualization
- [ ] Add transcription preview (last N words)
- [ ] Make overlay always-on-top
- [ ] Make overlay draggable
- [ ] Add auto-hide when recording stops
- [ ] Add configuration for overlay position
- [ ] Test on Windows, macOS, Linux

---

## Test Plan

### Manual Testing
- [ ] Start recording and verify overlay appears
- [ ] Verify audio level updates in real-time
- [ ] Verify transcription preview updates
- [ ] Drag overlay and verify position saves
- [ ] Stop recording and verify overlay hides
- [ ] Test on multiple monitors

---

## Documentation Updates

- [ ] Update README.md with overlay feature
- [ ] Add screenshots of overlay

---

## Related Issues

- Related to: #009-2 (System Notifications)
- Related to: #002-1 (Audio Capture - audio level data)

---

## Notes

**Future Enhancements**:
- Customizable overlay theme
- Waveform visualization
- Confidence indicators
- Keyboard shortcut to toggle overlay

---

## Definition of Done

- [ ] Overlay implemented and functional
- [ ] Shows recording status, audio level, preview
- [ ] Works on all platforms
- [ ] Documentation updated
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

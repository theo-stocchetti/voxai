# [ISSUE-007-2] Settings UI Implementation

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: L
**EPIC**: EPIC 7 - Gestion de la Configuration

---

## Description

Implement a graphical settings window using egui to allow users to configure VoxAI preferences including transcription model, language, hotkeys, audio device, and output behavior.

---

## Context

Users need an intuitive interface to configure VoxAI without editing configuration files manually. The settings UI should be accessible from the system tray menu and provide clear, organized options.

---

## Acceptance Criteria

- [ ] Settings window opens from system tray menu
- [ ] Settings organized in logical sections/tabs
- [ ] Users can configure: model, language, hotkey, audio device, output
- [ ] Changes are saved immediately or on "Apply"
- [ ] Settings window works on all platforms (Windows, macOS, Linux)
- [ ] Responsive and user-friendly interface

---

## Technical Details

### Affected Components
- `src/ui/settings.rs` - Settings window implementation with egui
- `src/ui/mod.rs` - Settings window lifecycle management
- `src/config/mod.rs` - Load and save configuration

### Dependencies
- [ ] `egui` / `eframe` for GUI
- [ ] Persistent configuration (007-1)
- [ ] Related to all configurable features

### Implementation Notes

**UI Framework**: egui (eframe)
- Cross-platform native UI
- Rust-native, good integration
- Lightweight and performant

**Settings Sections**:
1. **General**: Language, startup behavior
2. **Transcription**: Model selection, language detection
3. **Audio**: Input device, noise reduction, VAD
4. **Hotkeys**: Global hotkey customization
5. **Output**: Text injection mode, formatting
6. **Advanced**: GPU acceleration, logging, updates

**Code Example**:
```rust
use eframe::egui;

pub struct SettingsWindow {
    config: Config,
}

impl eframe::App for SettingsWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("VoxAI Settings");

            ui.separator();

            // General settings
            ui.collapsing("General", |ui| {
                ui.label("Language:");
                egui::ComboBox::from_label("Language")
                    .selected_text(&self.config.language)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.language, "auto".to_string(), "Auto-detect");
                        ui.selectable_value(&mut self.config.language, "en".to_string(), "English");
                        ui.selectable_value(&mut self.config.language, "fr".to_string(), "French");
                        // ... more languages
                    });
            });

            // Transcription settings
            ui.collapsing("Transcription", |ui| {
                ui.label("Whisper Model:");
                egui::ComboBox::from_label("Model")
                    .selected_text(&self.config.model)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.model, "tiny".to_string(), "Tiny (fastest)");
                        ui.selectable_value(&mut self.config.model, "base".to_string(), "Base");
                        ui.selectable_value(&mut self.config.model, "small".to_string(), "Small");
                        ui.selectable_value(&mut self.config.model, "medium".to_string(), "Medium");
                    });
            });

            // Audio settings
            ui.collapsing("Audio", |ui| {
                ui.label("Input Device:");
                // Device selector (007-3)
                ui.checkbox(&mut self.config.noise_reduction, "Enable Noise Reduction");
                ui.checkbox(&mut self.config.vad_enabled, "Enable Voice Activity Detection");
            });

            // Save button
            ui.separator();
            if ui.button("Save Settings").clicked() {
                self.save_config();
            }
        });
    }
}
```

---

## Tasks Breakdown

- [ ] Add egui/eframe dependencies to Cargo.toml
- [ ] Create `src/ui/settings.rs` module
- [ ] Implement settings window with eframe::App
- [ ] Add General settings section
- [ ] Add Transcription settings section (model, language)
- [ ] Add Audio settings section (device selector, noise reduction, VAD)
- [ ] Add Hotkeys settings section (hotkey customization UI)
- [ ] Add Output settings section (injection mode, formatting)
- [ ] Add Advanced settings section (GPU, logging)
- [ ] Implement "Save" and "Cancel" buttons
- [ ] Load current config on window open
- [ ] Save config on Apply/Save
- [ ] Add settings menu item to system tray
- [ ] Test on Windows, macOS, Linux

---

## Test Plan

### Unit Tests
- [ ] Test config loading in settings window
- [ ] Test config saving from settings window

### Integration Tests
- [ ] Test opening settings from system tray
- [ ] Test changing settings and saving
- [ ] Test settings persist after restart

### Manual Testing
- [ ] Open settings window from tray menu
- [ ] Change model selection
- [ ] Change language setting
- [ ] Change audio device
- [ ] Customize hotkey
- [ ] Save and verify changes applied
- [ ] Restart app and verify settings persisted
- [ ] Test on Windows, macOS, Linux

---

## Documentation Updates

- [ ] Update README.md with settings UI features
- [ ] Update CLAUDE.md with settings implementation details
- [ ] Add screenshots of settings window
- [ ] Document all available settings

---

## Related Issues

- Depends on: #007-1 (Persistent Configuration)
- Related to: #003-2 (Model Management - model selector)
- Related to: #003-3 (Multi-language - language selector)
- Related to: #006-4 (Hotkey Configuration - hotkey UI)
- Related to: #007-3 (Audio Device Selection)
- Related to: #008-4 (Text Post-Processing - formatting options)

---

## Notes

**egui Advantages**:
- Pure Rust, no native dependencies
- Cross-platform (Windows, macOS, Linux)
- Immediate mode GUI (simple state management)
- Good for settings windows

**Alternative UI Frameworks**:
- iced: Elm-inspired, more complex
- gtk-rs: Native look, complex setup
- Qt bindings: Heavy dependency

**Settings Organization Best Practices**:
- Group related settings
- Use collapsible sections or tabs for clarity
- Provide tooltips for complex settings
- Show recommended/default values
- Validate input (e.g., hotkey conflicts)

**Future Enhancements**:
- Tabbed interface for better organization
- Search/filter settings
- Import/export settings
- Settings profiles (work, home, etc.)
- Real-time preview of settings changes

---

## Definition of Done

- [ ] Settings window implemented with egui
- [ ] All major settings configurable via UI
- [ ] Settings load and save correctly
- [ ] Window accessible from system tray
- [ ] Tests passing on all platforms
- [ ] Documentation updated with screenshots
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

//! Settings window using egui
//!
//! Provides a graphical settings interface for configuring VoxAI

use crate::config::{Config, save_config};
use anyhow::Result;
use eframe::egui;

/// Settings window application
pub struct SettingsWindow {
    config: Config,
    has_changes: bool,
    status_message: Option<String>,
    show_advanced: bool,
}

impl SettingsWindow {
    /// Create a new settings window with given config
    pub fn new(config: Config) -> Self {
        Self {
            config,
            has_changes: false,
            status_message: None,
            show_advanced: false,
        }
    }

    /// Run the settings window
    pub fn run(config: Config) -> Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([600.0, 500.0])
                .with_title("VoxAI Settings"),
            ..Default::default()
        };

        eframe::run_native(
            "VoxAI Settings",
            options,
            Box::new(|_cc| Box::new(SettingsWindow::new(config)) as Box<dyn eframe::App>),
        )
        .map_err(|e| anyhow::anyhow!("Failed to run settings window: {}", e))
    }

    /// Save current configuration
    fn save(&mut self) {
        match save_config(&self.config) {
            Ok(_) => {
                self.has_changes = false;
                self.status_message = Some("Settings saved successfully!".to_string());
                log::info!("Settings saved successfully");
            }
            Err(e) => {
                self.status_message = Some(format!("Error saving settings: {}", e));
                log::error!("Failed to save settings: {}", e);
            }
        }
    }

    /// Reset to default configuration
    fn reset_to_defaults(&mut self) {
        self.config = Config::default();
        self.has_changes = true;
        self.status_message = Some("Settings reset to defaults".to_string());
    }
}

impl eframe::App for SettingsWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("VoxAI Settings");
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Audio Settings
                ui.heading("Audio");
                ui.horizontal(|ui| {
                    ui.label("Input Device:");
                    if ui.text_edit_singleline(&mut self.config.audio.input_device).changed() {
                        self.has_changes = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Sample Rate:");
                    ui.label(format!("{} Hz", self.config.audio.sample_rate));
                    ui.label("(Fixed at 16kHz for Whisper)");
                });

                if ui.checkbox(&mut self.config.audio.noise_reduction, "Enable Noise Reduction").changed() {
                    self.has_changes = true;
                }

                ui.separator();

                // Transcription Settings
                ui.heading("Transcription");

                ui.horizontal(|ui| {
                    ui.label("Whisper Model:");
                    egui::ComboBox::from_label("")
                        .selected_text(&self.config.transcription.model)
                        .show_ui(ui, |ui| {
                            let models = ["tiny", "base", "small", "medium"];
                            for model in models {
                                if ui.selectable_value(&mut self.config.transcription.model, model.to_string(), model).clicked() {
                                    self.has_changes = true;
                                }
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Language:");
                    if ui.text_edit_singleline(&mut self.config.transcription.language).changed() {
                        self.has_changes = true;
                    }
                    ui.label("(Use 'auto' for automatic detection)");
                });

                if ui.checkbox(&mut self.config.transcription.enable_gpu, "Enable GPU Acceleration").changed() {
                    self.has_changes = true;
                }

                ui.horizontal(|ui| {
                    ui.label("Voice Activity Detection:");
                    if ui.add(egui::Slider::new(&mut self.config.transcription.vad_aggressiveness, 0..=3)).changed() {
                        self.has_changes = true;
                    }
                });

                ui.separator();

                // Hotkeys Settings
                ui.heading("Hotkeys");
                ui.horizontal(|ui| {
                    ui.label("Toggle Recording:");
                    if ui.text_edit_singleline(&mut self.config.hotkeys.toggle_recording).changed() {
                        self.has_changes = true;
                    }
                });

                ui.separator();

                // UI Settings
                ui.heading("User Interface");

                if ui.checkbox(&mut self.config.ui.show_overlay, "Show Status Overlay").changed() {
                    self.has_changes = true;
                }

                if ui.checkbox(&mut self.config.ui.system_notifications, "Enable System Notifications").changed() {
                    self.has_changes = true;
                }

                if ui.checkbox(&mut self.config.ui.auto_capitalization, "Auto-Capitalize First Letter").changed() {
                    self.has_changes = true;
                }
            });

            ui.separator();

            // Status message
            if let Some(ref msg) = self.status_message {
                ui.label(msg);
            }

            // Action buttons
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    self.save();
                }

                if ui.button("Reset to Defaults").clicked() {
                    self.reset_to_defaults();
                }

                if self.has_changes {
                    ui.label("⚠ You have unsaved changes");
                }
            });
        });
    }
}

/// Launch settings window (non-blocking)
pub fn launch_settings_window(config: Config) -> Result<()> {
    log::info!("Launching settings window");

    // Spawn in separate thread to avoid blocking
    std::thread::spawn(move || {
        match SettingsWindow::run(config) {
            Ok(_) => {
                log::info!("Settings window closed");
            }
            Err(e) => {
                log::error!("Settings window error: {}", e);
            }
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_window_creation() {
        let config = Config::default();
        let settings = SettingsWindow::new(config);
        assert!(!settings.has_changes);
        assert!(settings.status_message.is_none());
        assert!(!settings.show_advanced);
    }

    #[test]
    fn test_reset_to_defaults() {
        let mut config = Config::default();
        config.transcription.model = "medium".to_string();

        let mut settings = SettingsWindow::new(config);
        settings.reset_to_defaults();

        assert_eq!(settings.config.transcription.model, "base");
        assert!(settings.has_changes);
    }
}

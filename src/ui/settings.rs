//! Settings UI window (egui)
//!
//! Provides a graphical settings window for VoxAI configuration

use eframe::egui;
use anyhow::Result;

use crate::config::Config;

/// Settings window state
pub struct SettingsWindow {
    config: Config,
    show_advanced: bool,
}

impl SettingsWindow {
    /// Create new settings window with current config
    pub fn new(config: Config) -> Self {
        Self {
            config,
            show_advanced: false,
        }
    }

    /// Show the settings window (blocking until closed)
    pub fn show(mut self) -> Result<Config> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([500.0, 600.0])
                .with_title("VoxAI Settings"),
            ..Default::default()
        };

        let config = std::sync::Arc::new(std::sync::Mutex::new(self.config.clone()));
        let config_clone = config.clone();

        eframe::run_simple_native("VoxAI Settings", options, move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("VoxAI Settings");
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut cfg = config_clone.lock().unwrap();

                    // General Settings
                    ui.group(|ui| {
                        ui.label("General");

                        ui.horizontal(|ui| {
                            ui.label("Hotkey:");
                            ui.text_edit_singleline(&mut cfg.hotkey);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Model Size:");
                            egui::ComboBox::from_label("")
                                .selected_text(&cfg.model_size)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut cfg.model_size, "tiny".to_string(), "Tiny (75MB)");
                                    ui.selectable_value(&mut cfg.model_size, "base".to_string(), "Base (142MB)");
                                    ui.selectable_value(&mut cfg.model_size, "small".to_string(), "Small (466MB)");
                                    ui.selectable_value(&mut cfg.model_size, "medium".to_string(), "Medium (1.5GB)");
                                });
                        });
                    });

                    ui.add_space(10.0);

                    // Audio Settings
                    ui.group(|ui| {
                        ui.label("Audio");

                        ui.checkbox(&mut cfg.noise_reduction, "Noise Reduction");
                        ui.checkbox(&mut cfg.voice_activity_detection, "Voice Activity Detection");
                    });

                    ui.add_space(10.0);

                    // Output Settings
                    ui.group(|ui| {
                        ui.label("Output");

                        ui.checkbox(&mut cfg.copy_to_clipboard, "Copy to Clipboard");
                        ui.checkbox(&mut cfg.auto_capitalize, "Auto Capitalize");
                    });

                    ui.add_space(10.0);

                    // Advanced Settings (collapsible)
                    ui.collapsing("Advanced", |ui| {
                        ui.checkbox(&mut cfg.gpu_acceleration, "GPU Acceleration");
                        ui.horizontal(|ui| {
                            ui.label("Sample Rate:");
                            ui.add(egui::DragValue::new(&mut cfg.sample_rate).speed(1000));
                        });
                    });
                });

                ui.separator();

                // Buttons
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        // Save and close
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    if ui.button("Cancel").clicked() {
                        // Reset and close
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Reset to Defaults").clicked() {
                            *cfg = Config::default();
                        }
                    });
                });
            });
        })?;

        let final_config = config.lock().unwrap().clone();
        Ok(final_config)
    }
}

/// Launch settings window (non-blocking)
pub fn launch_settings_window(config: Config) -> Result<()> {
    log::info!("Launching settings window");

    let window = SettingsWindow::new(config);

    // Spawn in separate thread to avoid blocking
    std::thread::spawn(move || {
        match window.show() {
            Ok(new_config) => {
                log::info!("Settings saved");
                // TODO: Apply new config
                let _ = new_config.save();
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
        let window = SettingsWindow::new(config);
        assert!(!window.show_advanced);
    }
}

//! Windows system tray implementation
//!
//! Provides system tray functionality for Windows 10/11

use tray_icon::{Icon, TrayIcon, TrayIconBuilder};
use anyhow::{Context, Result};

use super::menu::{create_menu, AppState, MenuItems};

/// Windows system tray manager
pub struct WindowsTray {
    tray: TrayIcon,
    menu_items: MenuItems,
    current_state: AppState,
    icon_idle: Icon,
    icon_recording: Icon,
    icon_processing: Icon,
}

impl WindowsTray {
    /// Create a new Windows system tray
    pub fn new() -> Result<Self> {
        log::info!("Initializing Windows system tray");

        // Load icons
        let icon_idle = Self::load_icon("idle")?;
        let icon_recording = Self::load_icon("recording")?;
        let icon_processing = Self::load_icon("processing")?;

        // Create menu
        let (menu, menu_items) = create_menu()
            .context("Failed to create tray menu")?;

        // Build tray icon
        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("VoxAI - Voice Transcription")
            .with_icon(icon_idle.clone())
            .build()
            .context("Failed to build tray icon")?;

        log::info!("Windows system tray initialized successfully");

        Ok(Self {
            tray,
            menu_items,
            current_state: AppState::Idle,
            icon_idle,
            icon_recording,
            icon_processing,
        })
    }

    /// Load an icon from assets
    fn load_icon(name: &str) -> Result<Icon> {
        // Try to load .ico file
        let icon_path = format!("assets/icons/windows/{}.ico", name);

        if std::path::Path::new(&icon_path).exists() {
            Icon::from_path(&icon_path, None)
                .with_context(|| format!("Failed to load icon from {}", icon_path))
        } else {
            log::warn!("Icon file not found: {}, using placeholder", icon_path);
            // Create a placeholder RGBA icon (16x16)
            let color = match name {
                "recording" => [255, 0, 0, 255],   // Red
                "processing" => [52, 152, 219, 255], // Blue
                _ => [107, 107, 107, 255],         // Gray
            };
            let rgba = color.repeat(16 * 16);
            Icon::from_rgba(rgba.to_vec(), 16, 16)
                .context("Failed to create placeholder icon")
        }
    }

    /// Update the tray icon and menu for a new state
    pub fn set_state(&mut self, state: AppState) -> Result<()> {
        if state == self.current_state {
            return Ok(());
        }

        log::info!("Updating Windows tray state: {:?} -> {:?}", self.current_state, state);

        // Update icon
        let icon = match state {
            AppState::Idle => &self.icon_idle,
            AppState::Recording => &self.icon_recording,
            AppState::Processing => &self.icon_processing,
        };

        self.tray.set_icon(Some(icon.clone()))
            .context("Failed to update tray icon")?;

        // Update menu
        super::menu::update_menu_for_state(&self.menu_items, state)?;

        self.current_state = state;
        Ok(())
    }

    /// Get the current state
    pub fn current_state(&self) -> AppState {
        self.current_state
    }

    /// Show a notification
    pub fn show_notification(&self, title: &str, message: &str) -> Result<()> {
        log::info!("Windows notification: {} - {}", title, message);

        #[cfg(target_os = "windows")]
        {
            use notify_rust::Notification;
            Notification::new()
                .summary(title)
                .body(message)
                .icon("voxai")
                .timeout(3000)
                .show()
                .context("Failed to show notification")?;
        }

        Ok(())
    }
}

impl Drop for WindowsTray {
    fn drop(&mut self) {
        log::info!("Shutting down Windows system tray");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_loading() {
        let result = WindowsTray::load_icon("idle");
        // Should succeed with placeholder
        assert!(result.is_ok());
    }

    #[test]
    fn test_state_transitions() {
        let states = [AppState::Idle, AppState::Recording, AppState::Processing];
        for state in states {
            assert_eq!(state, state);
        }
    }
}

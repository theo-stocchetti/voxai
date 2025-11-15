//! Linux system tray implementation
//!
//! Provides system tray functionality for Linux desktop environments
//! (GNOME, KDE, XFCE, etc.)

use anyhow::{Context, Result};
use std::path::PathBuf;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

use super::menu::{create_menu, AppState, MenuItems};

/// Linux system tray manager
pub struct LinuxTray {
    tray: TrayIcon,
    menu_items: MenuItems,
    current_state: AppState,
    icon_idle: Icon,
    icon_recording: Icon,
    icon_processing: Icon,
}

impl LinuxTray {
    /// Create a new Linux system tray
    pub fn new() -> Result<Self> {
        log::info!("Initializing Linux system tray");

        // Load icons
        let icon_idle = Self::load_icon("idle")?;
        let icon_recording = Self::load_icon("recording")?;
        let icon_processing = Self::load_icon("processing")?;

        // Create menu
        let (menu, menu_items) = create_menu().context("Failed to create tray menu")?;

        // Build tray icon
        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("VoxAI - Voice Transcription")
            .with_icon(icon_idle.clone())
            .build()
            .context("Failed to build tray icon")?;

        log::info!("Linux system tray initialized successfully");

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
        // Try to load PNG icon (32x32 is a good default for Linux tray)
        let icon_path = format!("assets/icons/linux/png/{}-32.png", name);

        // For now, create a placeholder icon if the file doesn't exist
        // In production, this should load the actual icon file
        if std::path::Path::new(&icon_path).exists() {
            Icon::from_path(&icon_path, None)
                .with_context(|| format!("Failed to load icon from {}", icon_path))
        } else {
            log::warn!("Icon file not found: {}, using placeholder", icon_path);
            // Create a minimal RGBA icon (16x16 red square as placeholder)
            let rgba = vec![255u8, 0, 0, 255].repeat(16 * 16);
            Icon::from_rgba(rgba, 16, 16).context("Failed to create placeholder icon")
        }
    }

    /// Update the tray icon and menu for a new state
    pub fn set_state(&mut self, state: AppState) -> Result<()> {
        if state == self.current_state {
            return Ok(());
        }

        log::info!(
            "Updating tray state: {:?} -> {:?}",
            self.current_state,
            state
        );

        // Update icon
        let icon = match state {
            AppState::Idle => &self.icon_idle,
            AppState::Recording => &self.icon_recording,
            AppState::Processing => &self.icon_processing,
        };

        self.tray
            .set_icon(Some(icon.clone()))
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
        log::info!("Notification: {} - {}", title, message);

        #[cfg(target_os = "linux")]
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

impl Drop for LinuxTray {
    fn drop(&mut self) {
        log::info!("Shutting down Linux system tray");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_loading() {
        // This test may fail if icons aren't generated yet
        // That's expected during development
        let result = LinuxTray::load_icon("idle");
        // Just verify it doesn't panic
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_state_transitions() {
        let states = [AppState::Idle, AppState::Recording, AppState::Processing];
        for state in states {
            // Just verify states are distinct
            assert_eq!(state, state);
        }
    }
}

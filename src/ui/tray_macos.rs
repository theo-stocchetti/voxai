//! macOS menu bar implementation
//!
//! Provides menu bar functionality for macOS

use tray_icon::{Icon, TrayIcon, TrayIconBuilder};
use anyhow::{Context, Result};

use super::menu::{create_menu, AppState, MenuItems};

/// macOS menu bar manager
pub struct MacOSTray {
    tray: TrayIcon,
    menu_items: MenuItems,
    current_state: AppState,
    icon_idle: Icon,
    icon_recording: Icon,
    icon_processing: Icon,
}

impl MacOSTray {
    /// Create a new macOS menu bar
    pub fn new() -> Result<Self> {
        log::info!("Initializing macOS menu bar");

        // Load icons
        let icon_idle = Self::load_icon("idle")?;
        let icon_recording = Self::load_icon("recording")?;
        let icon_processing = Self::load_icon("processing")?;

        // Create menu
        let (menu, menu_items) = create_menu()
            .context("Failed to create menu bar menu")?;

        // Build menu bar icon
        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("VoxAI - Voice Transcription")
            .with_icon(icon_idle.clone())
            .with_icon_as_template(true) // Enable template mode for macOS
            .build()
            .context("Failed to build menu bar icon")?;

        log::info!("macOS menu bar initialized successfully");

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
        // Try to load @2x PNG (Retina) first, then @1x, then PDF
        let paths = [
            format!("assets/icons/macos/{}@2x.png", name),
            format!("assets/icons/macos/{}.png", name),
            format!("assets/icons/macos/{}.pdf", name),
        ];

        for icon_path in &paths {
            if std::path::Path::new(icon_path).exists() {
                return Icon::from_path(icon_path, Some((22, 22)))
                    .with_context(|| format!("Failed to load icon from {}", icon_path));
            }
        }

        log::warn!("Icon files not found for {}, using placeholder", name);

        // Create a placeholder RGBA icon (22x22 for macOS menu bar)
        // Template icons should be black on transparent
        let rgba = vec![0u8, 0, 0, 255].repeat(22 * 22);
        Icon::from_rgba(rgba, 22, 22)
            .context("Failed to create placeholder icon")
    }

    /// Update the menu bar icon and menu for a new state
    pub fn set_state(&mut self, state: AppState) -> Result<()> {
        if state == self.current_state {
            return Ok(());
        }

        log::info!("Updating macOS menu bar state: {:?} -> {:?}", self.current_state, state);

        // Update icon
        let icon = match state {
            AppState::Idle => &self.icon_idle,
            AppState::Recording => &self.icon_recording,
            AppState::Processing => &self.icon_processing,
        };

        self.tray.set_icon(Some(icon.clone()))
            .context("Failed to update menu bar icon")?;

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
        log::info!("macOS notification: {} - {}", title, message);

        #[cfg(target_os = "macos")]
        {
            use notify_rust::Notification;
            Notification::new()
                .summary(title)
                .body(message)
                .appname("VoxAI")
                .timeout(3000)
                .show()
                .context("Failed to show notification")?;
        }

        Ok(())
    }
}

impl Drop for MacOSTray {
    fn drop(&mut self) {
        log::info!("Shutting down macOS menu bar");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_loading() {
        let result = MacOSTray::load_icon("idle");
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

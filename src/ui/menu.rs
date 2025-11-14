//! Menu management for system tray
//!
//! Provides a cross-platform menu structure for the VoxAI system tray

use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};

/// Application state for menu items
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Idle,
    Recording,
    Processing,
}

/// Menu identifiers
pub struct MenuItems {
    pub start_recording: MenuItem,
    pub settings: MenuItem,
    pub quit: MenuItem,
}

/// Create the system tray menu
pub fn create_menu() -> anyhow::Result<(Menu, MenuItems)> {
    let menu = Menu::new();

    // Start/Stop Recording toggle
    let start_recording = MenuItem::new("Start Recording", true, None);

    // Settings
    let settings = MenuItem::new("Settings", true, None);

    // Separator
    let separator = PredefinedMenuItem::separator();

    // Quit
    let quit = MenuItem::new("Quit VoxAI", true, None);

    // Build menu structure
    menu.append(&start_recording)?;
    menu.append(&settings)?;
    menu.append(&separator)?;
    menu.append(&quit)?;

    let items = MenuItems {
        start_recording,
        settings,
        quit,
    };

    Ok((menu, items))
}

/// Update menu based on application state
pub fn update_menu_for_state(items: &MenuItems, state: AppState) -> anyhow::Result<()> {
    match state {
        AppState::Idle => {
            items.start_recording.set_text("Start Recording");
            items.start_recording.set_enabled(true);
        }
        AppState::Recording => {
            items.start_recording.set_text("Stop Recording");
            items.start_recording.set_enabled(true);
        }
        AppState::Processing => {
            items.start_recording.set_text("Processing...");
            items.start_recording.set_enabled(false);
        }
    }
    Ok(())
}

/// Handle menu events
pub fn handle_menu_event(event: MenuEvent, items: &MenuItems) -> Option<MenuAction> {
    if event.id == items.start_recording.id() {
        Some(MenuAction::ToggleRecording)
    } else if event.id == items.settings.id() {
        Some(MenuAction::OpenSettings)
    } else if event.id == items.quit.id() {
        Some(MenuAction::Quit)
    } else {
        None
    }
}

/// Actions that can be triggered from the menu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    ToggleRecording,
    OpenSettings,
    Quit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_menu() {
        let result = create_menu();
        assert!(result.is_ok());
    }

    #[test]
    fn test_app_states() {
        assert_ne!(AppState::Idle, AppState::Recording);
        assert_ne!(AppState::Recording, AppState::Processing);
    }
}

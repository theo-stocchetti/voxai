//! Global hotkey management module
//!
//! Cross-platform global keyboard shortcuts for triggering recording

use anyhow::Result;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::HotkeyManager;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::HotkeyManager;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::HotkeyManager;

/// Hotkey event - triggered when the global hotkey is pressed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyEvent {
    Pressed,
    Released,
}

/// Hotkey callback type
pub type HotkeyCallback = Box<dyn Fn(HotkeyEvent) + Send + Sync + 'static>;

/// Default hotkey strings per platform
pub mod defaults {
    pub const WINDOWS: &str = "Ctrl+Shift+Space";
    pub const MACOS: &str = "Cmd+Shift+R";
    pub const LINUX: &str = "Ctrl+Shift+R";

    #[cfg(target_os = "windows")]
    pub const DEFAULT: &str = WINDOWS;

    #[cfg(target_os = "macos")]
    pub const DEFAULT: &str = MACOS;

    #[cfg(target_os = "linux")]
    pub const DEFAULT: &str = LINUX;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_hotkeys() {
        // Verify default hotkeys are defined
        assert!(!defaults::DEFAULT.is_empty());
    }
}

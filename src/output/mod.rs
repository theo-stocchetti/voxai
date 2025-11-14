//! Text output module
//!
//! Handles text injection, clipboard operations, and formatting

use anyhow::Result;

pub mod clipboard;
pub mod formatter;

#[cfg(target_os = "windows")]
pub mod text_injector_windows;
#[cfg(target_os = "windows")]
pub use text_injector_windows::TextInjector;

#[cfg(target_os = "macos")]
pub mod text_injector_macos;
#[cfg(target_os = "macos")]
pub use text_injector_macos::TextInjector;

#[cfg(target_os = "linux")]
pub mod text_injector_linux;
#[cfg(target_os = "linux")]
pub use text_injector_linux::TextInjector;

/// Output method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMethod {
    /// Type the text using keyboard simulation
    TypeText,
    /// Copy to clipboard
    Clipboard,
    /// Both: type and copy
    Both,
}

impl Default for OutputMethod {
    fn default() -> Self {
        Self::TypeText
    }
}

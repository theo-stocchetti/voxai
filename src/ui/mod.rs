//! User interface module
//!
//! Platform-specific system tray implementations and settings UI

pub mod menu;

// Platform-specific tray implementations
#[cfg(target_os = "linux")]
pub mod tray_linux;

#[cfg(target_os = "windows")]
pub mod tray_windows;

#[cfg(target_os = "macos")]
pub mod tray_macos;

// Re-export the appropriate tray for the current platform
#[cfg(target_os = "linux")]
pub use tray_linux::LinuxTray as Tray;

#[cfg(target_os = "windows")]
pub use tray_windows::WindowsTray as Tray;

#[cfg(target_os = "macos")]
pub use tray_macos::MacOSTray as Tray;

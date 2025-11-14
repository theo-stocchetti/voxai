//! Linux global hotkey implementation
//!
//! Uses the global-hotkey crate with X11 support
//! Wayland support is limited due to security restrictions

use anyhow::{Context, Result, anyhow};
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use std::sync::Arc;
use std::thread;

use super::{HotkeyCallback, HotkeyEvent};

/// Linux hotkey manager
pub struct HotkeyManager {
    manager: Arc<GlobalHotKeyManager>,
    hotkey: HotKey,
    _listener_thread: Option<thread::JoinHandle<()>>,
    display_server: DisplayServer,
}

/// Display server type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DisplayServer {
    X11,
    Wayland,
    Unknown,
}

impl HotkeyManager {
    /// Create a new Linux hotkey manager
    ///
    /// # Arguments
    /// * `hotkey_str` - Hotkey string (e.g., "Ctrl+Shift+R")
    /// * `callback` - Function called when hotkey is triggered
    ///
    /// # Notes
    /// - Works reliably on X11
    /// - Limited support on Wayland (see documentation)
    /// - May require manual configuration on Wayland
    pub fn new(hotkey_str: &str, callback: HotkeyCallback) -> Result<Self> {
        let display_server = Self::detect_display_server();

        log::info!(
            "Registering Linux global hotkey: {} (Display server: {:?})",
            hotkey_str,
            display_server
        );

        if display_server == DisplayServer::Wayland {
            log::warn!(
                "Wayland detected: Global hotkeys have limited support. \
                 Consider using your desktop environment's keyboard settings \
                 to manually configure the hotkey to call VoxAI."
            );
        }

        let manager = GlobalHotKeyManager::new()
            .context("Failed to create global hotkey manager")?;

        let hotkey = Self::parse_hotkey(hotkey_str)
            .context("Failed to parse hotkey string")?;

        match manager.register(hotkey) {
            Ok(_) => {
                log::info!("Linux hotkey registered successfully: {}", hotkey_str);
            }
            Err(e) => {
                if display_server == DisplayServer::Wayland {
                    log::error!(
                        "Failed to register hotkey on Wayland: {}. \
                         Please configure hotkey manually in your desktop environment settings.",
                        e
                    );
                    return Err(anyhow!(
                        "Hotkey registration failed on Wayland. Please configure manually."
                    ));
                } else {
                    return Err(e).context("Failed to register hotkey - it may already be in use");
                }
            }
        }

        let manager_arc = Arc::new(manager);

        // Spawn listener thread
        let listener_thread = thread::spawn(move || {
            log::debug!("Hotkey listener thread started");

            let receiver = GlobalHotKeyEvent::receiver();

            loop {
                if let Ok(event) = receiver.recv() {
                    log::trace!("Hotkey event received: {:?}", event);

                    let hotkey_event = match event.state {
                        HotKeyState::Pressed => HotkeyEvent::Pressed,
                        HotKeyState::Released => HotkeyEvent::Released,
                    };

                    callback(hotkey_event);
                }
            }
        });

        Ok(Self {
            manager: manager_arc,
            hotkey,
            _listener_thread: Some(listener_thread),
            display_server,
        })
    }

    /// Detect which display server is being used
    fn detect_display_server() -> DisplayServer {
        // Check XDG_SESSION_TYPE environment variable
        if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
            match session_type.to_lowercase().as_str() {
                "x11" => return DisplayServer::X11,
                "wayland" => return DisplayServer::Wayland,
                _ => {}
            }
        }

        // Check WAYLAND_DISPLAY
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            return DisplayServer::Wayland;
        }

        // Check DISPLAY (X11)
        if std::env::var("DISPLAY").is_ok() {
            return DisplayServer::X11;
        }

        DisplayServer::Unknown
    }

    /// Parse hotkey string into HotKey struct
    ///
    /// Examples:
    /// - "Ctrl+Shift+R"
    /// - "Ctrl+Alt+Space"
    /// - "Super+Shift+V"
    fn parse_hotkey(s: &str) -> Result<HotKey> {
        let parts: Vec<&str> = s.split('+').map(|p| p.trim()).collect();

        if parts.is_empty() {
            return Err(anyhow!("Empty hotkey string"));
        }

        let mut modifiers = Modifiers::empty();
        let mut key_code: Option<Code> = None;

        for part in parts {
            match part.to_lowercase().as_str() {
                "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
                "shift" => modifiers |= Modifiers::SHIFT,
                "alt" => modifiers |= Modifiers::ALT,
                "super" | "meta" | "win" => modifiers |= Modifiers::SUPER,

                // Letter keys
                "a" => key_code = Some(Code::KeyA),
                "b" => key_code = Some(Code::KeyB),
                "c" => key_code = Some(Code::KeyC),
                "d" => key_code = Some(Code::KeyD),
                "e" => key_code = Some(Code::KeyE),
                "f" => key_code = Some(Code::KeyF),
                "g" => key_code = Some(Code::KeyG),
                "h" => key_code = Some(Code::KeyH),
                "i" => key_code = Some(Code::KeyI),
                "j" => key_code = Some(Code::KeyJ),
                "k" => key_code = Some(Code::KeyK),
                "l" => key_code = Some(Code::KeyL),
                "m" => key_code = Some(Code::KeyM),
                "n" => key_code = Some(Code::KeyN),
                "o" => key_code = Some(Code::KeyO),
                "p" => key_code = Some(Code::KeyP),
                "q" => key_code = Some(Code::KeyQ),
                "r" => key_code = Some(Code::KeyR),
                "s" => key_code = Some(Code::KeyS),
                "t" => key_code = Some(Code::KeyT),
                "u" => key_code = Some(Code::KeyU),
                "v" => key_code = Some(Code::KeyV),
                "w" => key_code = Some(Code::KeyW),
                "x" => key_code = Some(Code::KeyX),
                "y" => key_code = Some(Code::KeyY),
                "z" => key_code = Some(Code::KeyZ),

                // Special keys
                "space" => key_code = Some(Code::Space),
                "enter" | "return" => key_code = Some(Code::Enter),
                "escape" | "esc" => key_code = Some(Code::Escape),
                "tab" => key_code = Some(Code::Tab),
                "backspace" => key_code = Some(Code::Backspace),

                // Function keys
                "f1" => key_code = Some(Code::F1),
                "f2" => key_code = Some(Code::F2),
                "f3" => key_code = Some(Code::F3),
                "f4" => key_code = Some(Code::F4),
                "f5" => key_code = Some(Code::F5),
                "f6" => key_code = Some(Code::F6),
                "f7" => key_code = Some(Code::F7),
                "f8" => key_code = Some(Code::F8),
                "f9" => key_code = Some(Code::F9),
                "f10" => key_code = Some(Code::F10),
                "f11" => key_code = Some(Code::F11),
                "f12" => key_code = Some(Code::F12),

                _ => return Err(anyhow!("Unknown key: {}", part)),
            }
        }

        let key_code = key_code
            .ok_or_else(|| anyhow!("No key specified in hotkey string"))?;

        Ok(HotKey::new(Some(modifiers), key_code))
    }

    /// Get the display server type
    pub fn display_server(&self) -> &str {
        match self.display_server {
            DisplayServer::X11 => "X11",
            DisplayServer::Wayland => "Wayland",
            DisplayServer::Unknown => "Unknown",
        }
    }
}

impl Drop for HotkeyManager {
    fn drop(&mut self) {
        log::info!("Unregistering Linux hotkey");
        let _ = self.manager.unregister(self.hotkey);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hotkey_ctrl_shift_r() {
        let hotkey = HotkeyManager::parse_hotkey("Ctrl+Shift+R");
        assert!(hotkey.is_ok());
    }

    #[test]
    fn test_parse_hotkey_super_shift_v() {
        let hotkey = HotkeyManager::parse_hotkey("Super+Shift+V");
        assert!(hotkey.is_ok());
    }

    #[test]
    fn test_parse_hotkey_invalid() {
        let hotkey = HotkeyManager::parse_hotkey("Ctrl+Invalid");
        assert!(hotkey.is_err());
    }

    #[test]
    fn test_detect_display_server() {
        // Just verify it doesn't panic
        let _server = HotkeyManager::detect_display_server();
    }
}

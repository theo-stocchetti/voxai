//! macOS global hotkey implementation
//!
//! Uses the global-hotkey crate with macOS Carbon/Cocoa APIs

use anyhow::{Context, Result, anyhow};
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use std::sync::Arc;
use std::thread;

use super::{HotkeyCallback, HotkeyEvent};

/// macOS hotkey manager
pub struct HotkeyManager {
    manager: Arc<GlobalHotKeyManager>,
    hotkey: HotKey,
    _listener_thread: Option<thread::JoinHandle<()>>,
}

impl HotkeyManager {
    /// Create a new macOS hotkey manager
    ///
    /// # Arguments
    /// * `hotkey_str` - Hotkey string (e.g., "Cmd+Shift+R")
    /// * `callback` - Function called when hotkey is triggered
    ///
    /// # Notes
    /// - May require Accessibility permissions in System Settings
    /// - Some system shortcuts cannot be overridden (⌘Q, ⌘W, etc.)
    pub fn new(hotkey_str: &str, callback: HotkeyCallback) -> Result<Self> {
        log::info!("Registering macOS global hotkey: {}", hotkey_str);

        let manager = GlobalHotKeyManager::new()
            .context("Failed to create global hotkey manager. Check Accessibility permissions in System Settings > Privacy & Security > Accessibility")?;

        let hotkey = Self::parse_hotkey(hotkey_str)
            .context("Failed to parse hotkey string")?;

        manager.register(hotkey)
            .context("Failed to register hotkey - it may already be in use or require Accessibility permissions")?;

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

        log::info!("macOS hotkey registered successfully: {}", hotkey_str);

        Ok(Self {
            manager: manager_arc,
            hotkey,
            _listener_thread: Some(listener_thread),
        })
    }

    /// Parse hotkey string into HotKey struct
    ///
    /// Examples:
    /// - "Cmd+Shift+R"
    /// - "Ctrl+Alt+Space"
    /// - "Opt+Shift+V"
    fn parse_hotkey(s: &str) -> Result<HotKey> {
        let parts: Vec<&str> = s.split('+').map(|p| p.trim()).collect();

        if parts.is_empty() {
            return Err(anyhow!("Empty hotkey string"));
        }

        let mut modifiers = Modifiers::empty();
        let mut key_code: Option<Code> = None;

        for part in parts {
            match part.to_lowercase().as_str() {
                "cmd" | "command" | "super" | "meta" => modifiers |= Modifiers::SUPER,
                "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
                "shift" => modifiers |= Modifiers::SHIFT,
                "opt" | "option" | "alt" => modifiers |= Modifiers::ALT,

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
                "backspace" | "delete" => key_code = Some(Code::Backspace),

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
}

impl Drop for HotkeyManager {
    fn drop(&mut self) {
        log::info!("Unregistering macOS hotkey");
        let _ = self.manager.unregister(self.hotkey);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hotkey_cmd_shift_r() {
        let hotkey = HotkeyManager::parse_hotkey("Cmd+Shift+R");
        assert!(hotkey.is_ok());
    }

    #[test]
    fn test_parse_hotkey_ctrl_opt_space() {
        let hotkey = HotkeyManager::parse_hotkey("Ctrl+Opt+Space");
        assert!(hotkey.is_ok());
    }

    #[test]
    fn test_parse_hotkey_invalid() {
        let hotkey = HotkeyManager::parse_hotkey("Cmd+Invalid");
        assert!(hotkey.is_err());
    }
}

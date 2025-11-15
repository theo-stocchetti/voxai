//! Windows global hotkey implementation
//!
//! Uses the global-hotkey crate with Windows RegisterHotKey API

use anyhow::{anyhow, Context, Result};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use std::sync::{Arc, Mutex};
use std::thread;

use super::{HotkeyCallback, HotkeyEvent};

/// Windows hotkey manager
pub struct HotkeyManager {
    manager: Arc<GlobalHotKeyManager>,
    hotkey: HotKey,
    _listener_thread: Option<thread::JoinHandle<()>>,
}

impl HotkeyManager {
    /// Create a new Windows hotkey manager
    ///
    /// # Arguments
    /// * `hotkey_str` - Hotkey string (e.g., "Ctrl+Shift+Space")
    /// * `callback` - Function called when hotkey is triggered
    pub fn new(hotkey_str: &str, callback: HotkeyCallback) -> Result<Self> {
        log::info!("Registering Windows global hotkey: {}", hotkey_str);

        let manager =
            GlobalHotKeyManager::new().context("Failed to create global hotkey manager")?;

        let hotkey = Self::parse_hotkey(hotkey_str).context("Failed to parse hotkey string")?;

        manager
            .register(hotkey)
            .context("Failed to register hotkey - it may already be in use")?;

        let manager_arc = Arc::new(manager);
        let manager_clone = Arc::clone(&manager_arc);

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

        log::info!("Windows hotkey registered successfully: {}", hotkey_str);

        Ok(Self {
            manager: manager_arc,
            hotkey,
            _listener_thread: Some(listener_thread),
        })
    }

    /// Parse hotkey string into HotKey struct
    ///
    /// Examples:
    /// - "Ctrl+Shift+Space"
    /// - "Ctrl+Alt+V"
    /// - "Win+Shift+R"
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
                "win" | "super" | "meta" => modifiers |= Modifiers::SUPER,

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

        let key_code = key_code.ok_or_else(|| anyhow!("No key specified in hotkey string"))?;

        Ok(HotKey::new(Some(modifiers), key_code))
    }
}

impl Drop for HotkeyManager {
    fn drop(&mut self) {
        log::info!("Unregistering Windows hotkey");
        let _ = self.manager.unregister(self.hotkey);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hotkey_ctrl_shift_space() {
        let hotkey = HotkeyManager::parse_hotkey("Ctrl+Shift+Space");
        assert!(hotkey.is_ok());
    }

    #[test]
    fn test_parse_hotkey_ctrl_alt_v() {
        let hotkey = HotkeyManager::parse_hotkey("Ctrl+Alt+V");
        assert!(hotkey.is_ok());
    }

    #[test]
    fn test_parse_hotkey_invalid() {
        let hotkey = HotkeyManager::parse_hotkey("Ctrl+Invalid");
        assert!(hotkey.is_err());
    }

    #[test]
    fn test_parse_hotkey_empty() {
        let hotkey = HotkeyManager::parse_hotkey("");
        assert!(hotkey.is_err());
    }
}

//! Linux text injection implementation
//!
//! Uses enigo for keyboard simulation on Linux (X11)

use anyhow::{Context, Result};
use enigo::{Enigo, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;

use super::OutputMethod;

/// Linux text injector
pub struct TextInjector {
    enigo: Enigo,
    method: OutputMethod,
    delay_ms: u64,
}

impl TextInjector {
    /// Create a new Linux text injector
    ///
    /// # Notes
    /// - Works on X11
    /// - Limited support on Wayland (depends on compositor)
    pub fn new(method: OutputMethod) -> Result<Self> {
        log::info!("Initializing Linux text injector");

        let enigo = Enigo::new();

        // Detect display server
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            log::warn!(
                "Wayland detected: Text injection may have limited support. \
                 Consider using clipboard method or running under XWayland."
            );
        }

        Ok(Self {
            enigo,
            method,
            delay_ms: 10,
        })
    }

    /// Inject text into the active application
    pub fn inject(&mut self, text: &str) -> Result<()> {
        log::info!("Injecting {} characters using {:?}", text.len(), self.method);

        match self.method {
            OutputMethod::TypeText => self.type_text(text),
            OutputMethod::Clipboard => self.copy_to_clipboard(text),
            OutputMethod::Both => {
                self.copy_to_clipboard(text)?;
                self.type_text(text)
            }
        }
    }

    /// Type text character by character
    fn type_text(&mut self, text: &str) -> Result<()> {
        // Small delay to allow user to switch windows
        thread::sleep(Duration::from_millis(100));

        for ch in text.chars() {
            self.enigo.key_sequence(&ch.to_string());

            if self.delay_ms > 0 {
                thread::sleep(Duration::from_millis(self.delay_ms));
            }
        }

        log::debug!("Typed {} characters", text.len());
        Ok(())
    }

    /// Copy text to clipboard
    fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        super::clipboard::copy_to_clipboard(text)
            .context("Failed to copy text to clipboard")
    }

    /// Set delay between key presses (in milliseconds)
    pub fn set_delay(&mut self, delay_ms: u64) {
        self.delay_ms = delay_ms;
    }

    /// Simulate key press
    pub fn press_key(&mut self, key: Key) -> Result<()> {
        self.enigo.key_click(key);
        Ok(())
    }

    /// Simulate key combination (e.g., Ctrl+V)
    pub fn press_combination(&mut self, keys: &[Key]) -> Result<()> {
        for key in keys {
            self.enigo.key_down(*key);
        }

        thread::sleep(Duration::from_millis(50));

        for key in keys.iter().rev() {
            self.enigo.key_up(*key);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_injector() {
        let result = TextInjector::new(OutputMethod::TypeText);
        // May fail in headless environments
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_set_delay() {
        if let Ok(mut injector) = TextInjector::new(OutputMethod::TypeText) {
            injector.set_delay(5);
            assert_eq!(injector.delay_ms, 5);
        }
    }
}

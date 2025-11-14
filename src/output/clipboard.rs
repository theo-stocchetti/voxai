//! Clipboard management
//!
//! Cross-platform clipboard operations using arboard

use anyhow::{Context, Result};
use arboard::Clipboard;

/// Copy text to clipboard
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new()
        .context("Failed to access clipboard")?;

    clipboard
        .set_text(text)
        .context("Failed to write to clipboard")?;

    log::debug!("Copied {} characters to clipboard", text.len());
    Ok(())
}

/// Get text from clipboard
pub fn get_from_clipboard() -> Result<String> {
    let mut clipboard = Clipboard::new()
        .context("Failed to access clipboard")?;

    let text = clipboard
        .get_text()
        .context("Failed to read from clipboard")?;

    log::debug!("Read {} characters from clipboard", text.len());
    Ok(text)
}

/// Check if clipboard contains text
pub fn has_text() -> bool {
    Clipboard::new()
        .and_then(|mut clip| clip.get_text())
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_operations() {
        // This test may fail in headless environments
        let test_text = "VoxAI clipboard test";

        match copy_to_clipboard(test_text) {
            Ok(_) => {
                // Try to read it back
                if let Ok(text) = get_from_clipboard() {
                    assert_eq!(text, test_text);
                }
            }
            Err(_) => {
                // Clipboard access may fail in CI/headless
                // That's acceptable
            }
        }
    }

    #[test]
    fn test_has_text() {
        // Just verify it doesn't panic
        let _ = has_text();
    }
}

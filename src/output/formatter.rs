//! Text formatting utilities
//!
//! Post-processing for transcribed text

use anyhow::Result;

/// Formatting options
#[derive(Debug, Clone)]
pub struct FormattingOptions {
    /// Add punctuation if missing
    pub add_punctuation: bool,
    /// Capitalize first letter of sentences
    pub capitalize_sentences: bool,
    /// Remove extra whitespace
    pub trim_whitespace: bool,
    /// Add final period if missing
    pub add_final_period: bool,
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            add_punctuation: false, // Whisper usually adds punctuation
            capitalize_sentences: true,
            trim_whitespace: true,
            add_final_period: false,
        }
    }
}

/// Format transcribed text
pub fn format_text(text: &str, options: &FormattingOptions) -> Result<String> {
    let mut formatted = text.to_string();

    // Trim whitespace
    if options.trim_whitespace {
        formatted = formatted.trim().to_string();
        // Collapse multiple spaces
        while formatted.contains("  ") {
            formatted = formatted.replace("  ", " ");
        }
    }

    // Capitalize first letter
    if options.capitalize_sentences && !formatted.is_empty() {
        let mut chars: Vec<char> = formatted.chars().collect();
        if let Some(first) = chars.first_mut() {
            *first = first.to_uppercase().next().unwrap_or(*first);
        }
        formatted = chars.into_iter().collect();
    }

    // Add final period
    if options.add_final_period && !formatted.is_empty() {
        let last_char = formatted.chars().last().unwrap();
        if !matches!(last_char, '.' | '!' | '?') {
            formatted.push('.');
        }
    }

    log::trace!("Formatted text: '{}' -> '{}'", text, formatted);
    Ok(formatted)
}

/// Common text transformations
pub mod transforms {
    /// Remove filler words
    pub fn remove_fillers(text: &str) -> String {
        let fillers = ["um", "uh", "like", "you know", "kind of", "sort of"];
        let mut result = text.to_string();

        for filler in &fillers {
            // Remove with surrounding spaces
            result = result.replace(&format!(" {} ", filler), " ");
            result = result.replace(&format!("{} ", filler), "");
            result = result.replace(&format!(" {}", filler), "");
        }

        result.trim().to_string()
    }

    /// Convert to title case
    pub fn to_title_case(text: &str) -> String {
        text.split_whitespace()
            .map(|word| {
                let mut chars: Vec<char> = word.chars().collect();
                if let Some(first) = chars.first_mut() {
                    *first = first.to_uppercase().next().unwrap_or(*first);
                }
                chars.into_iter().collect::<String>()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Convert to uppercase
    pub fn to_upper(text: &str) -> String {
        text.to_uppercase()
    }

    /// Convert to lowercase
    pub fn to_lower(text: &str) -> String {
        text.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_basic() {
        let options = FormattingOptions::default();
        let result = format_text("hello world", &options).unwrap();
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn test_format_with_period() {
        let mut options = FormattingOptions::default();
        options.add_final_period = true;
        let result = format_text("hello world", &options).unwrap();
        assert_eq!(result, "Hello world.");
    }

    #[test]
    fn test_format_trim() {
        let options = FormattingOptions::default();
        let result = format_text("  hello   world  ", &options).unwrap();
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn test_remove_fillers() {
        let text = "um hello uh world like you know";
        let result = transforms::remove_fillers(text);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_title_case() {
        let result = transforms::to_title_case("hello world foo bar");
        assert_eq!(result, "Hello World Foo Bar");
    }
}

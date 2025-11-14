//! Multi-language support for transcription
//!
//! Manages language detection and selection for Whisper models

use serde::{Deserialize, Serialize};

/// Supported languages for transcription
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Language {
    /// Automatic language detection
    Auto,
    /// English
    English,
    /// French
    French,
    /// Spanish
    Spanish,
    /// German
    German,
    /// Italian
    Italian,
    /// Portuguese
    Portuguese,
    /// Chinese
    Chinese,
    /// Japanese
    Japanese,
    /// Korean
    Korean,
    /// Custom language code
    Custom(String),
}

impl Language {
    /// Convert language to ISO 639-1 code
    pub fn to_code(&self) -> String {
        match self {
            Language::Auto => "auto".to_string(),
            Language::English => "en".to_string(),
            Language::French => "fr".to_string(),
            Language::Spanish => "es".to_string(),
            Language::German => "de".to_string(),
            Language::Italian => "it".to_string(),
            Language::Portuguese => "pt".to_string(),
            Language::Chinese => "zh".to_string(),
            Language::Japanese => "ja".to_string(),
            Language::Korean => "ko".to_string(),
            Language::Custom(code) => code.clone(),
        }
    }

    /// Parse language from code
    pub fn from_code(code: &str) -> Self {
        match code {
            "auto" => Language::Auto,
            "en" => Language::English,
            "fr" => Language::French,
            "es" => Language::Spanish,
            "de" => Language::German,
            "it" => Language::Italian,
            "pt" => Language::Portuguese,
            "zh" => Language::Chinese,
            "ja" => Language::Japanese,
            "ko" => Language::Korean,
            _ => Language::Custom(code.to_string()),
        }
    }

    /// Get all supported languages
    pub fn all() -> Vec<Language> {
        vec![
            Language::Auto,
            Language::English,
            Language::French,
            Language::Spanish,
            Language::German,
            Language::Italian,
            Language::Portuguese,
            Language::Chinese,
            Language::Japanese,
            Language::Korean,
        ]
    }

    /// Get display name
    pub fn display_name(&self) -> String {
        match self {
            Language::Auto => "Auto-detect".to_string(),
            Language::English => "English".to_string(),
            Language::French => "Français".to_string(),
            Language::Spanish => "Español".to_string(),
            Language::German => "Deutsch".to_string(),
            Language::Italian => "Italiano".to_string(),
            Language::Portuguese => "Português".to_string(),
            Language::Chinese => "中文".to_string(),
            Language::Japanese => "日本語".to_string(),
            Language::Korean => "한국어".to_string(),
            Language::Custom(code) => code.clone(),
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Auto
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_to_code() {
        assert_eq!(Language::English.to_code(), "en");
        assert_eq!(Language::French.to_code(), "fr");
        assert_eq!(Language::Auto.to_code(), "auto");
    }

    #[test]
    fn test_language_from_code() {
        assert_eq!(Language::from_code("en"), Language::English);
        assert_eq!(Language::from_code("fr"), Language::French);
        assert_eq!(Language::from_code("auto"), Language::Auto);
    }

    #[test]
    fn test_custom_language() {
        let custom = Language::Custom("custom".to_string());
        assert_eq!(custom.to_code(), "custom");
    }

    #[test]
    fn test_all_languages() {
        let languages = Language::all();
        assert!(languages.len() >= 10);
        assert!(languages.contains(&Language::English));
    }
}

//! Whisper model management
//!
//! Handles different Whisper model sizes and their specifications

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Available Whisper model sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelSize {
    /// Tiny model (~75 MB) - fastest, least accurate
    Tiny,
    /// Base model (~142 MB) - good balance
    Base,
    /// Small model (~466 MB) - better accuracy
    Small,
    /// Medium model (~1.5 GB) - high accuracy, requires GPU
    Medium,
}

impl ModelSize {
    /// Get the filename for this model
    pub fn filename(&self) -> &'static str {
        match self {
            Self::Tiny => "ggml-tiny.bin",
            Self::Base => "ggml-base.bin",
            Self::Small => "ggml-small.bin",
            Self::Medium => "ggml-medium.bin",
        }
    }

    /// Get the expected file size in bytes
    pub fn file_size(&self) -> u64 {
        match self {
            Self::Tiny => 75_000_000,
            Self::Base => 142_000_000,
            Self::Small => 466_000_000,
            Self::Medium => 1_500_000_000,
        }
    }

    /// Get the download URL for this model
    pub fn download_url(&self) -> String {
        format!(
            "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}",
            self.filename()
        )
    }

    /// Get the human-readable name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Tiny => "Tiny",
            Self::Base => "Base",
            Self::Small => "Small",
            Self::Medium => "Medium",
        }
    }
}

impl std::fmt::Display for ModelSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl std::str::FromStr for ModelSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tiny" => Ok(Self::Tiny),
            "base" => Ok(Self::Base),
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            _ => Err(format!("Unknown model size: {}", s)),
        }
    }
}

/// Get the default model directory path
pub fn get_models_dir() -> anyhow::Result<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    let models_dir = home_dir.join(".voxai").join("models");

    // Create directory if it doesn't exist
    if !models_dir.exists() {
        std::fs::create_dir_all(&models_dir)?;
    }

    Ok(models_dir)
}

/// Get the full path for a specific model
pub fn get_model_path(model: ModelSize) -> anyhow::Result<PathBuf> {
    let models_dir = get_models_dir()?;
    Ok(models_dir.join(model.filename()))
}

/// Check if a model is already downloaded
pub fn is_model_downloaded(model: ModelSize) -> anyhow::Result<bool> {
    let model_path = get_model_path(model)?;
    Ok(model_path.exists())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_size_parsing() {
        assert_eq!("base".parse::<ModelSize>().unwrap(), ModelSize::Base);
        assert_eq!("TINY".parse::<ModelSize>().unwrap(), ModelSize::Tiny);
        assert!("invalid".parse::<ModelSize>().is_err());
    }

    #[test]
    fn test_model_filenames() {
        assert_eq!(ModelSize::Tiny.filename(), "ggml-tiny.bin");
        assert_eq!(ModelSize::Base.filename(), "ggml-base.bin");
    }

    #[test]
    fn test_models_dir() {
        let dir = get_models_dir().unwrap();
        assert!(dir.to_string_lossy().contains(".voxai"));
    }
}

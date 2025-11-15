//! Whisper model management
//!
//! Handles different Whisper model sizes and their specifications

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use sysinfo::System;

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

    /// Get the expected file size in bytes (approximate)
    pub fn file_size(&self) -> u64 {
        match self {
            Self::Tiny => 75_852_960,      // ~75 MB
            Self::Base => 141_925_216,     // ~142 MB
            Self::Small => 466_179_104,    // ~466 MB
            Self::Medium => 1_533_628_064, // ~1.5 GB
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

    /// Get the expected SHA256 checksum for this model
    /// Note: These are placeholder checksums. Real checksums should be obtained from
    /// the official Whisper.cpp repository or HuggingFace.
    pub fn expected_sha256(&self) -> Option<&'static str> {
        // TODO: Add real SHA256 checksums from whisper.cpp repo
        // For now, return None to skip verification
        // In production, these should be verified checksums
        None
    }

    /// Get RAM requirement in GB
    pub fn ram_requirement(&self) -> u64 {
        match self {
            Self::Tiny => 1,
            Self::Base => 1,
            Self::Small => 2,
            Self::Medium => 5,
        }
    }

    /// Get expected speed multiplier (X times realtime on average CPU)
    pub fn speed_multiplier(&self) -> f32 {
        match self {
            Self::Tiny => 10.0,
            Self::Base => 7.0,
            Self::Small => 4.0,
            Self::Medium => 2.0,
        }
    }

    /// Get all available models
    pub fn all() -> &'static [ModelSize] {
        &[
            ModelSize::Tiny,
            ModelSize::Base,
            ModelSize::Small,
            ModelSize::Medium,
        ]
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

/// Automatically select the best model based on system capabilities
pub fn auto_select_model() -> ModelSize {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_count = sys.cpus().len();
    let total_ram_gb = sys.total_memory() / (1024 * 1024 * 1024);

    log::info!(
        "System capabilities: {} CPUs, {} GB RAM",
        cpu_count,
        total_ram_gb
    );

    // GPU detection (simple heuristic for now)
    let has_gpu = detect_gpu_available();

    // Selection logic based on system capabilities
    let selected_model = if has_gpu && total_ram_gb >= 8 {
        // Powerful system with GPU - use Medium model
        ModelSize::Medium
    } else if cpu_count >= 8 && total_ram_gb >= 4 {
        // Powerful CPU, enough RAM - use Small model
        ModelSize::Small
    } else if cpu_count >= 4 && total_ram_gb >= 2 {
        // Average system - use Base model
        ModelSize::Base
    } else {
        // Low-end system - use Tiny model
        ModelSize::Tiny
    };

    log::info!("Auto-selected model: {}", selected_model);
    selected_model
}

/// Detect if GPU is available (simple heuristic)
fn detect_gpu_available() -> bool {
    // Check for CUDA (NVIDIA)
    #[cfg(feature = "cuda")]
    {
        if cuda_is_available() {
            log::info!("CUDA GPU detected");
            return true;
        }
    }

    // Check for Apple Silicon (Metal)
    #[cfg(target_os = "macos")]
    {
        if is_apple_silicon() {
            log::info!("Apple Silicon detected (Metal support)");
            return true;
        }
    }

    // TODO: Add OpenCL detection for AMD/Intel GPUs
    false
}

/// Check if running on Apple Silicon
#[cfg(target_os = "macos")]
fn is_apple_silicon() -> bool {
    use std::process::Command;

    if let Ok(output) = Command::new("sysctl")
        .arg("-n")
        .arg("machdep.cpu.brand_string")
        .output()
    {
        let cpu_brand = String::from_utf8_lossy(&output.stdout);
        return cpu_brand.contains("Apple");
    }

    false
}

/// Check if CUDA is available (placeholder)
#[cfg(feature = "cuda")]
fn cuda_is_available() -> bool {
    // TODO: Implement proper CUDA detection
    // This would use cudart or similar to check for CUDA devices
    false
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
        assert_eq!(ModelSize::Small.filename(), "ggml-small.bin");
        assert_eq!(ModelSize::Medium.filename(), "ggml-medium.bin");
    }

    #[test]
    fn test_models_dir() {
        let dir = get_models_dir().unwrap();
        assert!(dir.to_string_lossy().contains(".voxai"));
    }

    #[test]
    fn test_auto_select_model() {
        // Should not panic and return a valid model
        let model = auto_select_model();
        println!("Auto-selected model: {}", model);
        assert!(ModelSize::all().contains(&model));
    }

    #[test]
    fn test_model_properties() {
        assert_eq!(ModelSize::Tiny.ram_requirement(), 1);
        assert_eq!(ModelSize::Medium.ram_requirement(), 5);
        assert!(ModelSize::Tiny.speed_multiplier() > ModelSize::Medium.speed_multiplier());
    }

    #[test]
    fn test_all_models() {
        let all = ModelSize::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&ModelSize::Tiny));
        assert!(all.contains(&ModelSize::Medium));
    }
}

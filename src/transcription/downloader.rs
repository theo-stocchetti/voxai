//! Model downloader with progress indication
//!
//! Downloads Whisper models from HuggingFace

use super::models::{get_model_path, ModelSize};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Download a Whisper model
pub async fn download_model(model: ModelSize) -> Result<std::path::PathBuf> {
    let model_path = get_model_path(model)?;

    // Skip if already exists
    if model_path.exists() {
        info!("Model {} already exists at {:?}", model, model_path);
        return Ok(model_path);
    }

    info!("Downloading {} model...", model);
    info!("URL: {}", model.download_url());

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(600))
        .build()?;

    let response = client
        .get(&model.download_url())
        .send()
        .await?
        .error_for_status()?;

    let total_size = response
        .content_length()
        .ok_or_else(|| anyhow::anyhow!("Failed to get content length"))?;

    // Create progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{msg}\n[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
            )?
            .progress_chars("█▓▒░ "),
    );
    pb.set_message(format!("Downloading {} model", model));

    // Create temporary file
    let temp_path = model_path.with_extension("tmp");
    let mut file = File::create(&temp_path)
        .await
        .context("Failed to create temporary file")?;

    // Download with progress
    let mut downloaded = 0u64;
    let mut hasher = Sha256::new();
    let mut stream = response.bytes_stream();

    use futures_util::StreamExt;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        hasher.update(&chunk);
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    file.flush().await?;
    drop(file);

    pb.finish_with_message(format!("{} model downloaded successfully", model));

    // Calculate checksum
    let hash = hasher.finalize();
    let checksum = format!("{:x}", hash);
    info!("SHA256: {}", checksum);

    // Rename temp file to final name
    tokio::fs::rename(&temp_path, &model_path)
        .await
        .context("Failed to rename downloaded file")?;

    info!("Model saved to: {:?}", model_path);

    Ok(model_path)
}

/// Verify model file integrity
pub async fn verify_model(model: ModelSize) -> Result<bool> {
    let model_path = get_model_path(model)?;

    if !model_path.exists() {
        return Ok(false);
    }

    // Check file size
    let metadata = tokio::fs::metadata(&model_path).await?;
    let actual_size = metadata.len();
    let expected_size = model.file_size();

    // Allow 10% variance in file size
    let size_diff = (actual_size as i64 - expected_size as i64).abs();
    let size_tolerance = (expected_size as f64 * 0.1) as i64;

    if size_diff > size_tolerance {
        warn!(
            "Model {} size mismatch: expected ~{} bytes, got {} bytes",
            model, expected_size, actual_size
        );
        return Ok(false);
    }

    Ok(true)
}

/// Delete a downloaded model
pub async fn delete_model(model: ModelSize) -> Result<()> {
    let model_path = get_model_path(model)?;

    if model_path.exists() {
        tokio::fs::remove_file(&model_path).await?;
        info!("Deleted model: {:?}", model_path);
    }

    Ok(())
}

/// List all downloaded models
pub async fn list_downloaded_models() -> Result<Vec<ModelSize>> {
    let mut models = Vec::new();

    for model in &[
        ModelSize::Tiny,
        ModelSize::Base,
        ModelSize::Small,
        ModelSize::Medium,
    ] {
        let model_path = get_model_path(*model)?;
        if model_path.exists() {
            models.push(*model);
        }
    }

    Ok(models)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_models() {
        let models = list_downloaded_models().await.unwrap();
        // Just ensure it doesn't crash
        println!("Downloaded models: {:?}", models);
    }
}

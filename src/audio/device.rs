//! Audio device enumeration and management

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait};

/// Device information
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub is_default: bool,
    pub supported_sample_rates: Vec<u32>,
    pub supported_channels: Vec<u16>,
}

/// List available audio input devices
pub fn list_devices() -> Result<Vec<DeviceInfo>> {
    let host = cpal::default_host();

    let default_device = host.default_input_device();
    let default_name = default_device.as_ref().and_then(|d| d.name().ok());

    let mut devices = Vec::new();

    for device in host
        .input_devices()
        .context("Failed to enumerate input devices")?
    {
        let name = device
            .name()
            .unwrap_or_else(|_| "Unknown Device".to_string());
        let is_default = Some(&name) == default_name.as_ref();

        // Get supported configurations
        let supported_configs: Vec<_> = device
            .supported_input_configs()
            .map(|configs| configs.collect())
            .unwrap_or_default();

        let supported_sample_rates: Vec<u32> = supported_configs
            .iter()
            .flat_map(|config| vec![config.min_sample_rate().0, config.max_sample_rate().0])
            .collect();

        let supported_channels: Vec<u16> = supported_configs
            .iter()
            .map(|config| config.channels())
            .collect();

        devices.push(DeviceInfo {
            name,
            is_default,
            supported_sample_rates,
            supported_channels,
        });
    }

    Ok(devices)
}

/// Get the default input device
pub fn get_default_device() -> Result<cpal::Device> {
    let host = cpal::default_host();
    host.default_input_device()
        .context("No default input device available")
}

/// Find device by name
pub fn find_device_by_name(name: &str) -> Result<Option<cpal::Device>> {
    let host = cpal::default_host();

    for device in host
        .input_devices()
        .context("Failed to enumerate input devices")?
    {
        if let Ok(device_name) = device.name() {
            if device_name == name {
                return Ok(Some(device));
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_devices() {
        // This test may fail in CI environments without audio devices
        match list_devices() {
            Ok(devices) => {
                println!("Found {} audio input devices", devices.len());
                for device in devices {
                    println!("  - {} (default: {})", device.name, device.is_default);
                }
            }
            Err(e) => {
                println!("Warning: Could not enumerate devices: {}", e);
            }
        }
    }

    #[test]
    fn test_get_default_device() {
        match get_default_device() {
            Ok(device) => {
                if let Ok(name) = device.name() {
                    println!("Default device: {}", name);
                }
            }
            Err(e) => {
                println!("Warning: No default device: {}", e);
            }
        }
    }
}

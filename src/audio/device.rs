//! Audio device enumeration and management

use anyhow::Result;

/// Device information
#[allow(dead_code)]
pub struct DeviceInfo {
    pub name: String,
    pub is_default: bool,
}

/// List available audio input devices
#[allow(dead_code)]
pub fn list_devices() -> Result<Vec<DeviceInfo>> {
    // TODO: Implement device enumeration
    Ok(vec![])
}

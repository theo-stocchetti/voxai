//! Audio device enumeration and management

use anyhow::Result;

/// Device information
pub struct DeviceInfo {
    pub name: String,
    pub is_default: bool,
}

/// List available audio input devices
pub fn list_devices() -> Result<Vec<DeviceInfo>> {
    // TODO: Implement device enumeration
    Ok(vec![])
}

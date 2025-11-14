# [ISSUE-007-3] Audio Input Device Selection

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 7 - Gestion de la Configuration

---

## Description

Allow users to select which audio input device to use for recording. Enumerate available microphones and let users choose their preferred device in the settings UI.

---

## Context

Users may have multiple audio input devices (built-in microphone, USB microphone, audio interface, virtual audio cables). VoxAI should allow users to choose which device to use rather than always using the system default.

---

## Acceptance Criteria

- [ ] Enumerate all available audio input devices
- [ ] Display device list in settings UI
- [ ] Users can select preferred input device
- [ ] Selected device persists in configuration
- [ ] VoxAI uses selected device for recording
- [ ] Handle device removal gracefully (fallback to default)

---

## Technical Details

### Affected Components
- `src/audio/device.rs` - Device enumeration and management
- `src/ui/settings.rs` - Device selector UI
- `src/config/schema.rs` - Add device configuration

### Dependencies
- [ ] CPAL audio library
- [ ] Settings UI (007-2)
- [ ] Persistent configuration (007-1)

### Implementation Notes

**Device Enumeration with CPAL**:
```rust
use cpal::traits::{HostTrait, DeviceTrait};

pub fn enumerate_input_devices() -> Result<Vec<DeviceInfo>> {
    let host = cpal::default_host();
    let devices: Vec<DeviceInfo> = host.input_devices()?
        .filter_map(|device| {
            let name = device.name().ok()?;
            Some(DeviceInfo {
                id: device.name().ok()?, // Use name as ID
                name,
                is_default: false, // TODO: check if default
            })
        })
        .collect();

    Ok(devices)
}

pub fn get_default_input_device() -> Result<Device> {
    let host = cpal::default_host();
    host.default_input_device()
        .ok_or_else(|| anyhow!("No default input device"))
}
```

**Configuration Schema**:
```rust
#[derive(Serialize, Deserialize)]
struct AudioConfig {
    input_device: Option<String>, // Device name, None = default
    sample_rate: u32,              // 16000 for Whisper
    noise_reduction: bool,
    vad_enabled: bool,
}
```

**Settings UI**:
- ComboBox/Dropdown with device names
- Show "[Default]" for system default
- Refresh button to re-enumerate devices

---

## Tasks Breakdown

- [ ] Implement audio device enumeration in `audio/device.rs`
- [ ] Detect default input device
- [ ] Add device configuration field to schema
- [ ] Create device selector UI component in settings
- [ ] Implement device selection persistence
- [ ] Update audio capture to use configured device
- [ ] Add "Refresh Devices" button in UI
- [ ] Handle device not available (fallback to default)
- [ ] Add device name to logs for debugging
- [ ] Test with multiple input devices

---

## Test Plan

### Unit Tests
- [ ] Test device enumeration
- [ ] Test default device detection
- [ ] Test device selection persistence

### Integration Tests
- [ ] Test recording with selected device
- [ ] Test fallback when device unavailable
- [ ] Test device change during runtime (if possible)

### Manual Testing
- [ ] Enumerate devices on system with multiple inputs
- [ ] Select non-default device and verify recording uses it
- [ ] Disconnect selected device and verify fallback
- [ ] Test on systems with:
  - Built-in microphone only
  - USB microphone
  - Audio interface with multiple inputs
  - Virtual audio cable (e.g., VB-Audio)

---

## Documentation Updates

- [ ] Update README.md with device selection feature
- [ ] Update CLAUDE.md with device management details
- [ ] Document how to troubleshoot device issues
- [ ] Add FAQ for common device problems

---

## Related Issues

- Related to: #002-1 (Audio Capture - uses selected device)
- Depends on: #007-1 (Persistent Configuration)
- Depends on: #007-2 (Settings UI)

---

## Notes

**Device Identification**:
- CPAL uses device names as identifiers
- Device names may not be unique (e.g., multiple "USB Microphone")
- Consider using device index + name for uniqueness

**Common Device Types**:
- Built-in microphone (laptop, desktop)
- USB microphones (Blue Yeti, Rode, etc.)
- Audio interfaces (Focusrite, Behringer, etc.)
- Headset microphones
- Virtual audio cables (for capturing system audio)

**Edge Cases**:
- Device unplugged while recording → graceful fallback
- Device renamed by OS → may lose selection
- Permission denied for device access → clear error message

**Platform Differences**:
- **Windows**: Enumerate via WASAPI
- **macOS**: Enumerate via CoreAudio
- **Linux**: Enumerate via ALSA/PulseAudio/JACK

**Future Enhancements**:
- Device status indicator (connected/disconnected)
- Test audio button (play test tone/show input level)
- Per-device configuration (sample rate, channels)
- Device hotplug detection and notification
- Audio mixer (combine multiple inputs)

---

## Definition of Done

- [ ] Device enumeration implemented
- [ ] Device selector in settings UI
- [ ] Device selection persists
- [ ] Recording uses selected device
- [ ] Fallback to default when device unavailable
- [ ] Tests passing with multiple devices
- [ ] Documentation updated
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

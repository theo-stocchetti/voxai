# Auto-Update System Design

**Status**: Planned for Week 7
**Priority**: Medium (Post-MVP)

---

## Overview

VoxAI will include an automatic update system that:
- Checks for new versions
- Downloads updates securely
- Verifies signatures
- Installs updates with user consent

---

## Architecture

```
┌─────────────────────┐
│   Update Checker    │
│  (Background task)  │
└──────────┬──────────┘
           │
           ├─> Check GitHub Releases API
           ├─> Compare versions
           └─> Notify user if update available
                   │
                   v
        ┌──────────────────┐
        │  Update Download │
        └──────────┬───────┘
                   │
                   ├─> Download .zip/.dmg/.deb
                   ├─> Verify SHA256
                   └─> Verify signature
                           │
                           v
                ┌──────────────────┐
                │ Update Installer │
                └──────────┬───────┘
                           │
                           ├─> Extract files
                           ├─> Replace binary
                           └─> Restart application
```

---

## Implementation Plan

### 1. Version Checking

```rust
// src/update/mod.rs
pub struct UpdateChecker {
    current_version: String,
    update_url: String,
}

impl UpdateChecker {
    pub async fn check_for_updates() -> Result<Option<UpdateInfo>> {
        // Check GitHub Releases API
        // Compare versions
        // Return update info if available
    }
}
```

### 2. Update Download

```rust
pub struct UpdateDownloader {
    update_info: UpdateInfo,
}

impl UpdateDownloader {
    pub async fn download(&self) -> Result<PathBuf> {
        // Download update package
        // Show progress
        // Verify checksum
    }
}
```

### 3. Update Installation

```rust
pub struct UpdateInstaller {
    package_path: PathBuf,
}

impl UpdateInstaller {
    pub fn install(&self) -> Result<()> {
        // Platform-specific installation
        // Windows: Run installer
        // macOS: Mount DMG and copy
        // Linux: Install package
    }
}
```

---

## Security Considerations

1. **HTTPS Only**: All downloads via HTTPS
2. **SHA256 Verification**: Verify checksums
3. **Code Signing**: Verify digital signatures
4. **User Consent**: Always ask before installing
5. **Rollback**: Backup old version

---

## UI Flow

1. **Check on Startup** (background)
2. **Notification**: "Update available: v1.2.0"
3. **User Action**:
   - "Download Now"
   - "Remind Me Later"
   - "Skip This Version"
4. **Download Progress**: Progress bar
5. **Install Prompt**: "Ready to install. Restart now?"
6. **Restart**: Application restarts with new version

---

## Testing

- [ ] Mock update server
- [ ] Test download with network errors
- [ ] Test installation on all platforms
- [ ] Test rollback
- [ ] Test signature verification

---

## Future Enhancements

- Delta updates (only download changes)
- Background installation
- Auto-restart during idle time
- Update channels (stable/beta)

---

## Status: Placeholder

This system will be implemented in Week 7.
For now, users must manually download updates.

# [ISSUE-011-4] Auto-Update System Implementation

**Created**: 2025-11-14
**Priority**: Moyenne
**Type**: Feature
**Status**: Todo
**Estimated Effort**: L
**EPIC**: EPIC 11 - Déploiement et Distribution

---

## Description

Implement automatic update system to notify users of new VoxAI versions and optionally download/install updates automatically. Use GitHub Releases as update source.

---

## Context

Auto-updates improve user experience by ensuring users have the latest features and bug fixes. VoxAI should check for updates periodically and notify users.

---

## Acceptance Criteria

- [ ] Check for updates on startup (configurable)
- [ ] Notify user when update available
- [ ] Show changelog/release notes
- [ ] Download update in background
- [ ] Install update (with user confirmation)
- [ ] Rollback mechanism if update fails
- [ ] Configurable update channel (stable, beta)

---

## Technical Details

### Tools
- **self_update** crate - Rust self-update library
- GitHub Releases API for version checking

### Implementation Notes

**Using self_update crate**:
```rust
use self_update::cargo_crate_version;

pub fn check_for_updates() -> Result<Option<String>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("voxai")
        .repo_name("voxai")
        .bin_name("voxai")
        .current_version(cargo_crate_version!())
        .build()?
        .get_latest_release()?;

    if status.version > cargo_crate_version!() {
        Ok(Some(status.version))
    } else {
        Ok(None)
    }
}

pub fn install_update() -> Result<()> {
    self_update::backends::github::Update::configure()
        .repo_owner("voxai")
        .repo_name("voxai")
        .bin_name("voxai")
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    Ok(())
}
```

**Update Flow**:
1. Check GitHub Releases API for latest version
2. Compare with current version
3. Notify user if new version available
4. Download update binary
5. Verify checksum
6. Replace current binary (requires restart)
7. Restart application

---

## Tasks Breakdown

- [ ] Add self_update dependency
- [ ] Implement update check on startup
- [ ] Implement version comparison
- [ ] Add update notification UI
- [ ] Implement download and install
- [ ] Add update settings (auto-check, channel)
- [ ] Test update process
- [ ] Handle errors gracefully

---

## Test Plan

- [ ] Publish test release on GitHub
- [ ] Verify update check detects new version
- [ ] Verify download works
- [ ] Verify installation and restart
- [ ] Test rollback if update fails

---

## Documentation Updates

- [ ] Document update system in README
- [ ] Document release process

---

## Notes

**Security Considerations**:
- Verify checksums/signatures of downloads
- Use HTTPS for all update checks
- Validate version numbers

**Future Enhancements**:
- Delta updates (only download changes)
- Background silent updates
- Beta/nightly update channels

---

## Definition of Done

- [ ] Auto-update system implemented
- [ ] Updates work from GitHub Releases
- [ ] User can opt-in/opt-out of auto-updates
- [ ] Tested with actual release
- [ ] Documentation updated
- [ ] Issue moved to done folder

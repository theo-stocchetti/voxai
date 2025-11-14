# [ISSUE-011-1] Windows Packaging and Installer

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 11 - Déploiement et Distribution

---

## Description

Create Windows installer package (.msi or .exe) for easy VoxAI installation. Include all dependencies, proper installation paths, start menu shortcuts, and uninstaller.

---

## Acceptance Criteria

- [ ] Installer package created (.msi or .exe)
- [ ] Installs VoxAI to Program Files
- [ ] Creates start menu shortcut
- [ ] Adds to Windows Apps & Features
- [ ] Includes uninstaller
- [ ] Optionally adds to startup
- [ ] Bundles required DLLs (Visual C++ runtime, etc.)

---

## Technical Details

### Tools
- **WiX Toolset** (for .msi) - Recommended
- **Inno Setup** (for .exe) - Alternative
- **cargo-wix** - Rust-friendly WiX wrapper

### Implementation Notes

**Using cargo-wix**:
```toml
# Cargo.toml
[package.metadata.wix]
upgrade-guid = "GUID-HERE"
path-guid = "GUID-HERE"
license = false
eula = false
```

```bash
# Build installer
cargo wix
```

**Installer Features**:
- Install to `C:\Program Files\VoxAI\`
- Add to Start Menu
- Desktop shortcut (optional)
- Add to startup (optional checkbox)
- Include Whisper model (tiny.bin) or download on first run

---

## Tasks Breakdown

- [ ] Set up cargo-wix or WiX Toolset
- [ ] Create WiX configuration
- [ ] Configure install paths
- [ ] Add start menu shortcut
- [ ] Bundle required dependencies
- [ ] Test installer on clean Windows install
- [ ] Test uninstaller
- [ ] Sign installer (future: code signing certificate)

---

## Test Plan

- [ ] Install on Windows 10
- [ ] Install on Windows 11
- [ ] Verify start menu shortcut works
- [ ] Verify uninstaller removes all files
- [ ] Test upgrade scenario

---

## Documentation Updates

- [ ] Add Windows installation instructions to README.md
- [ ] Document build process for Windows package

---

## Definition of Done

- [ ] Windows installer created
- [ ] Installer tested on clean Windows system
- [ ] Uninstaller working correctly
- [ ] Documentation updated
- [ ] Issue moved to done folder

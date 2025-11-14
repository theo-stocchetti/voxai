# [ISSUE-011-2] macOS Packaging and Distribution

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 11 - Déploiement et Distribution

---

## Description

Create macOS application bundle (.app) and optionally DMG installer for VoxAI distribution. Support both Intel and Apple Silicon (universal binary).

---

## Acceptance Criteria

- [ ] .app bundle created
- [ ] Universal binary (Intel + Apple Silicon)
- [ ] DMG installer for distribution
- [ ] Code signed (if certificate available)
- [ ] Notarized by Apple (if certificate available)
- [ ] Includes required assets and models
- [ ] Proper Info.plist configuration

---

## Technical Details

### Tools
- **cargo-bundle** - Create .app bundles
- **create-dmg** - Create DMG installers
- **codesign** - Sign application
- **xcrun notarytool** - Notarize with Apple

### Implementation Notes

**Using cargo-bundle**:
```toml
# Cargo.toml
[package.metadata.bundle]
name = "VoxAI"
identifier = "com.voxai.app"
icon = ["assets/icons/macos/icon.icns"]
version = "1.0.0"
resources = ["models/tiny.bin", "assets/"]
copyright = "Copyright © 2025 VoxAI"
category = "Utility"
```

```bash
# Build universal binary
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create -output target/universal/voxai \
  target/x86_64-apple-darwin/release/voxai \
  target/aarch64-apple-darwin/release/voxai

# Create .app bundle
cargo bundle --release

# Create DMG
create-dmg VoxAI.app
```

**Code Signing** (requires Apple Developer account):
```bash
codesign --deep --force --verify --verbose --sign "Developer ID Application: Name" VoxAI.app
xcrun notarytool submit VoxAI.dmg --wait
```

---

## Tasks Breakdown

- [ ] Set up cargo-bundle
- [ ] Configure bundle metadata
- [ ] Build universal binary (Intel + ARM)
- [ ] Create .app bundle
- [ ] Create DMG installer
- [ ] (Optional) Code sign and notarize
- [ ] Test on Intel Mac
- [ ] Test on Apple Silicon Mac

---

## Test Plan

- [ ] Install and run on macOS 13 Ventura (Intel)
- [ ] Install and run on macOS 14 Sonoma (Apple Silicon)
- [ ] Verify menu bar icon appears
- [ ] Verify all features work from .app

---

## Documentation Updates

- [ ] Add macOS installation instructions
- [ ] Document build process for macOS package

---

## Definition of Done

- [ ] .app bundle created
- [ ] Universal binary working
- [ ] DMG installer created
- [ ] Tested on Intel and Apple Silicon
- [ ] Documentation updated
- [ ] Issue moved to done folder

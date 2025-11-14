# [ISSUE-011-3] Linux Packaging and Distribution

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: L
**EPIC**: EPIC 11 - Déploiement et Distribution

---

## Description

Create Linux packages for major distributions: .deb (Debian/Ubuntu), .rpm (Fedora/RHEL), AppImage, and Flatpak. Ensure broad Linux compatibility.

---

## Acceptance Criteria

- [ ] .deb package for Debian/Ubuntu
- [ ] .rpm package for Fedora/RHEL
- [ ] AppImage for universal Linux
- [ ] (Optional) Flatpak package
- [ ] Desktop file installed
- [ ] Icons installed to system
- [ ] Packages tested on target distributions

---

## Technical Details

### Tools
- **cargo-deb** - Create .deb packages
- **cargo-generate-rpm** - Create .rpm packages
- **cargo-appimage** - Create AppImage
- **flatpak-builder** - Create Flatpak

### Implementation Notes

**Debian Package** (cargo-deb):
```toml
# Cargo.toml
[package.metadata.deb]
maintainer = "VoxAI Team <email@example.com>"
copyright = "2025, VoxAI Team"
license-file = ["LICENSE", "4"]
extended-description = "Real-time audio transcription with Whisper AI"
depends = "$auto, libasound2, libssl3"
section = "sound"
priority = "optional"
assets = [
    ["target/release/voxai", "usr/bin/", "755"],
    ["assets/voxai.desktop", "usr/share/applications/", "644"],
    ["assets/icons/**/*", "usr/share/icons/hicolor/", "644"],
]
```

```bash
# Build .deb
cargo deb
```

**RPM Package** (cargo-generate-rpm):
```bash
cargo build --release
cargo generate-rpm
```

**AppImage**:
```bash
# Create AppImage (portable, no installation)
cargo-appimage
```

**Desktop File** (voxai.desktop):
```ini
[Desktop Entry]
Name=VoxAI
Comment=Real-time voice transcription
Exec=voxai
Icon=voxai
Terminal=false
Type=Application
Categories=Audio;Utility;
```

---

## Tasks Breakdown

- [ ] Set up cargo-deb
- [ ] Create .deb package
- [ ] Set up cargo-generate-rpm
- [ ] Create .rpm package
- [ ] Create AppImage
- [ ] Create desktop file
- [ ] Install icons to hicolor theme
- [ ] Test .deb on Ubuntu 22.04, 24.04
- [ ] Test .rpm on Fedora
- [ ] Test AppImage on various distros

---

## Test Plan

- [ ] Install .deb on Ubuntu 22.04
- [ ] Install .rpm on Fedora 39
- [ ] Run AppImage on Arch Linux
- [ ] Verify desktop file creates launcher
- [ ] Verify icons appear correctly
- [ ] Verify system tray works

---

## Documentation Updates

- [ ] Add Linux installation instructions for each package type
- [ ] Document build process

---

## Definition of Done

- [ ] .deb, .rpm, and AppImage packages created
- [ ] Packages tested on target distributions
- [ ] Desktop integration working
- [ ] Documentation updated
- [ ] Issue moved to done folder

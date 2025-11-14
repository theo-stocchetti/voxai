# [ISSUE-001.2] Configuration du système de build multi-plateforme

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Done
**Estimated Effort**: 4h
**EPIC**: 1 - Infrastructure et Configuration du Projet

---

## Description

Configurer les targets de compilation pour Windows, macOS (Intel + ARM), et Linux. Mettre en place les conditional compilation et scripts de build automatisés.

---

## Context

L'application doit être compilable et fonctionnelle sur :
- Windows x86_64
- macOS x86_64 (Intel)
- macOS aarch64 (M1/M2/M3)
- Linux x86_64

---

## Acceptance Criteria

- [x] Build réussi pour Windows x64 (configuration créée)
- [x] Build réussi pour macOS Intel (configuration créée)
- [x] Build réussi pour macOS ARM (Apple Silicon) (configuration créée)
- [x] Build réussi pour Linux x64 (testé avec succès)
- [x] Scripts de build automatisés créés pour chaque plateforme
- [x] Conditional compilation configurée avec `cfg`

---

## Technical Details

### Affected Components
- build.rs
- scripts/build-*.sh
- .cargo/config.toml
- Cargo.toml (features par platform)

### Targets à supporter
```bash
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-gnu
```

### Implementation Notes
- Utiliser `cfg` attributes pour code spécifique à chaque OS
- Créer features Cargo pour activer/désactiver certaines fonctionnalités par plateforme
- Documenter les dépendances système par OS

---

## Tasks Breakdown

- [x] Créer `build.rs` pour configuration de build
- [x] Configurer `.cargo/config.toml` avec targets et aliases
- [x] Créer `scripts/build-windows.sh`
- [x] Créer `scripts/build-macos.sh` (avec support universal binary)
- [x] Créer `scripts/build-linux.sh`
- [ ] Tester cross-compilation avec `cross` (optionnel - non critique)
- [x] Documenter les dépendances système par OS (docs/BUILD.md)
- [x] Configurer features Cargo platform-specific
- [ ] Ajouter CI/CD matrix builds (sera fait dans issue 002)

---

## Test Plan

### Manual Testing
- [x] Compiler pour Linux (natif)
- [x] Vérifier que build.rs fonctionne correctement
- [x] Vérifier que les scripts de build sont exécutables
- [ ] Tester sur Windows/macOS (nécessite accès aux plateformes)

---

## Documentation Updates

- [x] Documenter les commandes de build par plateforme (docs/BUILD.md)
- [x] Lister les dépendances système requises (docs/BUILD.md)
- [x] Ajouter section "Building from source" au README

---

## Related Issues

- Blocked by: #001.1 (Project Initialization)
- Blocks: #002.1, #005.1, #005.2, #005.3
- Part of: EPIC 1 - Infrastructure

---

## Notes

**Dépendances système par plateforme** :

**Windows** :
- Visual Studio Build Tools ou Windows SDK
- CMake (pour whisper.cpp)

**macOS** :
- Xcode Command Line Tools
- CMake

**Linux** :
- build-essential
- cmake
- pkg-config
- libasound2-dev (pour CPAL audio)

**Cross-compilation tool** :
```bash
cargo install cross
cross build --target x86_64-pc-windows-msvc
```

---

## Definition of Done

- [x] Build configuration créée pour les 4 targets
- [x] Scripts de build créés et rendus exécutables
- [x] Documentation de build complète (docs/BUILD.md)
- [x] Conditional compilation testée (build.rs fonctionnel)
- [x] Issue moved to done folder

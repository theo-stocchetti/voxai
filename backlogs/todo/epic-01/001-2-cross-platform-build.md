# [ISSUE-001.2] Configuration du système de build multi-plateforme

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
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

- [ ] Build réussi pour Windows x64
- [ ] Build réussi pour macOS Intel
- [ ] Build réussi pour macOS ARM (Apple Silicon)
- [ ] Build réussi pour Linux x64
- [ ] Scripts de build automatisés créés pour chaque plateforme
- [ ] Conditional compilation configurée avec `cfg`

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

- [ ] Créer `build.rs` pour configuration de build
- [ ] Configurer `.cargo/config.toml` avec targets
- [ ] Créer `scripts/build-windows.sh`
- [ ] Créer `scripts/build-macos.sh`
- [ ] Créer `scripts/build-linux.sh`
- [ ] Tester cross-compilation avec `cross` (optionnel)
- [ ] Documenter les dépendances système par OS
- [ ] Configurer features Cargo platform-specific
- [ ] Ajouter CI/CD matrix builds (pour plus tard)

---

## Test Plan

### Manual Testing
- [ ] Compiler pour chaque target sur la machine de dev
- [ ] Tester l'exécution sur chaque OS (si possible)
- [ ] Vérifier que les scripts de build fonctionnent
- [ ] Tester la cross-compilation

---

## Documentation Updates

- [ ] Documenter les commandes de build par plateforme
- [ ] Lister les dépendances système requises
- [ ] Ajouter section "Building from source" au README

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

- [ ] Build réussi sur les 4 targets
- [ ] Scripts de build créés et testés
- [ ] Documentation de build complète
- [ ] Conditional compilation testée
- [ ] Issue moved to done folder

# [ISSUE-001.1] Initialisation du projet Rust

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: 2h
**EPIC**: 1 - Infrastructure et Configuration du Projet

---

## Description

Créer la structure du projet Cargo et configurer le workspace Rust multi-crates avec les dépendances principales.

---

## Context

VoxAI est une application de transcription audio en temps réel multi-plateforme (Windows, macOS, Linux) utilisant Rust et Whisper.cpp. Cette issue est la première étape fondamentale du projet.

---

## Acceptance Criteria

- [ ] Projet Cargo initialisé avec structure modulaire
- [ ] Compilation réussie avec `cargo build`
- [ ] README.md de base créé
- [ ] .gitignore configuré pour Rust
- [ ] Dépendances principales ajoutées au Cargo.toml

---

## Technical Details

### Affected Components
- Cargo.toml
- src/ structure
- .gitignore

### Dependencies principales

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
anyhow = "1.0"
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
log = "0.4"
env_logger = "0.11"
```

### Implementation Notes
- Utiliser Rust edition 2021
- Structure modulaire avec separation des concerns
- Préparer pour architecture multi-crates si nécessaire

---

## Tasks Breakdown

- [ ] Exécuter `cargo new voicescript --bin`
- [ ] Créer structure de dossiers `src/{audio,transcription,ui,config,hotkeys,output}`
- [ ] Ajouter dépendances de base au Cargo.toml
- [ ] Créer .gitignore complet pour Rust
- [ ] Créer README.md avec description du projet
- [ ] Test de compilation: `cargo build`
- [ ] Test d'exécution: `cargo run`

---

## Test Plan

### Manual Testing
- [ ] Vérifier que `cargo build` réussit
- [ ] Vérifier que `cargo run` exécute un "Hello World"
- [ ] Vérifier la structure des dossiers

---

## Documentation Updates

- [ ] Créer README.md initial
- [ ] Documenter la structure du projet
- [ ] Ajouter instructions de build de base

---

## Related Issues

- Blocks: #001.2, #001.3, et tous les autres issues
- Part of: EPIC 1 - Infrastructure

---

## Notes

Commandes à exécuter :
```bash
cargo new voicescript --bin
cd voicescript
mkdir -p src/{audio,transcription,ui,config,hotkeys,output}
cargo build
```

---

## Definition of Done

- [ ] Code compilé sans erreurs
- [ ] Structure de dossiers créée
- [ ] README.md présent
- [ ] .gitignore configuré
- [ ] Commit initial créé
- [ ] Issue moved to done folder

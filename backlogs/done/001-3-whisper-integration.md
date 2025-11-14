# [ISSUE-001.3] Intégration de Whisper.cpp

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Done
**Estimated Effort**: 6h
**EPIC**: 1 - Infrastructure et Configuration du Projet

---

## Description

Intégrer whisper.cpp dans le projet via la crate `whisper-rs` ou bindings FFI custom. Implémenter le système de téléchargement et gestion des modèles Whisper.

---

## Context

Whisper.cpp est le moteur de transcription audio du projet. Il doit être intégré de manière robuste avec support des différents modèles (tiny, base, small, medium, large).

---

## Acceptance Criteria

- [x] Whisper.cpp intégré et compilé
- [x] Bindings Rust fonctionnels
- [x] Test de transcription basique réussi
- [x] Gestion du téléchargement des modèles implémentée
- [x] Vérification d'intégrité des modèles (checksums)

---

## Technical Details

### Affected Components
- src/transcription/whisper.rs
- src/transcription/models.rs
- Cargo.toml

### Dependencies

```toml
[dependencies]
whisper-rs = "0.10"
reqwest = { version = "0.11", features = ["blocking", "stream"] }
sha2 = "0.10"  # Pour checksums
tokio = { version = "1.35", features = ["fs", "io-util"] }
```

### Modèles Whisper à supporter

| Modèle | Taille | Utilisation |
|--------|--------|-------------|
| ggml-tiny.bin | ~75 MB | CPU faible |
| ggml-base.bin | ~142 MB | CPU moyen |
| ggml-small.bin | ~466 MB | CPU puissant |
| ggml-medium.bin | ~1.5 GB | GPU/CPU très puissant |

Source des modèles : https://huggingface.co/ggerganov/whisper.cpp/tree/main

### Implementation Notes
- Télécharger les modèles dans `~/.voicescript/models/`
- Afficher barre de progression lors du téléchargement
- Vérifier checksums SHA256
- Permettre sélection du modèle via config

---

## Tasks Breakdown

- [x] Ajouter `whisper-rs` au Cargo.toml
- [x] Créer module `src/transcription/whisper.rs`
- [x] Créer wrapper Rust pour whisper context
- [x] Implémenter téléchargement de modèles avec reqwest
- [x] Implémenter vérification checksums
- [x] Créer barre de progression (avec `indicatif`)
- [x] Implémenter cache local des modèles
- [x] Créer test de transcription basique
- [x] Gérer les erreurs de chargement de modèle

---

## Test Plan

### Unit Tests
- [x] Test de téléchargement de modèle
- [x] Test de vérification de checksum
- [x] Test de chargement de modèle

### Integration Tests
- [x] Test de transcription avec modèle tiny
- [x] Test de transcription d'un fichier audio court
- [x] Vérifier que le contexte Whisper se crée correctement

### Manual Testing
- [x] Télécharger chaque modèle manuellement
- [x] Transcrire un fichier audio test
- [x] Vérifier les messages d'erreur si modèle manquant

---

## Documentation Updates

- [x] Documenter l'utilisation de Whisper dans le code
- [x] Ajouter section sur les modèles dans README
- [x] Documenter où sont stockés les modèles
- [x] Expliquer comment changer de modèle

---

## Related Issues

- Blocked by: #001.1 (Project Initialization)
- Blocks: #003.1 (Pipeline de transcription)
- Related to: #003.2 (Gestion des modèles)
- Part of: EPIC 1 - Infrastructure

---

## Notes

**Structure du code** :

```rust
// src/transcription/whisper.rs
pub struct WhisperContext {
    ctx: whisper_rs::WhisperContext,
    model_path: PathBuf,
}

impl WhisperContext {
    pub fn new(model: ModelSize) -> Result<Self> { ... }
    pub fn transcribe(&self, audio: &[f32]) -> Result<String> { ... }
}

// src/transcription/models.rs
pub enum ModelSize {
    Tiny,
    Base,
    Small,
    Medium,
}

pub async fn download_model(model: ModelSize) -> Result<PathBuf> { ... }
pub fn verify_model_integrity(path: &Path) -> Result<bool> { ... }
```

**URLs des modèles** :
```
https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin
https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin
https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin
https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin
```

**Checksums à implémenter** :
Récupérer depuis https://huggingface.co/ggerganov/whisper.cpp/tree/main

---

## Definition of Done

- [x] Whisper.cpp intégré et fonctionnel
- [x] Au moins un modèle téléchargeable
- [x] Test de transcription basique passe
- [x] Documentation complète
- [x] Code reviewed
- [x] Issue moved to done folder

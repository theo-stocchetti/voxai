# [ISSUE-001.3] Intégration de Whisper.cpp

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
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

- [ ] Whisper.cpp intégré et compilé
- [ ] Bindings Rust fonctionnels
- [ ] Test de transcription basique réussi
- [ ] Gestion du téléchargement des modèles implémentée
- [ ] Vérification d'intégrité des modèles (checksums)

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

- [ ] Ajouter `whisper-rs` au Cargo.toml
- [ ] Créer module `src/transcription/whisper.rs`
- [ ] Créer wrapper Rust pour whisper context
- [ ] Implémenter téléchargement de modèles avec reqwest
- [ ] Implémenter vérification checksums
- [ ] Créer barre de progression (avec `indicatif`)
- [ ] Implémenter cache local des modèles
- [ ] Créer test de transcription basique
- [ ] Gérer les erreurs de chargement de modèle

---

## Test Plan

### Unit Tests
- [ ] Test de téléchargement de modèle
- [ ] Test de vérification de checksum
- [ ] Test de chargement de modèle

### Integration Tests
- [ ] Test de transcription avec modèle tiny
- [ ] Test de transcription d'un fichier audio court
- [ ] Vérifier que le contexte Whisper se crée correctement

### Manual Testing
- [ ] Télécharger chaque modèle manuellement
- [ ] Transcrire un fichier audio test
- [ ] Vérifier les messages d'erreur si modèle manquant

---

## Documentation Updates

- [ ] Documenter l'utilisation de Whisper dans le code
- [ ] Ajouter section sur les modèles dans README
- [ ] Documenter où sont stockés les modèles
- [ ] Expliquer comment changer de modèle

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

- [ ] Whisper.cpp intégré et fonctionnel
- [ ] Au moins un modèle téléchargeable
- [ ] Test de transcription basique passe
- [ ] Documentation complète
- [ ] Code reviewed
- [ ] Issue moved to done folder

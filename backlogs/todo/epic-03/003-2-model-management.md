# [ISSUE-003.2] Gestion des modèles Whisper

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: 6h
**EPIC**: 3 - Transcription avec Whisper

---

## Description

Système complet de gestion des modèles Whisper: téléchargement automatique, sélection intelligente selon les performances du système, cache local, vérification d'intégrité.

---

## Context

L'application doit télécharger les modèles Whisper au premier lancement et permettre à l'utilisateur de changer de modèle selon ses besoins (rapidité vs précision).

---

## Acceptance Criteria

- [ ] Téléchargement automatique au premier lancement
- [ ] Sélection automatique tiny/base/small/medium selon CPU/GPU
- [ ] Barre de progression pour le téléchargement
- [ ] Vérification d'intégrité (checksums SHA256)
- [ ] Cache local des modèles
- [ ] Interface de sélection manuelle du modèle

---

## Technical Details

### Affected Components
- src/transcription/models.rs
- src/transcription/downloader.rs
- src/config.rs (model selection)
- src/ui/settings.rs (selection UI)

### Dependencies

```toml
[dependencies]
reqwest = { version = "0.11", features = ["stream"] }
tokio = { version = "1.35", features = ["fs"] }
sha2 = "0.10"
indicatif = "0.17"  # Progress bar
sysinfo = "0.30"    # System info (CPU/GPU detection)
```

### Modèles Whisper

| Modèle | Taille | RAM | Speed | Use Case |
|--------|--------|-----|-------|----------|
| tiny   | 75 MB  | ~1 GB | ~10x realtime | CPU faible |
| base   | 142 MB | ~1 GB | ~7x realtime  | CPU moyen |
| small  | 466 MB | ~2 GB | ~4x realtime  | CPU puissant |
| medium | 1.5 GB | ~5 GB | ~2x realtime  | GPU/CPU très puissant |

### Implementation Notes
- Stocker modèles dans `~/.voicescript/models/`
- Format: `ggml-{model}.bin`
- Télécharger depuis HuggingFace
- Vérifier SHA256 avant utilisation

---

## Tasks Breakdown

- [ ] Créer module `src/transcription/models.rs`
- [ ] Créer module `src/transcription/downloader.rs`
- [ ] Définir enum ModelSize
- [ ] Implémenter détection des capacités système
- [ ] Implémenter algorithme de sélection automatique
- [ ] Implémenter téléchargement avec progress bar
- [ ] Implémenter vérification SHA256
- [ ] Créer système de cache local
- [ ] Gérer les erreurs réseau (retry logic)
- [ ] Implémenter sélection manuelle via config
- [ ] Créer tests unitaires

---

## Test Plan

### Unit Tests
- [ ] Test de sélection automatique
- [ ] Test de vérification checksum (bon et mauvais)
- [ ] Test de gestion du cache

### Integration Tests
- [ ] Test de téléchargement complet (tiny model)
- [ ] Test de changement de modèle à chaud
- [ ] Vérifier que modèle corrompu est re-téléchargé

### Manual Testing
- [ ] Supprimer cache et relancer app
- [ ] Changer de modèle via settings
- [ ] Interrompre téléchargement et vérifier retry

---

## Documentation Updates

- [ ] Documenter les modèles disponibles
- [ ] Expliquer le choix automatique
- [ ] Documenter comment changer de modèle
- [ ] Ajouter tableau comparatif des modèles

---

## Related Issues

- Blocked by: #001.3 (Whisper integration)
- Related to: #003.1 (Transcription pipeline), #007.2 (Settings UI)
- Part of: EPIC 3 - Transcription

---

## Notes

**URLs des modèles HuggingFace** :

```rust
const MODEL_URLS: &[(&str, &str, &str)] = &[
    ("tiny",   "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",   "SHA256_HERE"),
    ("base",   "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",   "SHA256_HERE"),
    ("small",  "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",  "SHA256_HERE"),
    ("medium", "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin", "SHA256_HERE"),
];
```

**Structure du code** :

```rust
// src/transcription/models.rs
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ModelSize {
    Tiny,
    Base,
    Small,
    Medium,
}

impl ModelSize {
    pub fn file_name(&self) -> &str {
        match self {
            Self::Tiny => "ggml-tiny.bin",
            Self::Base => "ggml-base.bin",
            Self::Small => "ggml-small.bin",
            Self::Medium => "ggml-medium.bin",
        }
    }

    pub fn url(&self) -> &str { ... }
    pub fn expected_sha256(&self) -> &str { ... }
    pub fn size_mb(&self) -> usize { ... }
}

pub fn get_models_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap()
        .join("voicescript")
        .join("models")
}

pub fn auto_select_model() -> ModelSize {
    let sys = sysinfo::System::new_all();

    let cpu_count = sys.cpus().len();
    let total_ram_gb = sys.total_memory() / (1024 * 1024 * 1024);
    let has_gpu = detect_gpu();

    if has_gpu && total_ram_gb >= 8 {
        ModelSize::Medium
    } else if cpu_count >= 8 && total_ram_gb >= 4 {
        ModelSize::Small
    } else if cpu_count >= 4 {
        ModelSize::Base
    } else {
        ModelSize::Tiny
    }
}

// src/transcription/downloader.rs
pub async fn download_model(model: ModelSize) -> Result<PathBuf> {
    let url = model.url();
    let file_name = model.file_name();
    let models_dir = get_models_dir();
    let file_path = models_dir.join(file_name);

    // Check if already downloaded and valid
    if file_path.exists() && verify_checksum(&file_path, model.expected_sha256())? {
        return Ok(file_path);
    }

    // Download with progress bar
    let response = reqwest::get(url).await?;
    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(/* ... */);

    let mut file = tokio::fs::File::create(&file_path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        pb.inc(chunk.len() as u64);
    }

    pb.finish();

    // Verify checksum
    if !verify_checksum(&file_path, model.expected_sha256())? {
        tokio::fs::remove_file(&file_path).await?;
        return Err(anyhow!("Checksum verification failed"));
    }

    Ok(file_path)
}

fn verify_checksum(path: &Path, expected: &str) -> Result<bool> {
    use sha2::{Sha256, Digest};

    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;

    let hash = format!("{:x}", hasher.finalize());
    Ok(hash == expected)
}
```

**Logique de détection GPU** :
```rust
fn detect_gpu() -> bool {
    #[cfg(feature = "cuda")]
    return cuda_is_available();

    #[cfg(target_os = "macos")]
    return is_apple_silicon();

    false
}
```

---

## Definition of Done

- [ ] Téléchargement automatique fonctionnel
- [ ] Sélection auto basée sur système
- [ ] Checksums vérifiés
- [ ] Progress bar affichée
- [ ] Tests passent
- [ ] Documentation complète
- [ ] Issue moved to done folder

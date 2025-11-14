# [ISSUE-002.1] Implémentation de la capture audio avec CPAL

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: 8h
**EPIC**: 2 - Capture Audio

---

## Description

Utiliser la crate `cpal` pour implémenter la capture audio cross-platform. Détecter les périphériques audio, créer un buffer circulaire pour le streaming en temps réel.

---

## Context

La capture audio est le point d'entrée de l'application. Elle doit fonctionner de manière fiable sur Windows, macOS et Linux, avec un buffer circulaire pour le traitement en temps réel.

---

## Acceptance Criteria

- [ ] Capture audio fonctionnelle sur Windows, macOS, Linux
- [ ] Liste des microphones disponibles récupérable
- [ ] Sélection du microphone par défaut du système
- [ ] Buffer audio en temps réel fonctionnel (ring buffer)
- [ ] Format audio: 16kHz mono 16-bit (requis par Whisper)
- [ ] Resampling si nécessaire

---

## Technical Details

### Affected Components
- src/audio/capture.rs
- src/audio/device.rs
- src/audio/buffer.rs

### Dependencies

```toml
[dependencies]
cpal = "0.15"
ringbuf = "0.3"
rubato = "0.14"  # Pour resampling
```

### Implementation Notes
- Whisper nécessite audio 16kHz mono 16-bit
- Utiliser ring buffer pour éviter les pertes de données
- Gérer les erreurs de périphérique (déconnexion, etc.)
- Support hot-plug des périphériques

---

## Tasks Breakdown

- [ ] Créer module `src/audio/capture.rs`
- [ ] Implémenter énumération des périphériques audio
- [ ] Créer fonction de sélection du périphérique par défaut
- [ ] Implémenter création du stream audio avec CPAL
- [ ] Créer ring buffer thread-safe avec `ringbuf`
- [ ] Implémenter resampling vers 16kHz si nécessaire
- [ ] Implémenter conversion stereo → mono
- [ ] Implémenter conversion vers format 16-bit
- [ ] Gérer les erreurs de stream (overflow, underflow)
- [ ] Créer tests unitaires

---

## Test Plan

### Unit Tests
- [ ] Test d'énumération des périphériques
- [ ] Test de création du stream
- [ ] Test du ring buffer (write/read)
- [ ] Test de resampling

### Integration Tests
- [ ] Test de capture audio pendant 5 secondes
- [ ] Vérifier le format de sortie (16kHz mono 16-bit)
- [ ] Test avec différents périphériques

### Manual Testing
- [ ] Tester avec microphone USB
- [ ] Tester avec microphone intégré
- [ ] Tester déconnexion/reconnexion du micro
- [ ] Vérifier latence < 100ms

---

## Documentation Updates

- [ ] Documenter l'API de capture audio
- [ ] Ajouter exemples d'utilisation
- [ ] Documenter les formats audio supportés
- [ ] Documenter la gestion des périphériques

---

## Related Issues

- Blocked by: #001.2 (Cross-platform build)
- Blocks: #002.2 (Noise reduction), #003.1 (Transcription pipeline)
- Related to: #007.3 (Audio device selection UI)
- Part of: EPIC 2 - Capture Audio

---

## Notes

**Structure du code** :

```rust
// src/audio/capture.rs
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{HeapRb, HeapProducer, HeapConsumer};

pub struct AudioCapture {
    device: cpal::Device,
    config: cpal::StreamConfig,
    stream: Option<cpal::Stream>,
    producer: HeapProducer<f32>,
    consumer: HeapConsumer<f32>,
}

impl AudioCapture {
    pub fn new(device: Option<cpal::Device>) -> Result<Self> { ... }

    pub fn start(&mut self) -> Result<()> { ... }

    pub fn stop(&mut self) -> Result<()> { ... }

    pub fn read_samples(&mut self, buffer: &mut [f32]) -> usize { ... }

    pub fn list_devices() -> Result<Vec<DeviceInfo>> { ... }
}

pub struct DeviceInfo {
    pub name: String,
    pub is_default: bool,
    pub supported_configs: Vec<SupportedConfig>,
}
```

**Whisper audio format requirements** :
- Sample rate: 16000 Hz
- Channels: Mono (1 channel)
- Sample format: 32-bit float ou 16-bit PCM
- Normalisation: [-1.0, 1.0] pour float

**Ring buffer size** :
- Recommandé: 5 secondes = 16000 * 5 = 80000 samples
- Permet d'absorber les pics de traitement

---

## Definition of Done

- [ ] Capture audio fonctionnelle sur les 3 OS
- [ ] Format 16kHz mono confirmé
- [ ] Tests passent
- [ ] Documentation complète
- [ ] Code reviewed
- [ ] Issue moved to done folder

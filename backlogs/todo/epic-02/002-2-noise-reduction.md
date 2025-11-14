# [ISSUE-002.2] Implémentation de la réduction de bruit

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: 10h
**EPIC**: 2 - Capture Audio

---

## Description

Intégrer une bibliothèque de réduction de bruit (RNNoise) pour améliorer la qualité de la transcription. Optimiser pour fonctionner en temps réel avec latence minimale.

---

## Context

La réduction de bruit améliore significativement la précision de la transcription dans des environnements bruyants (ventilateurs, bruits de fond, etc.).

---

## Acceptance Criteria

- [ ] Réduction de bruit fonctionnelle
- [ ] Impact sur latence < 50ms
- [ ] Option activable/désactivable à chaud
- [ ] Tests A/B montrant amélioration de la qualité
- [ ] Pas de distorsion de la voix

---

## Technical Details

### Affected Components
- src/audio/noise_reduction.rs
- src/audio/capture.rs (intégration)
- src/config (toggle)

### Dependencies

```toml
[dependencies]
nnnoiseless = "0.5"  # Rust implementation of RNNoise
```

### Implementation Notes
- RNNoise fonctionne sur des frames de 480 samples à 48kHz
- Nécessite resampling 16kHz → 48kHz → processing → 16kHz
- Utiliser un flag pour activer/désactiver dynamiquement

---

## Tasks Breakdown

- [ ] Ajouter `nnnoiseless` au Cargo.toml
- [ ] Créer module `src/audio/noise_reduction.rs`
- [ ] Implémenter wrapper autour de RNNoise
- [ ] Implémenter resampling 16kHz ↔ 48kHz
- [ ] Intégrer dans le pipeline de capture audio
- [ ] Ajouter toggle dans configuration
- [ ] Créer benchmarks de performance
- [ ] Optimiser pour réduire latence
- [ ] Créer tests avec fichiers audio

---

## Test Plan

### Unit Tests
- [ ] Test de processing d'un frame audio
- [ ] Test de resampling bidirectionnel
- [ ] Test d'activation/désactivation

### Integration Tests
- [ ] Test avec fichier audio bruité
- [ ] Comparer audio avant/après réduction de bruit
- [ ] Mesurer la latence ajoutée

### Manual Testing
- [ ] Tester avec bruit de ventilateur
- [ ] Tester avec musique en fond
- [ ] Tester avec conversations en fond
- [ ] Vérifier que la voix reste naturelle

---

## Documentation Updates

- [ ] Documenter l'algorithme utilisé
- [ ] Expliquer comment activer/désactiver
- [ ] Documenter l'impact sur les performances
- [ ] Ajouter section dans le README

---

## Related Issues

- Blocked by: #002.1 (Audio capture)
- Related to: #007.2 (Settings UI)
- Part of: EPIC 2 - Capture Audio

---

## Notes

**RNNoise details** :
- Basé sur un réseau de neurones récurrent
- Entraîné sur des milliers d'heures d'audio
- Très efficace pour réduction de bruit stationnaire
- Frame size: 480 samples @ 48kHz (10ms)
- Latency: ~10ms

**Algorithme de processing** :
```rust
// src/audio/noise_reduction.rs
pub struct NoiseReducer {
    denoiser: nnnoiseless::DenoiseState,
    resampler_up: Resampler,    // 16kHz → 48kHz
    resampler_down: Resampler,  // 48kHz → 16kHz
    enabled: bool,
}

impl NoiseReducer {
    pub fn new() -> Self { ... }

    pub fn process(&mut self, audio_16k: &[f32]) -> Vec<f32> {
        if !self.enabled {
            return audio_16k.to_vec();
        }

        // 1. Resample to 48kHz
        let audio_48k = self.resampler_up.process(audio_16k);

        // 2. Process with RNNoise (480 samples at a time)
        let denoised_48k = self.denoise_frames(&audio_48k);

        // 3. Resample back to 16kHz
        self.resampler_down.process(&denoised_48k)
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
```

**Performance target** :
- Latence totale (resampling + denoising) : < 50ms
- Utilisation CPU : < 5% sur CPU moyen

---

## Definition of Done

- [ ] Réduction de bruit fonctionnelle
- [ ] Latence < 50ms mesurée
- [ ] Toggle implémenté
- [ ] Tests passent
- [ ] Documentation complète
- [ ] Issue moved to done folder

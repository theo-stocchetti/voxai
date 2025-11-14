# [ISSUE-002.3] Détection d'activité vocale (VAD)

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: 6h
**EPIC**: 2 - Capture Audio

---

## Description

Implémenter un système de Voice Activity Detection (VAD) pour détecter quand l'utilisateur parle et éviter de transcrire les silences, optimisant ainsi les ressources CPU/GPU et la batterie.

---

## Context

Transcrire continuellement le silence est un gaspillage de ressources. Un VAD permet de déclencher la transcription uniquement quand de la parole est détectée.

---

## Acceptance Criteria

- [ ] VAD détecte correctement la parole vs silence
- [ ] Seuils configurables
- [ ] Réduction de 60%+ des appels à Whisper inutiles
- [ ] Pas de faux négatifs (parole non détectée)
- [ ] Taux de faux positifs < 5%

---

## Technical Details

### Affected Components
- src/audio/vad.rs
- src/transcription/pipeline.rs (intégration)
- src/config (seuils)

### Implementation Options

**Option 1: WebRTC VAD** (léger, rapide)
```toml
[dependencies]
webrtc-vad = "0.4"
```

**Option 2: Énergie du signal** (très simple)
- Calculer RMS (Root Mean Square) du signal
- Threshold basé sur l'énergie

**Option 3: Silero VAD** (ML-based, plus précis)
```toml
[dependencies]
ort = "1.15"  # ONNX runtime
```

### Implementation Notes
- Recommandé: WebRTC VAD pour le MVP
- Utiliser des fenêtres de 30ms
- Ajouter hysteresis pour éviter le flickering
- Configurer seuils via UI

---

## Tasks Breakdown

- [ ] Choisir algorithme VAD (WebRTC recommandé)
- [ ] Créer module `src/audio/vad.rs`
- [ ] Implémenter wrapper autour de WebRTC VAD
- [ ] Ajouter buffering pour fenêtres de 30ms
- [ ] Implémenter logique d'hysteresis
- [ ] Intégrer VAD dans le pipeline de transcription
- [ ] Ajouter configuration des seuils
- [ ] Créer tests avec audio parlé et silence
- [ ] Créer visualisation du VAD (optionnel)

---

## Test Plan

### Unit Tests
- [ ] Test de détection de parole
- [ ] Test de détection de silence
- [ ] Test d'hysteresis

### Integration Tests
- [ ] Test avec fichier audio contenant parole + silence
- [ ] Mesurer taux de faux positifs/négatifs
- [ ] Vérifier réduction des appels à Whisper

### Manual Testing
- [ ] Parler normalement et vérifier détection
- [ ] Silence complet doit être détecté
- [ ] Parole faible doit être détectée
- [ ] Bruits de fond ne doivent pas trigger

---

## Documentation Updates

- [ ] Expliquer le fonctionnement du VAD
- [ ] Documenter les seuils configurables
- [ ] Ajouter métriques de performance

---

## Related Issues

- Blocked by: #002.1 (Audio capture)
- Blocks: #003.1 (Transcription pipeline)
- Part of: EPIC 2 - Capture Audio

---

## Notes

**WebRTC VAD** :
- 3 modes d'agressivité (0, 1, 2, 3)
- Mode 0: Très sensible (peu de faux négatifs)
- Mode 3: Très sélectif (peu de faux positifs)
- Frame sizes supportées: 10ms, 20ms, 30ms
- Sample rates: 8kHz, 16kHz, 32kHz, 48kHz

**Structure du code** :

```rust
// src/audio/vad.rs
pub struct VoiceActivityDetector {
    vad: webrtc_vad::Vad,
    mode: AggressivenessMode,
    hysteresis_frames: usize,
    speech_frames: usize,
}

pub enum AggressivenessMode {
    Quality = 0,      // Moins de faux négatifs
    LowBitrate = 1,   // Équilibré
    Aggressive = 2,   // Plus agressif
    VeryAggressive = 3, // Très sélectif
}

impl VoiceActivityDetector {
    pub fn new(mode: AggressivenessMode) -> Result<Self> { ... }

    pub fn is_speech(&mut self, audio_frame: &[i16]) -> bool {
        let has_speech = self.vad.is_voice_segment(audio_frame);

        // Hysteresis: require N consecutive speech frames
        if has_speech {
            self.speech_frames += 1;
        } else {
            self.speech_frames = 0;
        }

        self.speech_frames >= self.hysteresis_frames
    }

    pub fn set_aggressiveness(&mut self, mode: AggressivenessMode) { ... }
}
```

**Hysteresis logic** :
- Nécessiter 3 frames consécutifs de parole avant de trigger
- Nécessiter 10 frames consécutifs de silence avant de stopper
- Évite les activations/désactivations intempestives

**Performance metrics** :
- Sans VAD: 100% d'appels à Whisper
- Avec VAD: ~30-40% d'appels à Whisper (60-70% économie)
- Latence VAD: < 1ms par frame

---

## Definition of Done

- [ ] VAD implémenté et fonctionnel
- [ ] Tests de détection passent
- [ ] Réduction mesurable des appels Whisper
- [ ] Seuils configurables
- [ ] Documentation complète
- [ ] Issue moved to done folder

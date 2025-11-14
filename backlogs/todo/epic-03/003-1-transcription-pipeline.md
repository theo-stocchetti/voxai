# [ISSUE-003.1] Pipeline de transcription en temps réel

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: 12h
**EPIC**: 3 - Transcription avec Whisper

---

## Description

Créer le pipeline complet audio → Whisper → texte avec gestion des chunks audio, fenêtres glissantes, et traitement concurrent pour minimiser la latence.

---

## Context

C'est le cœur de l'application. Le pipeline doit capturer l'audio en continu, le découper en chunks, transcrire avec Whisper, et produire du texte en temps réel avec une latence cible < 2 secondes.

---

## Acceptance Criteria

- [ ] Transcription temps réel fonctionnelle
- [ ] Latence < 2 secondes entre parole et texte affiché
- [ ] Gestion des chunks audio avec overlap pour continuité
- [ ] Thread pool pour parallélisation
- [ ] Gestion propre des erreurs (modèle manquant, etc.)
- [ ] Buffer de sortie pour accumuler le texte

---

## Technical Details

### Affected Components
- src/transcription/pipeline.rs
- src/transcription/chunking.rs
- src/transcription/whisper.rs (utilise)
- src/audio/capture.rs (utilise)

### Dependencies

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
crossbeam-channel = "0.5"
```

### Architecture du Pipeline

```
Audio Capture → VAD → Chunking → Whisper → Text Buffer → Output
     ↓            ↓        ↓          ↓           ↓          ↓
  RingBuf   Speech/Silent  Overlap  Parallel   Accumulate  Inject
```

### Implementation Notes
- Chunk size: 5-10 secondes d'audio (80k-160k samples @ 16kHz)
- Overlap: 1-2 secondes pour éviter coupures de mots
- Utiliser tokio pour async/await
- Channel pour communication entre threads
- Gérer la concurrence avec plusieurs chunks en parallèle

---

## Tasks Breakdown

- [ ] Créer module `src/transcription/pipeline.rs`
- [ ] Définir struct `TranscriptionPipeline`
- [ ] Implémenter système de chunking avec overlap
- [ ] Créer channels pour audio → transcription → text
- [ ] Implémenter worker pool pour transcription parallèle
- [ ] Intégrer VAD pour trigger la transcription
- [ ] Implémenter buffer de texte accumulé
- [ ] Gérer déduplication du texte (overlap)
- [ ] Ajouter métriques de performance (latence, throughput)
- [ ] Gérer pause/resume de la transcription
- [ ] Créer tests end-to-end

---

## Test Plan

### Unit Tests
- [ ] Test de chunking avec overlap
- [ ] Test de buffer accumulation
- [ ] Test de déduplication

### Integration Tests
- [ ] Test complet: audio → texte
- [ ] Mesurer la latence end-to-end
- [ ] Test avec fichier audio long (>1 minute)
- [ ] Vérifier que le texte est cohérent

### Manual Testing
- [ ] Parler pendant 30 secondes et vérifier le texte
- [ ] Tester avec pauses dans la parole
- [ ] Tester avec différents accents
- [ ] Mesurer la latence perçue

---

## Documentation Updates

- [ ] Documenter l'architecture du pipeline
- [ ] Ajouter diagramme de flux
- [ ] Documenter les paramètres de chunking
- [ ] Expliquer la gestion de la latence

---

## Related Issues

- Blocked by: #001.3 (Whisper integration), #002.1 (Audio capture), #002.3 (VAD)
- Blocks: #008.1, #008.2, #008.3 (Text injection)
- Related to: #003.2 (Model management)
- Part of: EPIC 3 - Transcription

---

## Notes

**Structure du code** :

```rust
// src/transcription/pipeline.rs
use tokio::sync::mpsc;
use crossbeam_channel::{Sender, Receiver};

pub struct TranscriptionPipeline {
    // Audio input
    audio_rx: Receiver<AudioChunk>,

    // Whisper context
    whisper: Arc<Mutex<WhisperContext>>,

    // Text output
    text_tx: Sender<TranscriptionResult>,

    // Configuration
    chunk_size: usize,      // Samples
    overlap_size: usize,    // Samples
    max_concurrent: usize,  // Max parallel transcriptions

    // State
    running: Arc<AtomicBool>,
}

pub struct AudioChunk {
    samples: Vec<f32>,
    timestamp: Instant,
}

pub struct TranscriptionResult {
    text: String,
    timestamp: Instant,
    confidence: f32,
}

impl TranscriptionPipeline {
    pub fn new(
        audio_rx: Receiver<AudioChunk>,
        text_tx: Sender<TranscriptionResult>,
        whisper_model: ModelSize,
    ) -> Result<Self> { ... }

    pub async fn start(&mut self) -> Result<()> {
        // Main loop
        while self.running.load(Ordering::Relaxed) {
            // 1. Receive audio chunk
            let chunk = self.audio_rx.recv()?;

            // 2. Check VAD (skip if silence)
            if !self.vad.is_speech(&chunk.samples) {
                continue;
            }

            // 3. Transcribe (spawn task)
            let whisper = self.whisper.clone();
            let text_tx = self.text_tx.clone();
            tokio::spawn(async move {
                let result = whisper.lock().await.transcribe(&chunk.samples)?;
                text_tx.send(result).await?;
                Ok::<(), Error>(())
            });
        }
    }

    pub fn stop(&mut self) { ... }
}
```

**Chunking strategy** :
- Chunk size: 10 secondes = 160,000 samples @ 16kHz
- Overlap: 2 secondes = 32,000 samples
- Fenêtre glissante: déplacer de 8 secondes à chaque fois

**Exemple de timeline** :
```
Chunk 1: [0s -------- 10s]
Chunk 2:         [8s -------- 18s]
Chunk 3:                 [16s -------- 26s]
           ↑overlap↑         ↑overlap↑
```

**Parallélisation** :
- Max 2-3 transcriptions en parallèle
- Évite saturation CPU/GPU
- Queue les chunks si trop de load

**Latency breakdown** :
- Audio buffering: ~500ms
- VAD processing: < 10ms
- Whisper inference: 500ms - 1.5s (selon modèle/hardware)
- Text injection: < 50ms
- **Total target: < 2s**

---

## Definition of Done

- [ ] Pipeline fonctionnel end-to-end
- [ ] Latence < 2s mesurée
- [ ] Tests passent
- [ ] Pas de memory leaks
- [ ] Documentation complète
- [ ] Issue moved to done folder

# TEAM 1 : CORE / BACKEND
## VoxAI - Pipeline Audio & Transcription

**Équipe**: Core/Backend
**Focus**: Audio capture, traitement, et transcription
**Durée**: 7 semaines (6 semaines de dev actif + 1 semaine support)
**Effectif**: 2 développeurs Rust senior

---

## 👥 Composition de l'Équipe

### Profils Recommandés

**Lead Développeur (Dev A)** :
- **Compétences** : Rust expert, traitement du signal audio
- **Expérience** : 3+ ans Rust, audio processing ou DSP
- **Responsabilités** :
  - Audio pipeline (capture, noise reduction, VAD)
  - Architecture du pipeline
  - Code reviews
  - Coordination avec équipe Infra

**Développeur ML (Dev B)** :
- **Compétences** : Rust + ML/AI, Whisper.cpp
- **Expérience** : Intégration de modèles ML, GPU programming
- **Responsabilités** :
  - Transcription pipeline (Whisper integration)
  - GPU acceleration (CUDA/Metal/OpenCL)
  - Model management
  - Performance optimization

### Outils et Environnement

**Stack technique** :
- Rust 1.75+ (edition 2021)
- CPAL 0.15 (audio capture)
- whisper-rs 0.10 (Whisper bindings)
- nnnoiseless 0.5 (noise reduction)
- webrtc-vad 0.4 (voice activity detection)
- ringbuf 0.3 (ring buffer)
- tokio 1.35 (async runtime)

**Hardware requis** :
- Dev A : Linux/Windows avec microphone
- Dev B : Machine avec GPU (CUDA ou Metal) pour tests
- Accès à différents périphériques audio pour tests

**Environnement de dev** :
- Rust toolchain (rustup)
- cargo, rustfmt, clippy
- whisper.cpp models (tiny, base, small)
- Audio test files (divers accents, bruits de fond)

---

## 🎯 Responsabilités Globales

### Modules Sous Responsabilité

1. **src/audio/** - Pipeline audio complet
   - `capture.rs` - Capture via CPAL
   - `device.rs` - Enumération des devices
   - `buffer.rs` - Ring buffer management
   - `noise_reduction.rs` - RNNoise integration
   - `vad.rs` - Voice Activity Detection

2. **src/transcription/** - Pipeline de transcription
   - `whisper.rs` - Whisper context wrapper
   - `models.rs` - Model management
   - `downloader.rs` - Model downloading (si temps)
   - `pipeline.rs` - Transcription pipeline
   - `chunking.rs` - Audio chunking logic

3. **src/gpu/** (si temps permet)
   - `cuda.rs` - NVIDIA GPU support
   - `metal.rs` - Apple Silicon support
   - `opencl.rs` - AMD/Intel support

### Livrables Principaux

- ✅ Audio capture fonctionnel avec CPAL
- ✅ Noise reduction et VAD
- ✅ Transcription pipeline end-to-end
- ✅ GPU acceleration (CUDA + Metal minimum)
- ✅ Model management
- ✅ Performance benchmarks
- ✅ Tests unitaires (80%+ coverage sur votre code)

---

## 📅 Planning Détaillé - 7 Semaines

### SEMAINE 1 : Whisper Integration
**Dates** : Jour 1-5
**Charge** : 60% (setup + intégration de base)

#### Objectifs
- Intégrer whisper-rs dans le projet
- Tester transcription avec fichier audio sample
- Comprendre l'API Whisper

#### Tâches Détaillées

**Dev A (Lead)** :
- [ ] Setup environnement Rust
- [ ] Review CLAUDE.md et architecture
- [ ] Créer structure `src/audio/`
- [ ] Implémenter `audio/mod.rs` avec traits de base
- [ ] Recherche CPAL best practices

**Dev B (ML)** :
- [ ] **Issue 001.3** : Whisper Integration (6h)
  - Intégrer whisper-rs dans Cargo.toml
  - Créer `src/transcription/whisper.rs`
  - Wrapper pour WhisperContext
  - Test avec audio sample (hello_world.wav)
  - Vérifier que transcription fonctionne
- [ ] Télécharger models Whisper (tiny, base)
- [ ] Créer `src/transcription/models.rs` (skeleton)

#### Sync Points
- **Lundi matin** : Kickoff avec toutes les équipes
- **Mercredi** : Demo Whisper avec fichier test
- **Vendredi** : Revue code + planning Week 2

#### Critères de Succès
- ✅ whisper-rs compile et s'intègre
- ✅ Transcription d'un fichier WAV réussit
- ✅ Structure `src/audio/` et `src/transcription/` créée

#### Dépendances
- **Bloqué par** : 001.2 (Build system) - Équipe Infra
- **Débloque** : 002.1 (Audio capture), 003.2 (Model management)

---

### SEMAINE 2 : Audio Capture Pipeline
**Dates** : Jour 6-10
**Charge** : 90% (feature development)

#### Objectifs
- Implémenter capture audio temps réel avec CPAL
- Créer ring buffer pour streaming
- Gestion des devices audio

#### Tâches Détaillées

**Dev A (Lead)** :
- [ ] **Issue 002.1** : Audio Capture avec CPAL (8h) 🔴 **CRITIQUE**
  - Implémenter `audio/capture.rs`
    - Enumération des devices
    - Sélection device par défaut
    - Stream audio en temps réel
    - Conversion sample rate (→ 16kHz pour Whisper)
  - Implémenter `audio/buffer.rs`
    - Ring buffer avec ringbuf crate
    - Thread-safe producer/consumer
    - Configurable size (default 10 secondes)
  - Implémenter `audio/device.rs`
    - Liste des input devices
    - Device info (sample rate, channels, etc.)
    - Device selection API
  - Tests unitaires :
    - Test enumération devices
    - Test capture (avec device virtuel si possible)
    - Test ring buffer (concurrent access)
  - Documentation rustdoc

**Dev B (ML)** :
- [ ] **Issue 003.2** : Model Management (6h)
  - Implémenter `transcription/models.rs`
    - Enum ModelSize (Tiny, Base, Small, Medium, Large)
    - Model paths (dans ~/.voxai/models/)
    - Model loading logic
    - Model info (size, language, etc.)
  - Créer config pour models
  - Tests : loading different models
  - Documentation
- [ ] Préparer pipeline transcription (skeleton)
- [ ] Tests avec différents models

#### Sync Points
- **Lundi** : Planning semaine + review code Week 1
- **Mercredi** : **CHECKPOINT** - Demo audio capture en live
- **Vendredi** : Revue de code + démo models

#### Critères de Succès
- ✅ Audio capturé depuis microphone par défaut
- ✅ Ring buffer fonctionne sans race conditions
- ✅ Models chargés correctement (tiny, base)
- ✅ Tests unitaires >= 80% coverage
- ✅ `cargo test` passe sans erreur

#### Dépendances
- **Bloqué par** : 001.2 (Build system) ✓
- **Débloque** : 002.2 (Noise reduction), 002.3 (VAD), 003.1 (Transcription)

#### Livrable Clé
**🎯 Milestone** : Audio capture fonctionnel - REQUIS pour continuer

---

### SEMAINE 3 : Audio Processing (Noise Reduction + VAD)
**Dates** : Jour 11-15
**Charge** : 100% (2 features en parallèle)

#### Objectifs
- Implémenter noise reduction avec RNNoise
- Implémenter Voice Activity Detection
- Intégrer dans pipeline audio

#### Tâches Détaillées

**Dev A (Lead)** :
- [ ] **Issue 002.2** : Noise Reduction (10h)
  - Intégrer nnnoiseless crate
  - Implémenter `audio/noise_reduction.rs`
    - DenoiseState wrapper
    - Process audio buffers
    - Configurable (on/off)
  - Tests :
    - Test avec audio bruité
    - Benchmark performance
    - Test avec/sans noise reduction
  - Tuning des paramètres
  - Documentation + exemples

**Dev B (ML)** :
- [ ] **Issue 002.3** : Voice Activity Detection (6h)
  - Intégrer webrtc-vad crate
  - Implémenter `audio/vad.rs`
    - VAD wrapper
    - Détection speech vs silence
    - Configurable aggressiveness (0-3)
    - State machine (speaking/silence)
  - Tests :
    - Test avec audio speech
    - Test avec silence
    - Test avec speech intermittent
    - Accuracy >= 90%
  - Documentation
- [ ] Commencer à concevoir transcription pipeline

#### Sync Points
- **Lundi** : Planning + revue code Week 2
- **Mercredi** : **CHECKPOINT** - Integration test audio pipeline
  - Audio capture → Noise reduction → VAD → Chunks
- **Vendredi** : Revue code + démo pipeline complet

#### Critères de Succès
- ✅ Noise reduction réduit bruit de fond (test auditif)
- ✅ VAD détecte speech vs silence >= 90% accuracy
- ✅ Pipeline audio complet fonctionne
- ✅ Tests d'intégration passent
- ✅ Performance acceptable (< 25% CPU)

#### Dépendances
- **Bloqué par** : 002.1 (Audio capture) ✓
- **Débloque** : 003.1 (Transcription pipeline)

#### Livrable Clé
**🎯 Milestone** : Pipeline audio complet (capture → noise reduction → VAD)

---

### SEMAINE 4 : Transcription Pipeline 🔴🔴🔴 CRITIQUE
**Dates** : Jour 16-20
**Charge** : 120% (overtime possible si nécessaire)

#### Objectifs
- Implémenter pipeline de transcription end-to-end
- Audio → Chunks → Whisper → Texte
- Atteindre latence < 3 secondes

#### Tâches Détaillées

**Dev A (Lead)** :
- [ ] **Issue 004.4** : CPU Optimizations (8h)
  - Profiling avec `cargo flamegraph`
  - Identifier bottlenecks
  - Optimisations :
    - Reduce allocations (use `Vec::with_capacity`)
    - Optimize ring buffer access
    - Parallelize audio processing si possible
    - SIMD pour audio processing (si applicable)
  - Benchmarks avant/après
  - Documentation des optimizations
- [ ] Support Dev B sur transcription pipeline
- [ ] Tests d'intégration end-to-end

**Dev B (ML)** :
- [ ] **Issue 003.1** : Transcription Pipeline (12h) 🔴🔴🔴 **BLOQUANT**
  - Implémenter `transcription/pipeline.rs`
    - Receive audio chunks from VAD
    - Queue chunks (tokio channel)
    - Transcribe avec Whisper
    - Return transcribed text
    - Handle errors gracefully
  - Implémenter `transcription/chunking.rs`
    - Chunk audio selon VAD
    - Overlap pour continuité
    - Min/max chunk size
  - Architecture async avec tokio
  - Tests end-to-end :
    - Audio sample → Transcription
    - Multiple chunks
    - Different languages (if time)
    - Error handling
  - Tuning paramètres (chunk size, overlap, etc.)
  - Documentation complète
- [ ] **CRITICAL**: Atteindre latence < 3 secondes
- [ ] **CRITICAL**: WER (Word Error Rate) < 15% sur test set

#### Sync Points
- **Lundi** : Planning + focus sur transcription
- **Mercredi à MIDI** : 🔴🔴🔴 **GO/NO-GO CRITIQUE**
  - **DEMO OBLIGATOIRE** : Audio en live → Texte
  - Si échec : Toutes les équipes déployées
  - Décision : Continuer ou débugger
- **Vendredi** : Revue finale + démo complète

#### Critères de Succès (GO/NO-GO)
- ✅ Audio en temps réel → Transcription fonctionne
- ✅ Latence < 5 secondes (target < 3s)
- ✅ Qualité acceptable (WER < 15%)
- ✅ Pipeline ne crash pas
- ✅ Tests end-to-end passent

#### Dépendances
- **Bloqué par** : 001.3 (Whisper) ✓, 002.1 (Audio) ✓, 002.3 (VAD) ✓
- **Débloque** : 008.1/2/3 (Text injection) - TEAM 3

#### Risques et Mitigation

**Risque 1** : Latence trop élevée
- Mitigation : Reduce chunk size, use tiny model, optimize

**Risque 2** : Whisper crashes
- Mitigation : Wrapper avec error handling, retry logic

**Risque 3** : Accuracy insuffisante
- Mitigation : Tester différents models, tuner paramètres VAD

**Si échec au Checkpoint Mercredi** :
- 🔴 Toute l'équipe Core focalisée sur déblocage
- 🔴 Équipes Infra et UI peuvent aider (review, tests)
- 🔴 Envisager fallback : skip VAD/noise reduction
- 🔴 Peut retarder projet de 1-2 semaines

#### Livrable Clé
**🎯🎯🎯 MILESTONE CRITIQUE** : Transcription end-to-end fonctionnelle
**C'EST LE GATE LE PLUS IMPORTANT DU PROJET**

---

### SEMAINE 5 : GPU Acceleration
**Dates** : Jour 21-25
**Charge** : 100% (GPU implementation)

#### Objectifs
- Implémenter support CUDA (NVIDIA)
- Implémenter support Metal (Apple Silicon)
- Implémenter support OpenCL (AMD/Intel)

#### Tâches Détaillées

**Dev A (Lead)** :
- [ ] **Issue 004.2** : Metal Support (10h)
  - Créer `src/gpu/metal.rs`
  - Intégration Metal backend pour Whisper
  - Tests sur macOS (Apple Silicon si dispo)
  - Feature flag `metal`
  - Benchmarks vs CPU
  - Documentation
- [ ] Support Dev B sur CUDA

**Dev B (ML)** :
- [ ] **Issue 004.1** : CUDA Support (12h)
  - Créer `src/gpu/cuda.rs`
  - Intégration CUDA backend pour Whisper
  - Vérifier CUDA toolkit disponible
  - Runtime check pour GPU NVIDIA
  - Feature flag `cuda`
  - Tests sur machine NVIDIA
  - Benchmarks :
    - Tiny model : X realtime
    - Base model : Y realtime
    - Small model : Z realtime
  - Documentation
- [ ] **Issue 004.3** : OpenCL Support (12h)
  - Créer `src/gpu/opencl.rs`
  - Intégration OpenCL backend
  - Tests sur GPU AMD/Intel
  - Feature flag `opencl`
  - Benchmarks
  - Documentation

#### Répartition Recommandée
- Dev A : Metal (2 jours) + support CUDA (1 jour)
- Dev B : CUDA (2 jours) + OpenCL (3 jours)

#### Sync Points
- **Lundi** : Planning + revue transcription Week 4
- **Mercredi** : CHECKPOINT - CUDA démo
- **Vendredi** : Démo GPU sur les 3 backends

#### Critères de Succès
- ✅ CUDA fonctionne sur GPU NVIDIA (si disponible)
- ✅ Metal fonctionne sur Apple Silicon (si disponible)
- ✅ OpenCL fonctionne (fallback)
- ✅ Speedup >= 2x vs CPU
- ✅ Feature flags permettent compilation sans GPU

#### Dépendances
- **Bloqué par** : 003.1 (Transcription pipeline) ✓
- **Débloque** : 010.3 (Performance benchmarks)

#### Notes
- GPU peut être optionnel pour MVP
- Si pas de hardware disponible, tests unitaires suffiront
- Prioriser CUDA et Metal (plus communs)

---

### SEMAINE 6 : Performance & Benchmarks
**Dates** : Jour 26-30
**Charge** : 80% (polish + benchmarks)

#### Objectifs
- Performance benchmarks complets
- Optimisations finales
- Support aux autres équipes

#### Tâches Détaillées

**Dev A (Lead)** :
- [ ] Code reviews pour équipe UI
- [ ] Debugging issues remontées par QA
- [ ] Performance tuning final
- [ ] Documentation code

**Dev B (ML)** :
- [ ] **Issue 010.3** : Performance Benchmarks (8h)
  - Créer `benches/transcription_bench.rs`
  - Benchmarks criterion.rs :
    - Audio capture latency
    - Noise reduction overhead
    - VAD detection time
    - Transcription speed (par model)
    - GPU vs CPU speedup
  - Benchmark report (Markdown)
  - Performance targets validation :
    - Latency < 2s ✓
    - CPU < 25% ✓
    - RAM < 500 MB ✓
  - Documentation
- [ ] **Issue 003.3** : Multi-language Support (en collaboration avec Infra)
  - Tests avec différentes langues
  - Config langue dans settings
  - Documentation

#### Sync Points
- **Lundi** : Planning + focus QA
- **Mercredi** : Revue benchmarks
- **Vendredi** : Démo performance finale

#### Critères de Succès
- ✅ Tous les benchmarks dans les targets
- ✅ 0 bugs critiques
- ✅ Code coverage >= 80%
- ✅ Documentation à jour

---

### SEMAINE 7 : Support & Documentation
**Dates** : Jour 31-35
**Charge** : 50% (support mode)

#### Objectifs
- Support aux équipes Infra et UI pour intégration
- Documentation développeur
- Fixes de bugs

#### Tâches Détaillées

**Dev A & B** :
- [ ] **Issue 012.2** : Developer Documentation (8h)
  - Architecture doc (`docs/architecture.md`)
  - Audio pipeline doc
  - Transcription pipeline doc
  - GPU integration guide
  - Troubleshooting guide
- [ ] **Issue 012.3** : API Documentation (8h)
  - Rustdoc pour toutes les API publiques
  - Exemples de code
  - Integration examples
- [ ] Support packaging (équipe UI)
- [ ] Final QA
- [ ] Bug fixes

#### Sync Points
- **Lundi** : Planning dernière semaine
- **Mercredi** : Revue documentation
- **Vendredi** : 🚀 **RELEASE READINESS REVIEW**

#### Critères de Succès
- ✅ Documentation complète
- ✅ Tous les bugs résolus
- ✅ Ready for release

---

## 🔗 Coordination avec Autres Équipes

### Avec TEAM 2 (Infra)

#### Dépendances Critiques

**Week 1** :
- **Vous êtes bloqué par** : 001.2 (Build system)
  - Ne pouvez pas compiler sans build system
  - **Action** : Daily check-in avec Infra
  - **Fallback** : Setup build local temporaire

**Week 2** :
- **Vous débloquez** : 010.1 (Unit tests) - Infra peut tester votre code
- **Communication** : Partager API audio dès que stable

**Week 5** :
- **Collaboration** : 003.3 (Multi-language) - Config de langue
  - Infra gère UI settings
  - Vous gérez model selection logic

**Week 6-7** :
- **Collaboration** : Documentation
  - Infra : Developer docs general
  - Vous : Audio/Transcription technical docs

#### Meetings Réguliers

- **Daily standup** (async Slack) : Partager blockers
- **Mercredi checkpoint** : Demo progrès
- **Vendredi review** : Code review mutuelle

---

### Avec TEAM 3 (UI)

#### Dépendances Critiques

**Week 4** :
- **Vous débloquez** : 008.x (Text injection) 🔴 CRITIQUE
  - Sans votre transcription pipeline, UI ne peut pas injecter de texte
  - **Checkpoint Mercredi Week 4 est VITAL**

**Week 6** :
- **Collaboration** : 009.1 (Status overlay)
  - UI affiche niveaux audio
  - Vous exposez audio levels API

#### API à Exposer

Créer des API claires pour que UI puisse s'intégrer :

```rust
// src/audio/mod.rs
pub trait AudioCapture {
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn get_audio_level(&self) -> f32; // Pour UI feedback
}

// src/transcription/mod.rs
pub trait TranscriptionPipeline {
    fn transcribe(&self, audio: &[f32]) -> Result<String>;
    fn get_status(&self) -> TranscriptionStatus; // Idle, Processing, Done
}
```

#### Communication

- **Week 4 Mercredi** : 🔴 DEMO OBLIGATOIRE pour UI
- **Weekly sync** : Partager changements API

---

## ✅ Critères de Succès par Semaine

### Week 1 : Whisper Integration
- [x] whisper-rs intégré et compile
- [x] Transcription fichier WAV fonctionne
- [x] Structure code créée

### Week 2 : Audio Capture (CRITIQUE)
- [x] Audio capturé depuis micro
- [x] Ring buffer thread-safe
- [x] Models Whisper chargés
- [x] Tests unitaires >= 80%

### Week 3 : Audio Processing
- [x] Noise reduction réduit bruit
- [x] VAD >= 90% accuracy
- [x] Pipeline audio complet
- [x] Tests intégration passent

### Week 4 : Transcription Pipeline (GO/NO-GO)
- [x] Audio live → Transcription fonctionne
- [x] Latence < 3 secondes
- [x] WER < 15%
- [x] Tests end-to-end passent

### Week 5 : GPU Acceleration
- [x] CUDA fonctionne (si GPU dispo)
- [x] Metal fonctionne (si Apple Silicon)
- [x] OpenCL fonctionne
- [x] Speedup >= 2x

### Week 6 : Benchmarks
- [x] Benchmarks dans targets
- [x] Performance validée
- [x] 0 bugs critiques

### Week 7 : Documentation
- [x] Documentation complète
- [x] API documentée
- [x] Ready for release

---

## 📊 Métriques et KPIs

### Code Quality

**Coverage** :
- Target : >= 80%
- Critique : Audio capture, Transcription >= 90%

**Tests** :
- Unit tests : >= 50 tests
- Integration tests : >= 10 scenarios
- Benchmarks : >= 5 benchmarks

### Performance

**Latency** :
- Target : < 2 secondes
- Acceptable : < 3 secondes
- Critique : < 5 secondes

**CPU Usage** :
- Target : < 15%
- Acceptable : < 25%
- Max : < 40%

**RAM Usage** :
- Target : < 300 MB
- Acceptable : < 500 MB
- Max : < 800 MB

**Transcription Speed** :
- Tiny (CPU) : >= 10x realtime
- Base (CPU) : >= 7x realtime
- Small (CPU) : >= 4x realtime
- Small (GPU) : >= 8x realtime

### Quality

**Word Error Rate (WER)** :
- Target : < 10%
- Acceptable : < 15%
- Max : < 20%

**Voice Activity Detection** :
- Accuracy : >= 90%
- False positives : < 5%

---

## 🛠️ Outils et Processus

### Development Workflow

**Branching** :
```bash
# Créer branch pour chaque issue
git checkout -b core/002-1-audio-capture

# Commit réguliers
git commit -m "feat(audio): implement CPAL capture"

# Push et PR
git push -u origin core/002-1-audio-capture
```

**Testing** :
```bash
# Run tests avant chaque commit
cargo test

# Run tests avec output
cargo test -- --nocapture

# Run benchmarks
cargo bench

# Check coverage
cargo tarpaulin --out Html
```

**Linting** :
```bash
# Format code
cargo fmt

# Lint avec clippy
cargo clippy -- -D warnings
```

### Code Review

**Process** :
1. Dev termine feature
2. Self-review du code
3. Run tests + clippy
4. Créer PR avec description
5. Review par autre dev de l'équipe
6. Review par équipe Infra (si API changes)
7. Merge après 1 approval minimum

**PR Template** :
```markdown
## Issue
Closes #002-1

## Changes
- Implement audio capture with CPAL
- Add ring buffer for streaming
- Add device enumeration

## Tests
- [x] Unit tests added
- [x] Integration tests pass
- [x] Manual testing done

## Performance
- Latency: 50ms
- CPU: 10%
- RAM: 100 MB
```

---

## 🚨 Gestion des Risques

### Risques Identifiés

#### Risque 1 : Whisper.rs instable
**Probabilité** : Moyenne
**Impact** : 🔴 Critique

**Mitigation** :
- Tester différentes versions whisper-rs
- Fork whisper-rs si nécessaire
- Fallback : Intégrer whisper.cpp directement

#### Risque 2 : Latence trop élevée
**Probabilité** : Moyenne
**Impact** : 🟠 Haute

**Mitigation** :
- Profiling early (Week 2-3)
- Optimizations continues
- Use smaller model (tiny)
- Reduce chunk size

#### Risque 3 : GPU non disponible pour tests
**Probabilité** : Moyenne
**Impact** : 🟡 Moyenne

**Mitigation** :
- GPU optionnel pour MVP
- Tests CPU suffisent
- Utiliser CI/CD avec GPU runners si possible
- Cloud GPU pour tests (AWS, GCP)

#### Risque 4 : Week 4 transcription pipeline bloqué
**Probabilité** : Faible
**Impact** : 🔴🔴🔴 Bloquant

**Mitigation** :
- Prototyper dès Week 2-3
- Daily check-ins Week 4
- Toute l'équipe Core focalisée Week 4
- Escalation immédiate si problème

---

## 📚 Ressources et Documentation

### Documentation Technique

**Rust** :
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

**Audio** :
- [CPAL documentation](https://docs.rs/cpal/)
- [RNNoise paper](https://jmvalin.ca/demo/rnnoise/)
- [WebRTC VAD](https://webrtc.org/)

**Whisper** :
- [whisper.cpp](https://github.com/ggerganov/whisper.cpp)
- [whisper-rs](https://docs.rs/whisper-rs/)
- [Whisper paper](https://arxiv.org/abs/2212.04356)

**GPU** :
- [CUDA programming guide](https://docs.nvidia.com/cuda/)
- [Metal documentation](https://developer.apple.com/metal/)
- [OpenCL guide](https://www.khronos.org/opencl/)

### Repositories

**GitHub** :
- Main repo : `theo-stocchetti/voxai`
- Branch convention : `core/XXX-Y-feature-name`

**Issues** :
- Backlog local : `backlogs/todo/epic-XX/`
- Move to done : `backlogs/done/`

---

## 💬 Communication

### Channels Slack/Discord

**Votre channel principal** : `#core-dev`
- Discussions techniques audio/transcription
- Code reviews
- Debugging sessions

**Autres channels** :
- `#general` : Annonces générales
- `#blockers` : 🔴 Signalement blocages URGENT
- `#integration` : Tests intégration inter-équipes
- `#qa` : Qualité et testing

### Daily Standup (Async)

**Format** (chaque matin à 9h sur Slack) :
```
👋 [Votre nom]
✅ Hier: Implemented audio capture with CPAL
🎯 Aujourd'hui: Implement ring buffer + tests
🚧 Blockers: None
```

### Weekly Meetings

**Lundi 9h** : Planning de la semaine (30 min)
**Mercredi 14h** : Checkpoint technique (45 min)
**Vendredi 16h** : Revue + démo (1h)

---

## 🎯 Checklist Finale (End of Week 7)

### Code
- [ ] Tous les modules implémentés
- [ ] Tests >= 80% coverage
- [ ] `cargo test` passe 100%
- [ ] `cargo clippy` sans warnings
- [ ] Code formaté (`cargo fmt`)

### Fonctionnel
- [ ] Audio capture fonctionne
- [ ] Noise reduction réduit bruit
- [ ] VAD détecte voix >= 90%
- [ ] Transcription end-to-end fonctionne
- [ ] Latence < 2 secondes
- [ ] GPU fonctionne (CUDA + Metal)

### Performance
- [ ] Benchmarks dans targets
- [ ] CPU < 25%
- [ ] RAM < 500 MB
- [ ] WER < 15%

### Documentation
- [ ] Architecture doc complète
- [ ] API documentée (rustdoc)
- [ ] Developer guide
- [ ] Troubleshooting guide

### Qualité
- [ ] 0 bugs critiques
- [ ] < 5 bugs haute priorité
- [ ] Code reviewé
- [ ] Ready for release

---

## 🚀 Let's Build!

Vous êtes l'équipe **Core/Backend** - le cœur du système VoxAI. Votre travail est **critique** pour le succès du projet. Sans un pipeline audio et transcription robuste, rien d'autre ne fonctionne.

**Priorités absolues** :
1. 🔴 Week 2 : Audio capture MUST work
2. 🔴🔴🔴 Week 4 Mercredi : Transcription MUST work (GO/NO-GO)
3. 🟠 Week 5 : GPU acceleration (nice to have)

**Communication is key** :
- Daily standups
- Signal blockers immediately on #blockers
- Demo early, demo often
- Ask for help when needed

**Vous avez ça ! 💪**

Pour questions : #core-dev sur Slack

Références :
- CLAUDE.md
- PARALLELIZATION_ANALYSIS.md
- TEAM_ALLOCATION_PLAN.md

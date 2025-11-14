# VoxAI - Timeline Gantt et Diagrammes

**Date**: 2025-11-14
**Référence**: TEAM_ALLOCATION_PLAN.md

---

## Comparaison des Scénarios

### Vue d'Ensemble

| Critère | 2 Équipes | 3 Équipes |
|---------|-----------|-----------|
| **Durée totale** | 7-8 semaines | 6-7 semaines |
| **Développeurs** | 4-6 personnes | 5-7 personnes |
| **Complexité coordination** | 🟢 Faible | 🟡 Moyenne |
| **Parallélisation** | 70% | 85% |
| **Risque de blocages** | 🟡 Moyen | 🟢 Faible |
| **Coût** | 💰💰 Moyen | 💰💰💰 Élevé |
| **Recommandation** | PME/Startup | Scale-up/Enterprise |

---

## Scénario 2 Équipes - Gantt

### Configuration
- **Équipe 1 (Core)**: 2-3 développeurs
- **Équipe 2 (UI)**: 2-3 développeurs
- **Total**: 4-6 développeurs
- **Durée**: 7-8 semaines

### Timeline Détaillée

```
SEMAINE 1: Foundation & Infrastructure
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [001.2 Build ████████] [007.1 Config ████]
ÉQUIPE UI:      [005.4 Icons ████░░░░░░░░░░░░░░░░░░░░░░░░]

Livrables: ✅ Build system, Config, Icons, Whisper intégré
═══════════════════════════════════════════════════════════


SEMAINE 2: Audio Pipeline & Settings UI
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [002.1 Audio Capture ████████████]
                [010.1 Unit Tests ████████]
ÉQUIPE UI:      [007.2 Settings UI ████████████████]

Livrables: ✅ Audio capture, Tests unitaires, Settings UI
═══════════════════════════════════════════════════════════


SEMAINE 3: Audio Processing & Hotkeys
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [002.2 Noise Reduction ██████████]
                [002.3 VAD ██████]
                [003.2 Model Management ██████]
ÉQUIPE UI:      [006.1 Hotkey Win ██████]
                [006.2 Hotkey macOS ██████]
                [006.3 Hotkey Linux ████████]

Livrables: ✅ Pipeline audio complet, Hotkeys 3 plateformes
Checkpoint: 🎯 Integration test
═══════════════════════════════════════════════════════════


SEMAINE 4: Transcription Pipeline & System Tray
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [003.1 Transcription ████████████████] 🔴 CRITIQUE
                [010.2 Integration Tests ██████████]
                [004.4 CPU Optimizations ████████]
ÉQUIPE UI:      [005.1 Windows Tray ████████]
                [005.2 macOS Tray ████████]
                [005.3 Linux Tray ████████]

Livrables: ✅ Transcription end-to-end, Tray 3 plateformes
Checkpoint: 🎯🎯🎯 CRITIQUE - Transcription démo obligatoire
═══════════════════════════════════════════════════════════


SEMAINE 5-6: GPU Acceleration & Text Injection
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [004.1 CUDA ████████████]
                [004.2 Metal ██████████]
                [003.3 Multi-language ████████]
ÉQUIPE UI:      [008.1 Text Injection Win ██████]
                [008.2 Text Injection macOS ██████]
                [008.3 Text Injection Linux ████████]
                [006.4 Hotkey Config ██████]

Livrables: ✅ GPU support, Text injection 3 plateformes
Checkpoint: 🎯 MVP end-to-end fonctionnel
═══════════════════════════════════════════════════════════


SEMAINE 6-7: Polish & Visual Feedback
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [004.3 OpenCL ████████████]
                [010.3 Benchmarks ████████]
                [007.3 Audio Device ██████]
ÉQUIPE UI:      [009.1 Status Overlay ████████]
                [009.2 Notifications ████]
                [008.4 Text Processing ██████]
                [009.3 Performance UI ██████]

Livrables: ✅ GPU complet, Feedback visuel, Post-processing
Checkpoint: 🎯 QA review
═══════════════════════════════════════════════════════════


SEMAINE 7-8: Packaging & Documentation
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [012.2 Dev Docs ████████]
                [012.3 API Docs ████████]
ÉQUIPE UI:      [011.1 Windows Package ████████]
                [011.2 macOS Package ████████]
                [011.3 Linux Package ██████████]
                [011.4 Auto-update ████████████]
                [012.1 User Docs ████████]

Livrables: ✅ Packages 3 plateformes, Auto-update, Docs
Checkpoint: 🎯 Release readiness review
═══════════════════════════════════════════════════════════

🚀 RELEASE MVP
```

---

## Scénario 3 Équipes - Gantt

### Configuration
- **Équipe 1 (Core)**: 2 développeurs
- **Équipe 2 (Infra)**: 1-2 développeurs
- **Équipe 3 (UI)**: 2 développeurs
- **Total**: 5-6 développeurs
- **Durée**: 6-7 semaines

### Timeline Détaillée

```
SEMAINE 1: Foundation Parallélisée
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [001.3 Whisper ██████░░░░░░░░░░░░░░░░]
ÉQUIPE INFRA:   [001.2 Build ████████] [007.1 Config ████]
ÉQUIPE UI:      [005.4 Icons ████░░░░░░░░░░░░░░░░░░░░]

Livrables: ✅ Build, Config, Whisper, Icons
Sync Point: 🔴 End of Week 1 - Go/No-Go
═══════════════════════════════════════════════════════════


SEMAINE 2: Audio & Config UI & Hotkeys
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [002.1 Audio Capture ████████████]
                [003.2 Model Mgmt ██████]
ÉQUIPE INFRA:   [007.2 Settings UI ████████████████]
                [010.1 Unit Tests ████████]
ÉQUIPE UI:      [006.1 Win Hotkey ██████]
                [006.2 macOS Hotkey ██████]
                [006.3 Linux Hotkey ████████]

Livrables: ✅ Audio, Settings UI, Hotkeys 3 plateformes
Sync Point: 🔴 End of Week 2 - Audio capture demo
═══════════════════════════════════════════════════════════


SEMAINE 3: Audio Processing & Tray
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [002.2 Noise Reduction ██████████]
                [002.3 VAD ██████]
ÉQUIPE INFRA:   [010.2 Integration Tests ██████████]
ÉQUIPE UI:      [005.1 Windows Tray ████████]
                [005.2 macOS Tray ████████]
                [005.3 Linux Tray ████████]

Livrables: ✅ Pipeline audio, Tests intégration, Tray
Sync Point: 🔴 End of Week 3 - Integration checkpoint
═══════════════════════════════════════════════════════════


SEMAINE 4: Transcription Pipeline (CRITIQUE)
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [003.1 Transcription ████████████████] 🔴🔴🔴
                [004.4 CPU Optimizations ████████]
ÉQUIPE INFRA:   [006.4 Hotkey Config ██████]
                [007.3 Audio Device ██████]
ÉQUIPE UI:      [Prototypage Text Injection ████████]

Livrables: ✅ Transcription end-to-end, Config avancée
Sync Point: 🔴🔴🔴 CRITIQUE - Mid-Week 4 Transcription MUST work
⚠️ SI ÉCHEC: Toutes les équipes déployées pour débloquer
═══════════════════════════════════════════════════════════


SEMAINE 5: GPU & Text Injection
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [004.1 CUDA ████████████]
                [004.2 Metal ██████████]
                [004.3 OpenCL ████████████]
ÉQUIPE INFRA:   [003.3 Multi-language ████████]
                [010.3 Benchmarks ████████]
ÉQUIPE UI:      [008.1 Text Injection Win ██████]
                [008.2 Text Injection macOS ██████]
                [008.3 Text Injection Linux ████████]

Livrables: ✅ GPU complet, Text injection, Multi-langue
Sync Point: 🟠 End of Week 5 - MVP fonctionnel
🎯 Milestone: MVP end-to-end démo-able
═══════════════════════════════════════════════════════════


SEMAINE 6: Polish & Visual Feedback
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [QA Support ░░░░░░░░░░░░░░░░░░░░░░░░]
ÉQUIPE INFRA:   [012.2 Dev Docs ████████]
                [012.3 API Docs ████████]
ÉQUIPE UI:      [009.1 Status Overlay ████████]
                [009.2 Notifications ████]
                [008.4 Text Processing ██████]
                [009.3 Performance UI ██████]

Livrables: ✅ Feedback visuel, Post-processing, Dev docs
═══════════════════════════════════════════════════════════


SEMAINE 7: Deployment
═══════════════════════════════════════════════════════════
ÉQUIPE CORE:    [Final QA ██████]
                [Performance Tuning ██████]
ÉQUIPE INFRA:   [011.4 Auto-update ████████████]
ÉQUIPE UI:      [011.1 Windows Package ████████]
                [011.2 macOS Package ████████]
                [011.3 Linux Package ██████████]
                [012.1 User Docs ████████]

Livrables: ✅ Packages, Auto-update, Documentation complète
Sync Point: 🟢 End of Week 7 - Release readiness
═══════════════════════════════════════════════════════════

🚀 RELEASE MVP
```

---

## Chemin Critique - Diagramme de Dépendances

### Vue Globale

```
                    ┌─────────────┐
                    │   START     │
                    └──────┬──────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
           ▼               ▼               ▼
    ┌─────────┐    ┌──────────┐    ┌──────────┐
    │ 001.2   │    │  007.1   │    │  005.4   │
    │  Build  │    │  Config  │    │  Icons   │
    │  6h     │    │  4h      │    │  4h      │
    └────┬────┘    └─────┬────┘    └────┬─────┘
         │               │               │
         │      ┌────────┴───────┐       │
         │      │                 │       │
         ▼      ▼                 ▼       │
    ┌─────────┐ ┌──────────┐             │
    │ 002.1   │ │  007.2   │             │
    │  Audio  │ │ Settings │             │
    │  8h     │ │  12h     │             │
    └────┬────┘ └─────┬────┘             │
         │            │                   │
    ┌────┴────┐       │                   │
    │         │       │                   │
    ▼         ▼       │                   │
┌────────┐┌────────┐  │                   │
│ 002.2  ││ 002.3  │  │                   │
│ Noise  ││  VAD   │  │                   │
│  10h   ││  6h    │  │                   │
└───┬────┘└───┬────┘  │                   │
    │         │       │                   │
    └────┬────┘       │                   │
         │            │                   │
         ▼            │                   │
    ┌─────────┐       │                   │
    │ 003.1   │◄──────┘                   │
    │ Transc. │                           │
    │  12h    │  🔴🔴🔴 CRITIQUE           │
    └────┬────┘                           │
         │                                │
    ┌────┴────┐                           │
    │         │                           │
    ▼         ▼                           ▼
┌────────┐┌────────┐┌────────┐    ┌──────────┐
│ 008.1  ││ 008.2  ││ 008.3  │    │  005.x   │
│ Text   ││ Text   ││ Text   │◄───┤  Tray    │
│ Win 6h ││ macOS  ││ Linux  │    │  8h      │
└────────┘│  6h    ││  8h    │    └──────────┘
          └────────┘└────────┘
                 │
                 ▼
          ┌──────────┐
          │ 011.x    │
          │ Package  │
          │ 8-10h    │
          └────┬─────┘
               │
               ▼
          ┌──────────┐
          │ RELEASE  │
          └──────────┘
```

### Explication du Chemin Critique

**Séquence obligatoire** (ne peut pas être parallélisée) :

1. **001.2** (6h) → Build system
2. **002.1** (8h) → Audio capture
3. **002.2 + 002.3** (10h parallèle) → Noise reduction + VAD
4. **003.1** (12h) → Transcription pipeline 🔴
5. **008.x** (8h parallèle) → Text injection

**Total chemin critique**: ~42 heures de travail séquentiel

**Durée calendaire minimale**: 5-6 semaines (avec parallélisation des autres tâches)

---

## Matrice de Parallélisation

### Vague par Vague

| Vague | Issues Total | Séquentielles | Parallèles | % Parallel | Max Devs |
|-------|--------------|---------------|------------|------------|----------|
| **1** | 3 | 0 | 3 | 100% | 3 |
| **2** | 5 | 1 | 4 | 80% | 4 |
| **3** | 2 | 0 | 2 | 100% | 2 |
| **4** | 7 | 1 | 6 | 85% | 6 |
| **5** | 11 | 0 | 11 | 100% | 11 |
| **6** | 5 | 0 | 5 | 100% | 5 |
| **7** | 8 | 1 | 7 | 87% | 7 |
| **Total** | **41** | **3** | **38** | **~93%** | **6-8** |

### Interprétation

- **93% des tâches sont parallélisables** (avec assez de développeurs)
- **3 tâches séquentielles critiques**: 001.2, 002.1, 003.1
- **Optimal team size**: 6-8 développeurs pour maximiser la parallélisation
- **Pratique**: 4-6 développeurs est un bon compromis coût/vitesse

---

## Allocation des Ressources

### Scénario 2 Équipes (4-6 devs)

```
Semaine    Core (2-3)          UI (2-3)           Charge
═══════    ══════════          ════════           ══════
Week 1     ██████ (80%)        ████ (50%)         65%
Week 2     ████████ (100%)     ████████ (100%)    100%
Week 3     ████████ (100%)     ████████ (100%)    100%
Week 4     ██████████ (120%)   ████████ (100%)    110% ⚠️
Week 5     ████████ (100%)     ██████████ (120%)  110% ⚠️
Week 6     ████████ (100%)     ████████ (100%)    100%
Week 7     ██████ (80%)        ██████████ (120%)  100%
Week 8     ████ (50%)          ████████ (100%)    75%

Moyenne charge: 93%
Pics: Semaines 4-5 (besoin de flexibilité)
```

### Scénario 3 Équipes (5-6 devs)

```
Semaine    Core (2)    Infra (1-2)   UI (2)      Charge
═══════    ════════    ═══════════   ══════      ══════
Week 1     ████ (60%)  ██████ (80%)  ████ (50%)  63%
Week 2     ██████ (90%) ████████ (100%) ████████ (100%) 97%
Week 3     ████████ (100%) ██████ (80%) ████████ (100%) 93%
Week 4     ██████████ (120%) ██████ (80%) ████████ (100%) 100%
Week 5     ████████ (100%) ████████ (100%) ██████████ (120%) 107% ⚠️
Week 6     ████ (50%)  ████████ (100%) ████████ (100%) 83%
Week 7     ████ (50%)  ██████████ (120%) ██████████ (120%) 97%

Moyenne charge: 91%
Pics: Semaine 5 (GPU + Text injection)
```

---

## Points de Décision

### Go/No-Go Gates

#### Gate 1 : End of Week 1 (Foundation)
**Critères de passage** :
- ✅ `cargo build` réussit sur Windows/macOS/Linux
- ✅ Config se sauvegarde et se charge
- ✅ Whisper.rs intégré (test avec audio sample)
- ✅ Icônes exportées (3 états minimum)

**Si échec** :
- 🔴 STOP : Débloquer immédiatement
- 🔴 Toutes les équipes aident
- 🔴 Ne PAS continuer vers Week 2

---

#### Gate 2 : End of Week 2 (Audio Capture)
**Critères de passage** :
- ✅ Audio capturé depuis microphone par défaut
- ✅ Tests unitaires audio passent (>= 80% coverage)
- ✅ Settings UI affiche au moins 3 options
- ✅ Config modifiable via UI

**Si échec** :
- 🟠 RALENTIR : Core se concentre sur audio
- 🟠 UI continue avec mocks
- 🟠 Peut retarder de 2-3 jours

---

#### Gate 3 : End of Week 3 (Audio Processing)
**Critères de passage** :
- ✅ Noise reduction réduit bruit de fond
- ✅ VAD détecte speech vs silence (>= 90% accuracy)
- ✅ Hotkeys fonctionnent sur les 3 OS
- ✅ Tests d'intégration passent

**Si échec** :
- 🟡 CONTINUER avec fallback
- 🟡 Transcription peut continuer sans VAD
- 🟡 Noise reduction en beta

---

#### Gate 4 : Mid-Week 4 (Transcription Pipeline) 🔴🔴🔴
**Critères de passage** :
- ✅ Audio → Whisper → Texte fonctionne
- ✅ Latence < 5 secondes (target < 2s)
- ✅ WER (Word Error Rate) < 15% sur test set
- ✅ Pipeline ne crash pas

**Si échec** :
- 🔴🔴🔴 ALERTE : Toutes les équipes déployées
- 🔴🔴🔴 Peut retarder projet de 1-2 semaines
- 🔴🔴🔴 Envisager:
  - Simplifier pipeline (skip VAD/noise)
  - Utiliser modèle plus simple (tiny)
  - Augmenter chunk size
  - Debug avec équipe Whisper.rs

**⚠️ C'EST LE GATE LE PLUS CRITIQUE DU PROJET**

---

#### Gate 5 : End of Week 5 (MVP Complet)
**Critères de passage** :
- ✅ Démo end-to-end sur 3 OS
- ✅ Text injection fonctionne
- ✅ GPU accélération fonctionne (si GPU disponible)
- ✅ MVP utilisable

**Si échec** :
- 🟡 CONTINUER : Features de polish peuvent avancer
- 🟡 MVP peut être livré sans GPU
- 🟡 Text injection critique, reste bloquant

---

#### Gate 6 : End of Week 7 (Release Readiness)
**Critères de passage** :
- ✅ Packages installables sur 3 OS
- ✅ Documentation complète
- ✅ Tous les tests passent (>= 80% coverage)
- ✅ Auto-update fonctionne
- ✅ QA approuvée (0 critical bugs)

**Si échec** :
- 🟢 RETARDER : Release peut être reportée
- 🟢 Beta release possible
- 🟢 Phase de polish étendue

---

## Recommandations par Contexte

### Pour Startup / PME (Budget limité)

**Recommandation** : **2 Équipes (4-5 devs)**

**Pourquoi** :
- Coût optimisé
- Coordination simple
- Timeline acceptable (7-8 semaines)
- Moins de overhead

**Composition suggérée** :
- Core: 2 devs Rust seniors
- UI: 2 devs avec expérience multi-plateforme
- 1 dev peut "flotter" entre équipes

---

### Pour Scale-up / Enterprise (Budget confortable)

**Recommandation** : **3 Équipes (6 devs)**

**Pourquoi** :
- Timeline plus rapide (6-7 semaines)
- Meilleure spécialisation
- Moins de risque de blocage
- Infra dédiée réduit les goulots

**Composition suggérée** :
- Core: 2 devs Rust experts (audio/ML)
- Infra: 2 devs DevOps + Rust
- UI: 2 devs multi-plateforme (1 avec exp. Windows, 1 avec exp. macOS)

---

### Pour Agence / Consultant

**Recommandation** : **2 Équipes modulaires**

**Pourquoi** :
- Flexibilité pour ajuster ressources
- Facturation plus claire
- Peut scale up/down selon phase

**Modèle** :
- Phase 1-2 (Week 1-3): 4 devs
- Phase 3-4 (Week 4-5): 6 devs (ajout de 2 devs UI)
- Phase 5-7 (Week 6-8): 4 devs (retour à équipe de base)

---

## Métriques de Succès

### Définition de "Done"

**MVP Ready to Ship** = Tous les critères ci-dessous :

#### Fonctionnel
- [ ] Transcription fonctionne sur Windows/macOS/Linux
- [ ] Latence < 3 secondes (target < 2s)
- [ ] Text injection fonctionne dans 95% des apps
- [ ] Hotkey global fonctionne de manière fiable
- [ ] System tray responsive
- [ ] Settings sauvegardées et appliquées

#### Qualité
- [ ] Code coverage >= 80%
- [ ] Tous les tests passent (unit + integration)
- [ ] 0 critical bugs
- [ ] < 5 high-priority bugs
- [ ] Performance benchmarks dans les targets

#### Déploiement
- [ ] Packages installables sur 3 OS
- [ ] Auto-update fonctionne
- [ ] Documentation utilisateur complète
- [ ] Documentation développeur complète
- [ ] README avec quick start

#### Expérience Utilisateur
- [ ] Installation < 5 minutes
- [ ] First transcription < 2 minutes
- [ ] Interface intuitive (user testing)
- [ ] Feedback visuel clair
- [ ] Error messages compréhensibles

---

## Conclusion

### Résumé Exécutif

**Pour 2 équipes (4-6 devs)** :
- ✅ Timeline: 7-8 semaines
- ✅ Coût optimisé
- ✅ Coordination simple
- ⚠️ Pics de charge en semaines 4-5

**Pour 3 équipes (5-6 devs)** :
- ✅ Timeline: 6-7 semaines
- ✅ Meilleure parallélisation
- ✅ Moins de risque de blocage
- ⚠️ Coordination plus complexe

### Facteurs de Succès Clés

1. **Démarrer immédiatement avec Vague 1** (Build + Config + Icons)
2. **Sync Point Week 4 est CRITIQUE** (Transcription pipeline)
3. **Paralléliser max en Vague 5** (3 plateformes en même temps)
4. **Communication quotidienne** (daily standups)
5. **Tests continus** (ne pas attendre la fin)

### Next Steps

1. ✅ Lire ce document + TEAM_ALLOCATION_PLAN.md
2. ✅ Choisir scénario 2 ou 3 équipes
3. ✅ Constituer les équipes
4. ✅ Setup infrastructure (repos, CI/CD, comms)
5. ✅ Kickoff Week 1 - Vague 1

**Bonne chance ! 🚀**

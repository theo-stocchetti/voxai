# VoxAI - Guide de Synchronisation Inter-Équipes
## Auto-Organisation Sans Coordinateurs

**Date**: 2025-11-14
**Modèle**: Équipes auto-organisées avec synchronisation légère
**Équipes**: 3 (Core, Infra, UI)

---

## 📋 Table des Matières

1. [Philosophie](#philosophie)
2. [Structure de Communication](#structure-de-communication)
3. [Sync Points Critiques](#sync-points-critiques)
4. [Processus de Décision](#processus-de-décision)
5. [Gestion des Blocages](#gestion-des-blocages)
6. [Outils](#outils)
7. [Protocoles](#protocoles)

---

## Philosophie

### Principes d'Auto-Organisation

**VoxAI fonctionne sans coordinateurs** - les équipes s'auto-organisent en suivant ces principes :

1. **Autonomie** : Chaque équipe prend ses propres décisions techniques
2. **Transparence** : Toute information est partagée ouvertement
3. **Responsabilité** : Chaque équipe est responsable de ses livrables
4. **Collaboration** : Les équipes s'entraident spontanément
5. **Communication asynchrone first** : Minimiser les réunions

### Pas de Coordinateurs = Responsabilité Distribuée

**Ce qui change** :
- ❌ Pas de manager qui assigne les tâches
- ❌ Pas de coordinateur qui résout les conflits
- ❌ Pas de chef de projet qui prend les décisions

**Ce qui reste** :
- ✅ Chaque équipe a ses responsabilités claires (voir TEAM*.md)
- ✅ Les dépendances sont documentées et suivies
- ✅ Les sync points sont obligatoires et structurés
- ✅ Les problèmes sont escaladés rapidement

---

## Structure de Communication

### Channels de Communication

#### Slack/Discord Setup

**Channels publics** (tout le monde peut voir) :

```
#general
  - Annonces générales
  - Celebrations (milestones, releases)
  - Questions générales

#blockers 🔴 PRIORITÉ
  - Signalement des blocages
  - Response time : < 2h
  - Toute l'équipe notifiée
  - Résolution collaborative

#integration
  - Tests d'intégration inter-équipes
  - API changes
  - Breaking changes
  - Coordination technique

#qa
  - Bugs reports
  - Test results
  - Quality discussions

#demos
  - Annonces de demos
  - Screenshots/videos
  - Feedback utilisateur
```

**Channels par équipe** :

```
#team-core
  - Discussions techniques audio/transcription
  - Code reviews internes
  - Planification interne

#team-infra
  - Discussions build/config/tests
  - CI/CD
  - Planification interne

#team-ui
  - Discussions UI/plateform
  - Design decisions
  - Planification interne
```

#### Quand Utiliser Quel Channel ?

| Situation | Channel | Délai Réponse |
|-----------|---------|---------------|
| Je suis bloqué | `#blockers` | < 2h |
| Question technique | `#integration` ou channel équipe | < 4h |
| Bug trouvé | `#qa` | < 1 jour |
| Annonce générale | `#general` | N/A |
| Demo à partager | `#demos` | N/A |

---

## Sync Points Critiques

### Vue d'Ensemble

Les sync points sont **obligatoires** et **non-négociables**. Ce sont les moments où les équipes se synchronisent pour éviter les désalignements.

### Sync Hebdomadaire

#### Lundi Matin : Planning de la Semaine
**Heure** : 9h00
**Durée** : 30 minutes MAX
**Format** : Visio (Google Meet/Zoom) ou Slack huddle
**Participants** : Toutes les équipes (min 1 personne par équipe)

**Agenda** :
1. **Revue semaine précédente** (5 min)
   - Qu'est-ce qui a été livré ?
   - Qu'est-ce qui n'a pas été livré ? Pourquoi ?

2. **Planning semaine courante** (15 min)
   - Chaque équipe présente : Quoi ? Qui ? Quand ?
   - Identification des dépendances inter-équipes
   - Sync points de la semaine

3. **Blockers actuels** (5 min)
   - Signalement de tout blocage actuel
   - Plan de résolution

4. **Questions/Clarifications** (5 min)

**Output** :
- [ ] Planning semaine documenté (dans Slack pin ou Notion)
- [ ] Blockers identifiés
- [ ] Sync points confirmés

**Facilitation** :
- Pas de facilitateur fixe
- Rotation hebdomadaire (Week 1: Core, Week 2: Infra, Week 3: UI, repeat)
- Rôle : Timekeeper + noter actions

---

#### Mercredi Midi : Checkpoint Technique
**Heure** : 12h00 (après déjeuner)
**Durée** : 45 minutes MAX
**Format** : Visio + démo
**Participants** : Toutes les équipes

**Agenda** :
1. **Demos** (20 min)
   - Chaque équipe démo ce qui fonctionne
   - Pas de slides, juste du code qui tourne
   - Focus sur intégration inter-équipes

2. **Technical discussions** (15 min)
   - API changes
   - Architecture decisions
   - Breaking changes à venir

3. **Integration issues** (10 min)
   - Problèmes d'intégration rencontrés
   - Plan de résolution

**Output** :
- [ ] Demos faites (enregistrées si possible)
- [ ] Décisions techniques documentées
- [ ] Issues d'intégration tracées

**Règles** :
- Démo = code qui fonctionne (même partiel)
- Pas de démo = expliquer pourquoi + plan
- Questions bienvenues

---

#### Vendredi Après-Midi : Revue de Fin de Semaine
**Heure** : 16h00
**Durée** : 1 heure MAX
**Format** : Visio + rétrospective
**Participants** : Toutes les équipes

**Agenda** :
1. **Revue livrables** (20 min)
   - Chaque équipe : Qu'est-ce qui est DONE ?
   - Critères de "Done" vérifiés
   - Ce qui n'est pas fini → pourquoi ?

2. **Code reviews cross-team** (15 min)
   - Reviews de PRs importantes
   - Feedback technique

3. **Rétrospective** (20 min)
   - What went well ? ✅
   - What didn't go well ? ⚠️
   - Actions pour la semaine prochaine

4. **Planning next week** (5 min)
   - Preview de la semaine suivante
   - Anticipation des blocages

**Output** :
- [ ] Livrables validés
- [ ] Rétrospective documentée
- [ ] Actions pour semaine suivante

**Format Rétrospective** :
- Chacun écrit dans un doc partagé (5 min)
- Discussion collective (15 min)
- Vote sur top 3 actions

---

### Sync Quotidien (Asynchrone)

#### Daily Standup sur Slack
**Heure** : 9h00 (chacun poste quand il arrive)
**Format** : Message Slack
**Channel** : Chaque équipe dans son channel + cross-post résumé dans `#general`

**Template** :
```markdown
👋 [Nom]

✅ **Hier** :
- Terminé: Issue 002.1 (Audio capture)
- Tests: 15 tests passent

🎯 **Aujourd'hui** :
- Implémenter: Issue 002.2 (Noise reduction)
- Objectif: Finir avant EOD

🚧 **Blockers** :
- Aucun
OU
- Bloqué par: 001.2 (Build system) - En attente Team Infra
```

**Règles** :
- ✅ Post avant 10h
- ✅ Max 3-4 lignes
- ✅ Mentionner blockers explicitement
- ✅ Si bloqué, aussi poster dans `#blockers`

---

### Sync Points Critiques du Projet

Certains moments du projet sont **ultra-critiques** et nécessitent une synchronisation renforcée.

#### 🔴 SYNC POINT 1 : End of Week 1 (Build System)
**Date** : Vendredi Week 1, 16h
**GO/NO-GO** : Peut-on continuer vers Week 2 ?

**Participants** : Toutes les équipes (obligatoire)

**Critères** :
- [ ] `cargo build` réussit sur Windows/macOS/Linux
- [ ] Config système fonctionne
- [ ] Whisper intégré (Core)
- [ ] Icons disponibles (UI)

**Process** :
1. Team Infra démo build sur 3 OS
2. Team Core démo Whisper transcription
3. Team UI montre icons
4. **VOTE GO/NO-GO**
5. Si NO-GO : Plan de déblocage pour Week 2

**Si NO-GO** :
- Weekend work possible (volontaire)
- Lundi Week 2 : Focus déblocage
- Autres équipes aident

---

#### 🔴🔴🔴 SYNC POINT 2 : Mid-Week 4 (Transcription Pipeline)
**Date** : Mercredi Week 4, 12h
**GO/NO-GO** : Transcription fonctionne ?

**Participants** : Toutes les équipes (obligatoire)

**Critères** :
- [ ] Audio en temps réel → Transcription → Texte
- [ ] Latence < 5 secondes (target < 3s)
- [ ] Qualité acceptable (WER < 15%)
- [ ] Pipeline ne crash pas

**Process** :
1. Team Core démo LIVE transcription
   - Parler dans le micro
   - Voir texte apparaître
   - Tester avec bruit, silence, différents accents
2. Toutes les équipes testent
3. **VOTE GO/NO-GO**
4. Si NO-GO : TOUTES les équipes déployées sur debugging

**Si NO-GO** :
- 🚨 **ALERTE PROJET**
- Team UI et Infra aident Team Core
- Daily meetings jusqu'à résolution
- Peut retarder projet de 1-2 semaines

**Pourquoi critique ?**
- Team UI bloquée pour text injection (Week 5)
- Sans transcription, MVP impossible
- Chemin critique du projet

---

#### 🟠 SYNC POINT 3 : End of Week 5 (MVP Complet)
**Date** : Vendredi Week 5, 16h
**Milestone** : MVP fonctionnel

**Participants** : Toutes les équipes

**Critères** :
- [ ] Hotkey déclenche enregistrement
- [ ] Audio capturé
- [ ] Transcription fonctionne
- [ ] Texte injecté dans app active
- [ ] Fonctionne sur Windows/macOS/Linux

**Process** :
1. Demo end-to-end sur 3 OS
2. Chacun teste sur son poste
3. Collecte feedback
4. Liste bugs critiques
5. Priorisation bugs

**Celebration** :
- 🎉 MVP fonctionnel = grosse victoire !
- Pause team building (optionnel)
- Partager sur #general

---

## Processus de Décision

### Comment Prendre des Décisions Sans Coordinateur ?

#### Décisions Techniques (Scope: 1 équipe)

**Process** :
1. Équipe discute en interne (channel équipe)
2. Décision prise par consensus ou majorité
3. Documentation dans code/comments/docs
4. Annonce dans #integration si impact inter-équipes

**Exemples** :
- Choix d'une library (ex: nnnoiseless vs autre)
- Architecture interne d'un module
- Code style local

---

#### Décisions Techniques (Scope: Multiple équipes)

**Process** :
1. Proposant ouvre discussion dans #integration
2. Explique problème + propose solution(s)
3. Équipes concernées donnent feedback (24h)
4. Discussion si nécessaire (Mercredi checkpoint)
5. Décision par **consentement** (pas d'objection forte)
6. Documentation dans Architecture Decision Record (ADR)

**Exemples** :
- API entre modules (ex: Core → UI)
- Format de config
- Build system choices

**Template Discussion** :
```markdown
## [Proposal] API for Audio Levels

**Context**: UI needs to display audio levels in overlay

**Options**:
1. Polling API: `get_audio_level() -> f32`
2. Callback: `on_audio_level(f32)`
3. Channel: `audio_level_channel()`

**Recommendation**: Option 2 (Callback) because...

**Impact**:
- Team Core: implement callback
- Team UI: register callback

**Feedback deadline**: Tomorrow 16h

@team-core @team-ui
```

---

#### Décisions Projet (Scope: Tout le projet)

**Process** :
1. Discussion dans #general
2. Sync point (Lundi ou Vendredi)
3. Présentation options
4. Vote (si nécessaire)
5. Décision finale documentée

**Exemples** :
- Retarder release
- Couper features du MVP
- Changer priorités

---

### Principe du Consentement vs Consensus

**Consentement** (utilisé par VoxAI) :
- "Est-ce que quelqu'un a une **objection forte** ?"
- Si non → décision passe
- Plus rapide que consensus

**Consensus** (non utilisé) :
- "Est-ce que tout le monde est **d'accord** ?"
- Peut bloquer décisions
- Trop lent

**Objection forte** = argument technique valide
**Pas objection forte** = "j'aurais fait autrement mais OK"

---

## Gestion des Blocages

### Définition d'un Blocage

**Blocage** = Vous ne pouvez pas avancer sur votre tâche actuelle

**Exemples** :
- Attendez un livrable d'une autre équipe
- Bug bloquant dans dépendance
- Décision technique nécessaire
- Manque d'information

**Pas un blocage** :
- Tâche difficile mais faisable
- Besoin de réfléchir/designer
- Tests qui échouent (vous pouvez debugger)

---

### Process de Déblocage

#### 1. Signalement (< 30 min après détection)

**Action** : Poster dans `#blockers`

**Template** :
```markdown
🚧 **BLOCKER** - [Votre équipe]

**Bloqué sur** : Issue 008.1 (Windows text injection)

**Bloqué par** : Issue 003.1 (Transcription pipeline) - Team Core
- Besoin de API `get_transcription_result()`

**Impact** :
- Cannot implement text injection without transcription result
- Blocks Week 5 milestone

**Workaround possible** :
- Can prototype with mock transcription text

**Need help from** : @team-core

**Urgency** : 🔴 High (blocks critical path)
```

---

#### 2. Triage (< 2h)

**Équipe bloquante** répond :
- Acknowledge le blocage
- Estime temps de déblocage
- Propose workaround si possible

**Exemple réponse** :
```markdown
@team-ui Acknowledged!

**Timeline** : Will have API ready by tomorrow EOD

**Temporary workaround** :
Use this mock for now:
\`\`\`rust
fn mock_transcription() -> String {
    "Hello world".to_string()
}
\`\`\`

**Actions** :
- [ ] @dev-a: Expose API today
- [ ] @dev-b: Document API
- [ ] Update team-ui tomorrow morning
```

---

#### 3. Déblocage

**Équipe bloquante** :
- Priorise déblocage
- Communique progrès dans thread
- Notifie quand débloqué

**Équipe bloquée** :
- Travaille sur workaround si possible
- Travaille sur autre tâche
- Check updates régulièrement

---

#### 4. Escalation (si pas résolu en 1 jour)

**Escalation automatique** :
- Discussuion dans prochaine sync (Mercredi ou Vendredi)
- Toutes les équipes impliquées
- Plan de déblocage collectif

**Options d'escalation** :
- Prioriser déblocage (autres tâches en pause)
- Toute l'équipe aide à débloquer
- Workaround temporaire accepté
- Re-scope feature si nécessaire

---

## Outils

### Stack d'Outils Recommandés

#### Communication
**Slack** ou **Discord**
- Channels structurés
- Threads pour discussions
- Intégrations (GitHub, CI/CD)

#### Code & Reviews
**GitHub**
- Pull Requests
- Code reviews
- Issues (synced avec backlogs/)

#### Documentation
**Markdown dans repo** (option simple) ou **Notion** (option riche)
- Architecture docs
- ADRs (Architecture Decision Records)
- Meeting notes

#### Meetings
**Google Meet** ou **Zoom**
- Sync points hebdomadaires
- Screen sharing pour demos

#### Task Tracking
**Backlogs locaux** (Markdown) + **GitHub Projects** (optionnel)
- Issues dans backlogs/todo/
- Move to backlogs/done/ when complete
- GitHub Projects pour visualisation

---

## Protocoles

### Protocol : Pull Request Review

**Qui review ?**
- Au moins 1 personne de votre équipe
- Si change API inter-équipes : 1 personne de l'équipe impactée

**Timeline** :
- Review dans 24h (max 48h)
- Feedback constructif
- Approve ou Request Changes

**Template PR** :
```markdown
## Issue
Closes #002-1

## Changes
- Implement audio capture with CPAL
- Add ring buffer for streaming
- Add device enumeration

## Tests
- [x] Unit tests added (15 tests)
- [x] Manual testing done
- [x] Docs updated

## Impact on other teams
- @team-ui: Can now use `AudioCapture::start()` API

## Screenshots/Demo
[If applicable]
```

---

### Protocol : API Changes

**Process** :
1. Proposer change dans #integration (avant implémentation)
2. Expliquer impact
3. Attendre feedback (24h)
4. Implémenter
5. Documenter
6. Annoncer dans sync point

**Breaking changes** :
- 🚨 Annoncer minimum 2 jours avant
- Coordonner migration avec équipes impactées
- Créer migration guide

---

### Protocol : Bug Reports

**Qui reporte ?**
- Toute équipe peut reporter bug dans autre équipe

**Process** :
1. Poster dans #qa
2. Inclure :
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Environment (OS, etc.)
   - Severity (Critical/High/Medium/Low)
3. Équipe responsable triage (< 24h)
4. Fix ou track dans backlog

**Template** :
```markdown
🐛 **BUG** - [Module]

**Description** : Audio capture crashes on Linux

**Steps to reproduce** :
1. Start app
2. Press hotkey
3. App crashes

**Expected** : Audio captured

**Actual** : Segfault

**Environment** :
- OS: Ubuntu 22.04
- Rust: 1.75
- CPAL: 0.15

**Severity** : 🔴 Critical

**Found by** : @team-ui
**Responsible** : @team-core
```

---

### Protocol : Deployment/Release

**Process** (Week 7) :
1. Toutes les équipes : Feature freeze (lundi)
2. Bug fixes only (lundi-mercredi)
3. Final QA (jeudi)
4. Release review (vendredi)
5. Build packages
6. Test installations
7. **GO/NO-GO vote**
8. If GO → Release!

**Release Checklist** :
```markdown
## Release Checklist v1.0.0

### Fonctionnel
- [ ] Transcription fonctionne sur 3 OS
- [ ] Text injection dans 90% apps
- [ ] Hotkeys fonctionnent
- [ ] Tray/Menu responsive

### Tests
- [ ] All tests pass
- [ ] Manual testing done
- [ ] Performance validated

### Packaging
- [ ] Windows installer works
- [ ] macOS .dmg works
- [ ] Linux packages work

### Documentation
- [ ] User guide complete
- [ ] README updated
- [ ] CHANGELOG updated

### Quality
- [ ] 0 critical bugs
- [ ] < 5 high-priority bugs
- [ ] Code coverage >= 80%

### Legal
- [ ] Licenses verified
- [ ] CHANGELOG reviewed

## Decision
- [ ] GO for release
- [ ] NO-GO (list reasons)

## Signatures
- Team Core : ✅/❌
- Team Infra : ✅/❌
- Team UI : ✅/❌
```

---

## Résumé : Clés du Succès

### ✅ DO

1. **Communiquer tôt et souvent**
   - Post standups quotidiens
   - Signaler blocages immédiatement
   - Partager progrès dans #demos

2. **Respecter les sync points**
   - Lundi/Mercredi/Vendredi obligatoires
   - Venir préparé avec démos
   - Documenter décisions

3. **Aider les autres équipes**
   - Offrir help si une équipe est bloquée
   - Reviews cross-team
   - Partager knowledge

4. **Assumer responsabilité**
   - Respecter deadlines
   - Livrer ce qui est promis
   - Escalate si problème

5. **Transparence totale**
   - Partager problèmes
   - Pas de surprises
   - Honest status updates

---

### ❌ DON'T

1. **Siloer le travail**
   - Ne pas travailler en isolation
   - Ne pas ignorer les autres équipes

2. **Surprendre les autres**
   - Pas de breaking changes sans warning
   - Pas de scope changes unilatéraux

3. **Ignorer les blocages**
   - Ne pas attendre que ça se règle
   - Ne pas "espérer" que ça marche

4. **Skip les sync points**
   - Critiques pour alignement
   - Manquer = risque désalignement

5. **Blâmer**
   - Focus sur solutions, pas culpabilité
   - Tous responsables du succès

---

## Conclusion

**VoxAI réussit avec 3 équipes auto-organisées** grâce à :

1. **Responsabilités claires** (TEAM*.md)
2. **Communication structurée** (ce guide)
3. **Sync points réguliers**
4. **Escalation rapide des blocages**
5. **Culture de collaboration**

**Vous n'avez pas besoin de coordinateurs si vous avez** :
- ✅ Transparence
- ✅ Communication
- ✅ Responsabilité
- ✅ Collaboration

**Let's ship this! 🚀**

---

## Annexe : Checklists Rapides

### Checklist : Je démarre sur le projet (Day 1)

- [ ] Lire CLAUDE.md
- [ ] Lire TEAM_ALLOCATION_PLAN.md
- [ ] Lire mon TEAM_X.md
- [ ] Lire SYNC_GUIDE.md (ce document)
- [ ] Rejoindre tous les Slack channels
- [ ] Setup environnement dev
- [ ] Dire bonjour dans #general
- [ ] Participer au prochain Lundi Planning

---

### Checklist : Je suis bloqué

- [ ] Essayer de résoudre seul (30 min max)
- [ ] Demander à mon équipe (channel équipe)
- [ ] Si toujours bloqué : Poster dans #blockers
- [ ] Travailler sur workaround ou autre tâche
- [ ] Follow-up dans 2h si pas de réponse

---

### Checklist : Je veux faire un changement d'API

- [ ] Designer la nouvelle API
- [ ] Identifier équipes impactées
- [ ] Poster proposal dans #integration
- [ ] Attendre feedback (24h)
- [ ] Implémenter
- [ ] Documenter
- [ ] Annoncer dans sync point

---

### Checklist : Je finis une feature

- [ ] Tests écrits et passent
- [ ] Code reviewé
- [ ] Documentation à jour
- [ ] Démo dans prochain sync point
- [ ] Move issue to backlogs/done/
- [ ] Annoncer dans channel équipe

---

**Bon courage ! 💪**

Questions ? → #general sur Slack

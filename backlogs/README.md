# 📋 Backlogs - Système de Gestion des Issues

**Project**: VoxAI - Application de Transcription Audio Temps Réel
**Last Updated**: 2025-11-14

---

## Vue d'ensemble

Ce dossier contient toutes les issues du projet VoxAI au format Markdown. Ce système permet de gérer les tâches localement sans dépendre de GitHub Issues, tout en gardant une trace complète des besoins, spécifications, et progrès du projet.

---

## Structure du Dossier

```
backlogs/
├── README.md                  # Ce fichier
├── templates/                 # Templates pour nouvelles issues
│   └── issue-template.md     # Template standard
├── todo/                      # Issues à faire
│   ├── epic-01/              # Infrastructure
│   ├── epic-02/              # Capture Audio
│   ├── epic-03/              # Transcription
│   ├── epic-04/              # Accélération GPU
│   ├── epic-05/              # System Tray
│   ├── epic-06/              # Hotkeys
│   ├── epic-07/              # Configuration
│   ├── epic-08/              # Text Output
│   ├── epic-09/              # Visual Feedback
│   ├── epic-10/              # Testing
│   ├── epic-11/              # Deployment
│   ├── epic-12/              # Documentation
│   └── epic-13/              # Advanced Features
├── in-progress/               # Issues en cours
└── done/                      # Issues terminées
```

---

## Workflow

### 1. Créer une nouvelle issue

Utilisez le template disponible dans `templates/issue-template.md` :

```bash
cp templates/issue-template.md todo/epic-XX/XXX-issue-name.md
```

Puis remplissez toutes les sections du template.

### 2. Démarrer une issue

Quand vous commencez à travailler sur une issue :

1. Changez le statut dans le fichier : `Status: Todo` → `Status: In Progress`
2. Déplacez le fichier : `todo/` → `in-progress/`

```bash
mv todo/epic-01/001-1-project-initialization.md in-progress/
```

### 3. Terminer une issue

Quand l'issue est complète :

1. Vérifiez que tous les critères d'acceptation sont cochés
2. Changez le statut : `Status: In Progress` → `Status: Done`
3. Déplacez le fichier : `in-progress/` → `done/`

```bash
mv in-progress/001-1-project-initialization.md done/
```

### 4. Créer des issues GitHub (optionnel)

Si vous souhaitez créer des issues GitHub depuis ces fichiers Markdown :

```bash
# Utiliser gh CLI pour créer l'issue
gh issue create --title "[ISSUE-XXX] Title" --body-file todo/epic-01/001-1-issue.md
```

---

## Format des Issues

Chaque issue suit un format standardisé :

### Métadonnées
- **Created**: Date de création
- **Priority**: Critique | Haute | Moyenne | Basse
- **Type**: Feature | Bug | Refactor | Documentation | Chore
- **Status**: Todo | In Progress | Done
- **Estimated Effort**: XS | S | M | L | XL (ou heures)
- **EPIC**: Numéro et nom de l'EPIC parent

### Sections Principales
1. **Description**: Résumé concis de ce qui doit être fait
2. **Context**: Pourquoi cette issue existe
3. **Acceptance Criteria**: Critères mesurables de complétion
4. **Technical Details**: Détails techniques, composants affectés, dépendances
5. **Tasks Breakdown**: Liste des sous-tâches
6. **Test Plan**: Comment tester (unit, integration, manual)
7. **Documentation Updates**: Quels docs mettre à jour
8. **Related Issues**: Dépendances et relations
9. **Notes**: Informations supplémentaires, exemples de code
10. **Definition of Done**: Checklist finale

---

## EPICs du Projet VoxAI

### EPIC 1: Infrastructure et Configuration du Projet ⚙️
Configuration initiale du projet Rust, build multi-plateforme, intégration Whisper.cpp

**Issues**: 001.1, 001.2, 001.3

### EPIC 2: Capture Audio 🎤
Capture audio avec CPAL, réduction de bruit, détection d'activité vocale (VAD)

**Issues**: 002.1, 002.2, 002.3

### EPIC 3: Transcription avec Whisper 🗣️
Pipeline de transcription temps réel, gestion des modèles, support multi-langues

**Issues**: 003.1, 003.2, 003.3

### EPIC 4: Accélération Matérielle 🚀
Support GPU (CUDA, Metal, OpenCL), optimisations CPU

**Issues**: 004.1, 004.2, 004.3, 004.4

### EPIC 5: Interface System Tray 📱
System tray Windows, macOS Menu Bar, Linux tray, icônes et design

**Issues**: 005.1, 005.2, 005.3, 005.4

### EPIC 6: Raccourcis Clavier ⌨️
Hotkeys globaux (Windows, macOS, Linux), configuration des raccourcis

**Issues**: 006.1, 006.2, 006.3, 006.4

### EPIC 7: Gestion de la Configuration ⚙️
Configuration persistante, interface de paramètres, sélection périphérique audio

**Issues**: 007.1, 007.2, 007.3

### EPIC 8: Sortie et Injection du Texte 📝
Injection de texte (Windows, macOS, Linux), post-traitement

**Issues**: 008.1, 008.2, 008.3, 008.4

### EPIC 9: Indicateurs Visuels et Feedback 👁️
Overlay de statut, notifications système, indicateurs de performance

**Issues**: 009.1, 009.2, 009.3

### EPIC 10: Tests et Qualité ✅
Tests unitaires, tests d'intégration, benchmarks de performance

**Issues**: 010.1, 010.2, 010.3

### EPIC 11: Déploiement et Distribution 📦
Packaging (Windows, macOS, Linux), système de mise à jour automatique

**Issues**: 011.1, 011.2, 011.3, 011.4

### EPIC 12: Documentation 📚
Documentation utilisateur, documentation développeur, API docs

**Issues**: 012.1, 012.2, 012.3

### EPIC 13: Fonctionnalités Avancées (Phase 2) 🌟
Commandes vocales, macros, historique, profils

**Issues**: 013.1, 013.2, 013.3, 013.4

---

## Conventions de Nommage

### Fichiers d'issues

Format: `{EPIC_NUMBER}-{ISSUE_NUMBER}-{slug}.md`

Exemples :
- `001-1-project-initialization.md`
- `002-1-audio-capture-cpal.md`
- `003-1-transcription-pipeline.md`

### Dossiers EPIC

Format: `epic-{NUMBER}/`

Exemples :
- `epic-01/` (Infrastructure)
- `epic-02/` (Capture Audio)
- `epic-03/` (Transcription)

---

## Priorités

### Critique 🔴
Issues bloquantes pour le MVP, doivent être faites en premier.

**Exemples**:
- Initialisation du projet
- Capture audio
- Transcription Whisper
- System tray
- Hotkeys

### Haute 🟠
Issues importantes pour une bonne UX, à faire pour le MVP.

**Exemples**:
- Réduction de bruit
- Gestion des modèles
- Configuration persistante
- Injection de texte

### Moyenne 🟡
Fonctionnalités nice-to-have, peuvent attendre la phase 2.

**Exemples**:
- Overlay de statut
- Multi-langues
- Commandes vocales

### Basse 🟢
Optimisations et améliorations mineures.

**Exemples**:
- Indicateurs de performance
- Profils et contextes
- Fonctionnalités avancées

---

## Métriques et Tracking

### Suivi de Progression

Vous pouvez suivre la progression avec un simple script :

```bash
# Compter les issues par statut
echo "Todo: $(find todo -name '*.md' -not -name 'README.md' | wc -l)"
echo "In Progress: $(find in-progress -name '*.md' | wc -l)"
echo "Done: $(find done -name '*.md' | wc -l)"
```

### Estimation Totale

Effort estimé par taille :
- **XS**: 1-2 heures
- **S**: 2-4 heures
- **M**: 4-8 heures
- **L**: 8-16 heures
- **XL**: 16+ heures

---

## Commandes Utiles

### Rechercher une issue par mot-clé

```bash
grep -r "Whisper" todo/
```

### Lister toutes les issues d'un EPIC

```bash
ls todo/epic-01/
```

### Trouver les issues critiques

```bash
grep -l "Priority: Critique" todo/**/*.md
```

### Lister les dépendances d'une issue

```bash
grep "Blocked by:" todo/epic-03/003-1-transcription-pipeline.md
```

---

## Intégration avec Claude Code

Claude Code peut :
1. **Lire les issues** pour comprendre les besoins
2. **Implémenter les tâches** selon les spécifications
3. **Mettre à jour les issues** (cocher les critères d'acceptation)
4. **Déplacer les issues** quand terminées

### Exemple de workflow avec Claude :

```
User: "Implémente l'issue 001-1"
Claude: *Lit l'issue dans todo/epic-01/001-1-project-initialization.md*
Claude: *Crée la structure du projet selon les specs*
Claude: *Coche les critères d'acceptation*
Claude: *Déplace l'issue vers done/*
```

---

## Synchronisation avec GitHub (Optionnel)

Si vous souhaitez synchroniser avec GitHub Issues :

### Créer toutes les issues GitHub

```bash
for file in todo/**/*.md; do
  title=$(grep "^# " "$file" | head -1 | sed 's/# //')
  gh issue create --title "$title" --body-file "$file"
done
```

### Mettre à jour depuis GitHub

```bash
gh issue list --json number,title,state
```

---

## Best Practices

### ✅ À Faire

- Toujours utiliser le template pour nouvelles issues
- Remplir toutes les sections obligatoires
- Déplacer les fichiers selon l'avancement (todo → in-progress → done)
- Cocher les tâches au fur et à mesure
- Ajouter des notes et learnings dans les issues

### ❌ À Éviter

- Créer des issues sans description claire
- Laisser des issues "In Progress" trop longtemps
- Oublier de documenter les dépendances
- Négliger les critères d'acceptation
- Supprimer les issues terminées (les garder dans done/)

---

## Référence Rapide

| Action | Commande |
|--------|----------|
| Créer issue | `cp templates/issue-template.md todo/epic-XX/XXX-name.md` |
| Démarrer | `mv todo/XXX.md in-progress/` |
| Terminer | `mv in-progress/XXX.md done/` |
| Lister todo | `ls todo/epic-XX/` |
| Chercher | `grep -r "keyword" todo/` |
| Compter | `find todo -name '*.md' \| wc -l` |

---

## Questions Fréquentes

### Puis-je modifier le format des issues ?

Oui, mais assurez-vous de garder les sections essentielles :
- Description
- Acceptance Criteria
- Tasks Breakdown
- Definition of Done

### Que faire si une issue bloque ?

1. Documenter le blocage dans la section Notes
2. Créer une nouvelle issue pour résoudre le blocage
3. Ajouter la dépendance dans "Blocked by"
4. Passer à une autre issue en attendant

### Comment gérer les bugs découverts ?

Créer une nouvelle issue avec `Type: Bug` :
```bash
cp templates/issue-template.md todo/bug-XXX-description.md
```

### Puis-je regrouper plusieurs issues ?

Oui, utilisez des EPICs pour regrouper logiquement. Si besoin, créez des sous-EPICs.

---

## Contribution

Pour ajouter une nouvelle issue au backlog :

1. Copier le template
2. Remplir toutes les sections
3. Placer dans le bon EPIC folder
4. Mettre à jour ce README si nécessaire

---

## Ressources

- **Template**: `templates/issue-template.md`
- **Documentation Projet**: `../CLAUDE.md`
- **README Principal**: `../README.md`

---

**Happy coding! 🚀**

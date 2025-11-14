# TEAM 3 : UI / PLATFORM
## VoxAI - Multi-Platform Interfaces & Integration

**Équipe**: UI/Platform
**Focus**: System tray, hotkeys, text injection, visual feedback
**Durée**: 7 semaines
**Effectif**: 2 développeurs

---

## 👥 Composition de l'Équipe

### Profils Recommandés

**Développeur Platform Windows/Linux (Dev A)** :
- **Compétences** : Rust + Windows APIs, Linux APIs
- **Expérience** : Desktop apps, system programming
- **Responsabilités** :
  - System tray Windows + Linux
  - Global hotkeys Windows + Linux
  - Text injection Windows + Linux
  - Packaging Windows + Linux

**Développeur Platform macOS (Dev B)** :
- **Compétences** : Rust + macOS APIs, Objective-C/Swift interop
- **Expérience** : macOS development, Cocoa APIs
- **Responsabilités** :
  - Menu bar macOS
  - Global hotkeys macOS
  - Text injection macOS
  - Packaging macOS

### Flexibilité
- Si un dev connaît les 3 plateformes, peut alterner
- Peut se diviser par feature (tray, hotkeys, injection) plutôt que par OS
- Collaboration étroite nécessaire pour cohérence cross-platform

---

## 🎯 Responsabilités Globales

### Modules Sous Responsabilité

1. **src/ui/tray_*.rs** - System tray
   - `tray_windows.rs` - Windows system tray
   - `tray_macos.rs` - macOS menu bar
   - `tray_linux.rs` - Linux system tray
   - `menu.rs` - Menu management

2. **src/hotkeys/*.rs** - Global hotkeys
   - `windows.rs` - Windows hotkey (RegisterHotKey)
   - `macos.rs` - macOS hotkey (Carbon/Cocoa)
   - `linux.rs` - Linux hotkey (X11/Wayland)

3. **src/output/*.rs** - Text output
   - `text_injector_windows.rs` - SendInput API
   - `text_injector_macos.rs` - CGEvent API
   - `text_injector_linux.rs` - XTest/uinput
   - `clipboard.rs` - Clipboard management (arboard)
   - `formatter.rs` - Text formatting

4. **src/ui/feedback/*.rs** - Visual feedback
   - Status overlay
   - System notifications
   - Performance indicators

5. **scripts/** - Packaging
   - `build-windows.sh` - Windows installer
   - `build-macos.sh` - macOS .app/.dmg
   - `build-linux.sh` - .deb/.rpm/AppImage

6. **assets/** - Assets
   - Icons (idle, recording, processing)
   - Sounds (optional)
   - Images

### Livrables Principaux

- ✅ System tray fonctionnel sur Windows/macOS/Linux
- ✅ Global hotkeys sur 3 plateformes
- ✅ Text injection sur 3 plateformes
- ✅ Visual feedback (overlay, notifications)
- ✅ Packaging pour 3 plateformes
- ✅ Documentation utilisateur
- ✅ Icons et assets

---

## 📅 Planning Détaillé - 7 Semaines

### SEMAINE 1 : Icons Design & Prototyping
**Dates** : Jour 1-5
**Charge** : 50% (setup + design)

#### Objectifs
- Créer icônes pour tray
- Setup environnement cross-platform
- Prototyper tray basic

#### Tâches Détaillées

**Dev A (Windows/Linux)** :
- [ ] Setup environnement Windows
  - Rust + Windows SDK
  - Test compilation Windows APIs
- [ ] Setup environnement Linux (VM ou native)
  - Rust + X11/GTK deps
- [ ] Research Windows tray APIs
- [ ] Research Linux tray APIs (systray/tray-icon crate)

**Dev B (macOS)** :
- [ ] Setup environnement macOS
  - Rust + Xcode
  - Test compilation macOS APIs
- [ ] Research macOS menu bar APIs
- [ ] Prototyper menu bar basic

**Dev A & B** :
- [ ] **Issue 005.4** : Icons Design (4h)
  - Créer 3 icônes :
    - `idle.png/ico` - App au repos
    - `recording.png/ico` - En cours d'enregistrement
    - `processing.png/ico` - Transcription en cours
  - Formats :
    - Windows : .ico (16x16, 32x32, 64x64)
    - macOS : .icns (16x16 → 512x512)
    - Linux : .png (16x16, 32x32, 64x64)
  - Style minimaliste, clair
  - Sauvegarder dans `assets/icons/`
  - Documentation

#### Sync Points
- **Lundi** : Kickoff avec toutes les équipes
- **Mercredi** : Review prototypes
- **Vendredi** : Revue icons + planning Week 2

#### Critères de Succès
- ✅ Icônes créées et exportées
- ✅ Environnements setup
- ✅ Prototypes tray fonctionnent localement

#### Dépendances
- **Bloqué par** : RIEN (icons) / 001.2 (Build pour compilation)
- **Débloque** : 005.x (Tray implementations)

---

### SEMAINE 2-3 : Global Hotkeys (Parallèle par OS)
**Dates** : Jour 6-15
**Charge** : 100%

#### Objectifs
- Implémenter hotkeys sur les 3 plateformes
- Hotkeys fonctionnent globalement (même app pas en focus)

#### Répartition par Plateforme

**Dev A** :
- [ ] **Issue 006.1** : Windows Hotkeys (6h) 🔴 **CRITIQUE**
  - Implémenter `src/hotkeys/windows.rs`
  - Utiliser RegisterHotKey Win32 API
  - Hotkey par défaut : Ctrl+Shift+Space
  - Thread pour écouter messages hotkey
  - Intégration avec event loop
  - Tests :
    - Registration hotkey
    - Trigger hotkey
    - Unregister hotkey
  - Documentation

- [ ] **Issue 006.3** : Linux Hotkeys (8h) 🔴 **CRITIQUE**
  - Implémenter `src/hotkeys/linux.rs`
  - Support X11 (XGrabKey)
  - Support Wayland (si possible, sinon X11 fallback)
  - Hotkey par défaut : Ctrl+Shift+Space
  - Tests :
    - Registration
    - Trigger
    - Unregister
  - Documentation

**Dev B** :
- [ ] **Issue 006.2** : macOS Hotkeys (6h) 🔴 **CRITIQUE**
  - Implémenter `src/hotkeys/macos.rs`
  - Utiliser Carbon API ou Cocoa (NSEvent)
  - Hotkey par défaut : Cmd+Shift+Space
  - Tests :
    - Registration
    - Trigger
    - Unregister
  - Accessibility permissions handling
  - Documentation

- [ ] Support Dev A sur Linux si nécessaire

#### Sync Points
- **Semaine 2 Lundi** : Planning hotkeys
- **Semaine 2 Mercredi** : CHECKPOINT - Windows hotkeys démo
- **Semaine 2 Vendredi** : Revue + macOS démo
- **Semaine 3 Mercredi** : CHECKPOINT - Linux hotkeys démo
- **Semaine 3 Vendredi** : Revue finale hotkeys

#### Critères de Succès
- ✅ Hotkey enregistré sur Windows
- ✅ Hotkey enregistré sur macOS
- ✅ Hotkey enregistré sur Linux
- ✅ Hotkey déclenche action (log pour l'instant)
- ✅ Fonctionne même app en background

#### Dépendances
- **Bloqué par** : 001.2 (Build system) ✓
- **Débloque** : User interaction avec app

#### Note
À ce stade, hotkey peut juste logger ou toggle un flag. L'intégration avec audio capture sera faite par Team Core.

---

### SEMAINE 2-3 : System Tray (Parallèle par OS)
**Dates** : Jour 6-15
**Charge** : 100% (en parallèle avec hotkeys)

#### Objectifs
- Implémenter system tray sur 3 plateformes
- Menu avec options de base

#### Répartition par Plateforme

**Dev A** :
- [ ] **Issue 005.1** : Windows System Tray (8h) 🔴 **CRITIQUE**
  - Implémenter `src/ui/tray_windows.rs`
  - Utiliser tray-icon crate ou Win32 Shell_NotifyIcon
  - Icon dans system tray
  - Menu :
    - Start Recording (toggle)
    - Settings
    - Quit
  - Icon change selon état (idle/recording/processing)
  - Click handlers
  - Tests (manuels)
  - Documentation

- [ ] **Issue 005.3** : Linux System Tray (8h) 🔴 **CRITIQUE**
  - Implémenter `src/ui/tray_linux.rs`
  - Support GTK StatusIcon ou AppIndicator
  - Support KDE (si possible)
  - Menu similaire à Windows
  - Icon change selon état
  - Tests (plusieurs DEs: GNOME, KDE, XFCE)
  - Documentation

**Dev B** :
- [ ] **Issue 005.2** : macOS Menu Bar (8h) 🔴 **CRITIQUE**
  - Implémenter `src/ui/tray_macos.rs`
  - Utiliser NSStatusBar (Cocoa)
  - Icon dans menu bar
  - Menu similaire
  - Icon change selon état
  - Tests
  - Documentation

#### Sync Points
- **Semaine 2 Mercredi** : Windows tray démo
- **Semaine 2 Vendredi** : macOS tray démo
- **Semaine 3 Mercredi** : Linux tray démo
- **Semaine 3 Vendredi** : Revue complète

#### Critères de Succès
- ✅ Tray visible sur Windows
- ✅ Menu bar visible sur macOS
- ✅ Tray visible sur Linux
- ✅ Menu s'affiche et fonctionne
- ✅ Icons changent selon état

#### Dépendances
- **Bloqué par** : 001.2 (Build) ✓, 005.4 (Icons) ✓
- **Débloque** : UI complète pour utilisateur

---

### SEMAINE 4 : Prototyping Text Injection
**Dates** : Jour 16-20
**Charge** : 80% (prototyping + préparation)

#### Objectifs
- Préparer text injection (sans transcription encore)
- Prototyper avec texte statique
- Comprendre APIs plateform

#### Tâches Détaillées

**Dev A** :
- [ ] Research Windows text injection
  - SendInput API
  - Clipboard + Ctrl+V
  - Focus window handling
- [ ] Prototype Windows injection (texte statique)
- [ ] Research Linux text injection
  - XTest
  - uinput
  - Clipboard (xclip/xsel)
- [ ] Prototype Linux injection

**Dev B** :
- [ ] Research macOS text injection
  - CGEvent API
  - Accessibility API
  - Clipboard + Cmd+V
- [ ] Prototype macOS injection (texte statique)
- [ ] Handle permissions (Accessibility, Input Monitoring)

#### Sync Points
- **Lundi** : Planning + focus sur prototyping
- **Mercredi** : 🔴 **CHECKPOINT CRITIQUE TEAM CORE** - Transcription démo
  - Vous participez pour comprendre API transcription
  - Préparer intégration Week 5
- **Vendredi** : Revue prototypes injection

#### Critères de Succès
- ✅ Prototype injection fonctionne (texte statique)
- ✅ Comprendre APIs plateform
- ✅ Ready pour intégration Week 5

#### Dépendances
- **Bloqué par** : RIEN (prototyping)
- **Attente de** : 003.1 (Transcription) pour intégration Week 5

---

### SEMAINE 5 : Text Injection Integration 🔴 CRITIQUE
**Dates** : Jour 21-25
**Charge** : 120% (feature critique)

#### Objectifs
- Intégrer transcription → text injection
- Fonctionnel end-to-end sur les 3 OS

#### Tâches Détaillées

**Dev A** :
- [ ] **Issue 008.1** : Windows Text Injection (6h) 🔴 **CRITIQUE**
  - Implémenter `src/output/text_injector_windows.rs`
  - SendInput API pour typing
  - Fallback : Clipboard + Ctrl+V
  - Get active window (GetForegroundWindow)
  - Focus handling
  - Tests :
    - Injection dans Notepad
    - Injection dans Browser
    - Injection dans différentes apps
  - Documentation

- [ ] **Issue 008.3** : Linux Text Injection (8h) 🔴 **CRITIQUE**
  - Implémenter `src/output/text_injector_linux.rs`
  - XTest pour X11
  - uinput pour Wayland (si possible)
  - Fallback : Clipboard + Ctrl+V
  - Active window focus (X11: XGetInputFocus)
  - Tests sur différents DEs
  - Documentation

**Dev B** :
- [ ] **Issue 008.2** : macOS Text Injection (6h) 🔴 **CRITIQUE**
  - Implémenter `src/output/text_injector_macos.rs`
  - CGEvent API pour typing
  - Fallback : Clipboard + Cmd+V
  - Active app detection (NSWorkspace)
  - Accessibility permissions check
  - Tests :
    - Injection dans TextEdit
    - Injection dans Browser
    - Injection dans différentes apps
  - Documentation

**Dev A & B** :
- [ ] Integration end-to-end
  - Hotkey → Audio capture → Transcription → Text injection
  - Tests sur 3 OS
  - Debug issues

#### Sync Points
- **Lundi** : Planning + focus injection
- **Mercredi** : CHECKPOINT - Windows + macOS injection démo
- **Vendredi** : 🎯 **MILESTONE** - MVP end-to-end démo
  - Speak → Hotkey → Transcription → Text appears

#### Critères de Succès (GO/NO-GO MVP)
- ✅ Text injection fonctionne sur Windows
- ✅ Text injection fonctionne sur macOS
- ✅ Text injection fonctionne sur Linux
- ✅ MVP end-to-end démo-able
- ✅ Fonctionne dans 90%+ des apps

#### Dépendances
- **Bloqué par** : 003.1 (Transcription pipeline) 🔴🔴🔴
  - Si transcription pas prête, vous êtes bloqués
  - Prototypes Week 4 permettent de gagner du temps
- **Débloque** : MVP complet

#### Livrable Clé
**🎯🎯 MILESTONE MVP** : Application fonctionne end-to-end

---

### SEMAINE 6 : Visual Feedback & Polish
**Dates** : Jour 26-30
**Charge** : 100%

#### Objectifs
- Implémenter visual feedback (overlay, notifications)
- Text post-processing
- Performance indicators

#### Tâches Détaillées

**Dev A** :
- [ ] **Issue 009.1** : Status Overlay (8h)
  - Overlay transparent montrant :
    - Status (Listening/Processing/Done)
    - Audio levels (depuis Core API)
    - Transcription en cours (preview)
  - Multi-platform (ou platform-specific)
  - Positionnable
  - Auto-hide
  - Documentation

- [ ] **Issue 009.3** : Performance Indicators (6h)
  - Afficher dans tray menu :
    - CPU usage
    - RAM usage
    - Transcription speed
    - Model used
  - Real-time updates
  - Documentation

**Dev B** :
- [ ] **Issue 009.2** : System Notifications (4h)
  - Notifications natives OS :
    - Transcription started
    - Transcription complete
    - Errors
  - Multi-platform (notify-rust crate)
  - Configurable (on/off)
  - Documentation

- [ ] **Issue 008.4** : Text Post-Processing (6h)
  - Implémenter `src/output/formatter.rs`
  - Features :
    - Capitalize first letter
    - Add punctuation (if missing)
    - Trim whitespace
    - Custom transformations (via config)
  - Configurable via Settings UI (Team Infra)
  - Tests
  - Documentation

#### Sync Points
- **Lundi** : Planning Week 6
- **Mercredi** : CHECKPOINT - Démo feedback features
- **Vendredi** : Revue finale + QA

#### Critères de Succès
- ✅ Overlay affiche status
- ✅ Notifications fonctionnent
- ✅ Text post-processing améliore output
- ✅ Performance indicators visibles

#### Dépendances
- **Bloqué par** : 002.1 (Audio) pour audio levels, 007.2 (Settings) pour config
- **Débloque** : UX complète

---

### SEMAINE 7 : Packaging & User Documentation
**Dates** : Jour 31-35
**Charge** : 120% (rush final)

#### Objectifs
- Créer packages installables pour 3 OS
- Documentation utilisateur
- Final QA

#### Tâches Détaillées

**Dev A** :
- [ ] **Issue 011.1** : Windows Packaging (8h)
  - Créer `scripts/build-windows.sh`
  - Inno Setup ou WiX pour installer
  - Inclure :
    - Executable
    - Assets (icons)
    - Models Whisper (tiny par défaut)
    - README
  - Installer dans Program Files
  - Créer shortcuts
  - Uninstaller
  - Tests installation
  - Documentation

- [ ] **Issue 011.3** : Linux Packaging (10h)
  - Créer `scripts/build-linux.sh`
  - Formats :
    - .deb (Debian/Ubuntu)
    - .rpm (Fedora/RedHat)
    - AppImage (universal)
  - Inclure dépendances
  - Desktop entry (.desktop file)
  - Icon installation
  - Tests sur différentes distros
  - Documentation

**Dev B** :
- [ ] **Issue 011.2** : macOS Packaging (8h)
  - Créer `scripts/build-macos.sh`
  - Créer .app bundle
  - Info.plist configuration
  - Code signing (si possible)
  - Créer .dmg installer
  - Notarization (si compte dev Apple)
  - Tests installation
  - Documentation

**Dev A & B** :
- [ ] **Issue 012.1** : User Documentation (8h)
  - Créer `docs/USER_GUIDE.md`
  - Sections :
    - Installation (par OS)
    - Quick Start
    - Configuration (hotkeys, models, etc.)
    - Troubleshooting
    - FAQ
    - Support
  - Screenshots
  - Video tutorial (optionnel)

- [ ] Final QA
  - Tests sur fresh installations
  - Tests sur différentes versions OS
  - Tests avec différents apps
  - Bug fixes

#### Sync Points
- **Lundi** : Planning dernière semaine
- **Mercredi** : Revue packages
- **Vendredi** : 🚀 **RELEASE READINESS REVIEW**
  - Démo complète
  - Packages testés
  - Documentation review
  - Go/No-Go pour release

#### Critères de Succès
- ✅ Packages installables sur 3 OS
- ✅ Installation < 5 minutes
- ✅ Documentation utilisateur complète
- ✅ 0 bugs critiques
- ✅ Ready for public release

#### Livrable Clé
**🚀 RELEASE MVP**

---

## 🔗 Coordination avec Autres Équipes

### Avec TEAM 1 (Core)

#### Vous êtes Bloqué par Core

**Week 4-5** :
- **003.1 (Transcription)** 🔴🔴🔴 **BLOQUANT CRITIQUE**
  - Vous ne pouvez pas implémenter text injection sans transcription
  - **Checkpoint Mercredi Week 4 est VITAL**
  - Si transcription échoue, vous êtes bloqués

**Actions** :
- Participer au checkpoint Week 4
- Aider debugging si nécessaire
- Prototyper en avance (Week 4) pour gagner du temps

#### Vous Collaborez avec Core

**Week 6** :
- **009.1 (Status overlay)** - Vous affichez audio levels
  - Core expose API pour audio levels
  - Coordination sur format API

**API attendues de Core** :

```rust
// Pour text injection
pub trait TranscriptionResult {
    fn get_text(&self) -> &str;
}

// Pour overlay
pub trait AudioStatus {
    fn get_level(&self) -> f32; // 0.0 to 1.0
    fn is_recording(&self) -> bool;
}
```

---

### Avec TEAM 2 (Infra)

#### Vous êtes Bloqué par Infra

**Week 1** :
- **001.2 (Build system)** 🔴🔴🔴 - Ne pouvez pas compiler sans ça
  - Daily check-in avec Infra

**Week 2** :
- **007.2 (Settings UI)** 🟠 - Vous pouvez intégrer settings dans tray menu

**Week 5-6** :
- **006.4 (Hotkey config)** - Infra crée UI, vous utilisez config
- **007.3 (Audio device)** - Infra crée UI, vous affichez info
- **008.4 (Text processing)** - Infra crée config, vous appliquez

#### Vous Collaborez avec Infra

**Week 7** :
- **011.4 (Auto-update)** - Infra crée logic, vous affichez notifications update
- **012.1 (User docs)** - Vous créez docs utilisateur, Infra review

---

### Meetings & Communication

**Daily Standup** (async Slack) :
```
👋 [Dev A]
✅ Hier: Windows hotkeys implemented
🎯 Aujourd'hui: Linux hotkeys + tests
🚧 Blockers: None
```

**Weekly Sync** :
- Lundi : Planning
- Mercredi : Demo progrès
- Vendredi : Revue code

**Channels** :
- `#ui-platform` : Votre channel principal
- `#blockers` : Urgences
- `#integration` : Tests inter-équipes

---

## ✅ Critères de Succès par Semaine

### Week 1 : Icons & Setup
- [x] Icons créées (3 états)
- [x] Environnements setup (Win/macOS/Linux)
- [x] Prototypes tray fonctionnent

### Week 2-3 : Hotkeys & Tray
- [x] Hotkeys sur Windows
- [x] Hotkeys sur macOS
- [x] Hotkeys sur Linux
- [x] Tray sur Windows
- [x] Menu bar sur macOS
- [x] Tray sur Linux

### Week 4 : Prototyping
- [x] Prototypes text injection fonctionnent
- [x] APIs plateform comprises
- [x] Ready pour intégration Week 5

### Week 5 : Text Injection (MVP)
- [x] Text injection Windows
- [x] Text injection macOS
- [x] Text injection Linux
- [x] MVP end-to-end fonctionne

### Week 6 : Feedback & Polish
- [x] Status overlay
- [x] System notifications
- [x] Text post-processing
- [x] Performance indicators

### Week 7 : Packaging & Docs
- [x] Packages Windows/macOS/Linux
- [x] User documentation
- [x] Ready for release

---

## 📊 Métriques et KPIs

### Functionality

**Cross-Platform Compatibility** :
- Windows support : ✅
- macOS support : ✅
- Linux support : ✅

**App Compatibility (Text Injection)** :
- Target : >= 90% of common apps
- Test avec :
  - Text editors (Notepad, TextEdit, gedit, VS Code)
  - Browsers (Chrome, Firefox, Safari)
  - Office apps (Word, Google Docs)
  - Chat apps (Slack, Discord)

### Performance

**Injection Latency** :
- Target : < 100ms
- Acceptable : < 200ms

**Hotkey Response** :
- Target : < 50ms
- Acceptable : < 100ms

### Quality

**Visual Polish** :
- Icons clear and consistent
- Tray menu responsive
- Overlay non-intrusive

---

## 🛠️ Outils et Processus

### Development Tools

**Cross-Platform Testing** :
- Windows 11 (native ou VM)
- macOS (latest) (native ou VM)
- Linux : Ubuntu, Fedora, Arch (VMs)

**Platform-Specific Tools** :

**Windows** :
```bash
# Compile Windows
cargo build --target x86_64-pc-windows-msvc

# Run tests
cargo test

# Create installer
./scripts/build-windows.sh
```

**macOS** :
```bash
# Compile macOS
cargo build --target x86_64-apple-darwin

# Create .app
./scripts/build-macos.sh

# Code signing (if dev account)
codesign --sign "Developer ID" VoxAI.app
```

**Linux** :
```bash
# Compile Linux
cargo build --target x86_64-unknown-linux-gnu

# Create packages
./scripts/build-linux.sh
```

### Testing

**Manual Testing Checklist** :
- [ ] Tray icon visible
- [ ] Menu opens on click
- [ ] Hotkey triggers recording
- [ ] Audio captured
- [ ] Transcription appears
- [ ] Text injected in active app
- [ ] Overlay shows status
- [ ] Notifications appear

**Test Apps** :
- Notepad (Windows)
- TextEdit (macOS)
- gedit (Linux)
- VS Code (all)
- Chrome (all)
- Slack (all)

---

## 🚨 Gestion des Risques

### Risques Identifiés

#### Risque 1 : Permissions issues (macOS)
**Probabilité** : Haute
**Impact** : 🔴 Critique

**Mitigation** :
- Documenter permissions nécessaires
- UI pour guider utilisateur
- Fallback : Clipboard si permissions refusées

#### Risque 2 : Text injection incompatible avec certaines apps
**Probabilité** : Moyenne
**Impact** : 🟠 Moyenne

**Mitigation** :
- Fallback : Clipboard + Paste
- Documenter apps incompatibles
- Community feedback pour fixes

#### Risque 3 : Linux DE fragmentation
**Probabilité** : Haute
**Impact** : 🟡 Faible-Moyenne

**Mitigation** :
- Tester sur GNOME, KDE, XFCE
- Utiliser crates cross-DE (tray-icon)
- Fallback gracefully

#### Risque 4 : Week 5 bloqué par transcription
**Probabilité** : Faible
**Impact** : 🔴🔴 Haute

**Mitigation** :
- Prototyper Week 4
- Utiliser mock transcription si nécessaire
- Participer au debugging transcription

---

## 📚 Ressources

### Platform APIs

**Windows** :
- [Win32 API](https://docs.microsoft.com/en-us/windows/win32/)
- [SendInput](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput)
- [RegisterHotKey](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey)

**macOS** :
- [Cocoa](https://developer.apple.com/documentation/appkit)
- [CGEvent](https://developer.apple.com/documentation/coregraphics/cgevent)
- [NSStatusBar](https://developer.apple.com/documentation/appkit/nsstatusbar)

**Linux** :
- [X11](https://www.x.org/releases/current/doc/)
- [XTest](https://www.x.org/releases/X11R7.7/doc/libXtst/xtestlib.html)
- [GTK](https://www.gtk.org/)

### Rust Crates

**UI** :
- [tray-icon](https://docs.rs/tray-icon/) - Cross-platform tray
- [egui](https://docs.rs/egui/) - GUI (used by Infra)

**Input** :
- [global-hotkey](https://docs.rs/global-hotkey/) - Global hotkeys
- [enigo](https://docs.rs/enigo/) - Keyboard/mouse simulation

**Clipboard** :
- [arboard](https://docs.rs/arboard/) - Clipboard access

**Notifications** :
- [notify-rust](https://docs.rs/notify-rust/) - System notifications

---

## 🎯 Checklist Finale (End of Week 7)

### Functionality
- [ ] System tray sur Windows/macOS/Linux
- [ ] Global hotkeys sur 3 OS
- [ ] Text injection sur 3 OS
- [ ] MVP end-to-end fonctionne

### Visual Feedback
- [ ] Status overlay
- [ ] System notifications
- [ ] Performance indicators
- [ ] Text post-processing

### Packaging
- [ ] Windows installer (.exe)
- [ ] macOS package (.dmg)
- [ ] Linux packages (.deb/.rpm/AppImage)
- [ ] Installation testée

### Documentation
- [ ] User guide complet
- [ ] Installation instructions
- [ ] Troubleshooting guide
- [ ] Screenshots/videos

### Quality
- [ ] 0 bugs critiques
- [ ] Works in 90%+ apps
- [ ] Responsive UI
- [ ] Ready for release

---

## 🚀 Let's Build!

Vous êtes l'équipe **UI/Platform** - le visage de VoxAI pour les utilisateurs. Votre travail détermine l'expérience utilisateur et la qualité perçue du produit.

**Priorités absolues** :
1. 🟠 Week 2-3 : Hotkeys + Tray sur 3 OS
2. 🔴🔴 Week 5 : Text injection MUST work (dépend de Core Week 4)
3. 🟠 Week 7 : Packaging propre et professionnel

**Communication is key** :
- Daily standups
- Signal blockers immediately on #blockers
- Coordonner avec Core sur API transcription
- Test, test, test sur différentes apps

**Vous créez la magie ! ✨**

Pour questions : #ui-platform sur Slack

Références :
- CLAUDE.md
- TEAM_ALLOCATION_PLAN.md
- TEAM1_CORE.md
- TEAM2_INFRA.md

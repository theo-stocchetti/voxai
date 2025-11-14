# [ISSUE-005.1] Implémentation du System Tray (Windows)

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: 8h
**EPIC**: 5 - Interface System Tray

---

## Description

Implémenter l'icône et le menu dans la system tray Windows. L'application doit fonctionner en arrière-plan avec indicateurs visuels de statut (idle, recording, processing).

---

## Context

L'application VoxAI ne doit pas avoir de fenêtre principale permanente. Elle fonctionne via une icône dans la system tray avec un menu contextuel pour les actions et paramètres.

---

## Acceptance Criteria

- [ ] Icône visible dans system tray Windows
- [ ] Menu contextuel fonctionnel (clic droit)
- [ ] Double-clic ouvre les paramètres
- [ ] Indicateur visuel pour 3 états (idle/recording/processing)
- [ ] Option "Quitter" ferme proprement l'app
- [ ] Survit au redémarrage de explorer.exe

---

## Technical Details

### Affected Components
- src/ui/tray_windows.rs
- src/ui/menu.rs
- assets/icons/

### Dependencies

```toml
[dependencies]
tray-icon = "0.14"
winit = "0.29"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.52", features = [
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_Shell",
    "Win32_Foundation"
]}
```

### Implementation Notes
- Utiliser `tray-icon` crate pour abstraction cross-platform
- 3 icônes différentes pour les 3 états
- Menu items: Start/Stop, Settings, About, Quit
- Gérer les notifications Windows natives

---

## Tasks Breakdown

- [ ] Créer module `src/ui/tray_windows.rs`
- [ ] Implémenter struct TrayIcon
- [ ] Charger icônes depuis assets (idle/recording/processing)
- [ ] Créer menu contextuel avec items
- [ ] Implémenter handlers pour chaque menu item
- [ ] Gérer double-clic pour ouvrir settings
- [ ] Implémenter changement d'icône selon état
- [ ] Tester avec explorer.exe restart
- [ ] Gérer fermeture propre de l'app
- [ ] Créer tests (difficile pour UI, tests manuels principalement)

---

## Test Plan

### Manual Testing
- [ ] Vérifier icône apparaît dans la tray
- [ ] Clic droit affiche le menu
- [ ] Double-clic ouvre fenêtre paramètres
- [ ] Chaque menu item fonctionne
- [ ] Changer d'état change l'icône
- [ ] Redémarrer explorer.exe, icône réapparaît
- [ ] Quitter ferme l'app proprement
- [ ] Notifications Windows s'affichent

---

## Documentation Updates

- [ ] Documenter l'API du tray icon
- [ ] Ajouter screenshots des états
- [ ] Expliquer les différents états

---

## Related Issues

- Blocked by: #001.2 (Cross-platform build)
- Related to: #005.2 (macOS), #005.3 (Linux), #007.2 (Settings UI)
- Part of: EPIC 5 - System Tray

---

## Notes

**Structure du code** :

```rust
// src/ui/tray_windows.rs
use tray_icon::{TrayIcon, TrayIconBuilder, menu::Menu};

pub enum TrayState {
    Idle,
    Recording,
    Processing,
}

pub struct SystemTray {
    tray_icon: TrayIcon,
    menu: Menu,
    current_state: TrayState,
}

impl SystemTray {
    pub fn new() -> Result<Self> {
        let menu = Self::create_menu()?;
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip("VoiceScript")
            .with_icon(Self::load_icon(TrayState::Idle)?)
            .build()?;

        Ok(Self {
            tray_icon,
            menu,
            current_state: TrayState::Idle,
        })
    }

    fn create_menu() -> Result<Menu> {
        let menu = Menu::new();
        menu.append(&MenuItem::with_id("start", "Start Recording", true, None))?;
        menu.append(&MenuItem::with_id("settings", "Settings...", true, None))?;
        menu.append(&PredefinedMenuItem::separator())?;
        menu.append(&MenuItem::with_id("about", "About", true, None))?;
        menu.append(&MenuItem::with_id("quit", "Quit", true, None))?;
        Ok(menu)
    }

    pub fn set_state(&mut self, state: TrayState) -> Result<()> {
        self.current_state = state;
        let icon = Self::load_icon(state)?;
        self.tray_icon.set_icon(Some(icon))?;
        Ok(())
    }

    fn load_icon(state: TrayState) -> Result<Icon> {
        let icon_path = match state {
            TrayState::Idle => "assets/icons/idle.ico",
            TrayState::Recording => "assets/icons/recording.ico",
            TrayState::Processing => "assets/icons/processing.ico",
        };
        Icon::from_path(icon_path, Some((32, 32)))
    }

    pub fn handle_events(&mut self) {
        // Event loop pour gérer les clics
    }
}
```

**Menu items** :
- **Start Recording** / **Stop Recording** (toggle)
- **Settings...** → Ouvre fenêtre de configuration
- **---** (séparateur)
- **About** → Dialogue "À propos"
- **Quit** → Ferme l'application

**Icônes requirements** :
- Format: .ico (Windows natif)
- Tailles: 16x16, 32x32, 48x48 dans le même fichier
- 3 versions: idle (gris), recording (rouge), processing (bleu animé si possible)

**Notifications Windows** :
```rust
use windows::UI::Notifications::{ToastNotification, ToastNotificationManager};

fn send_notification(title: &str, message: &str) {
    // Utiliser Windows Toast Notifications
}
```

---

## Definition of Done

- [ ] Icône fonctionnelle dans system tray
- [ ] Menu opérationnel
- [ ] 3 états visuels fonctionnent
- [ ] Fermeture propre
- [ ] Tests manuels passés
- [ ] Documentation complète
- [ ] Issue moved to done folder

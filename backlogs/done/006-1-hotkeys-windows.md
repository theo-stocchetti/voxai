# [ISSUE-006.1] Capture de raccourcis globaux (Windows)

**Created**: 2025-11-14
**Priority**: Critique
**Type**: Feature
**Status**: Todo
**Estimated Effort**: 6h
**EPIC**: 6 - Raccourcis Clavier

---

## Description

Implémenter la capture de raccourcis clavier globaux sur Windows pour déclencher/arrêter l'enregistrement. Le raccourci doit fonctionner même quand l'application n'a pas le focus.

---

## Context

Le raccourci global (par défaut Ctrl+Shift+Space) est le moyen principal d'interaction avec l'application. Il doit être robuste et ne pas créer de conflits avec d'autres applications.

---

## Acceptance Criteria

- [ ] Raccourci global fonctionnel (ex: Ctrl+Shift+Space)
- [ ] Pas de conflit avec raccourcis système Windows
- [ ] Enregistrement/désenregistrement propre du hotkey
- [ ] Notification si le raccourci est déjà utilisé
- [ ] Toggle: 1er appui start, 2ème appui stop

---

## Technical Details

### Affected Components
- src/hotkeys/windows.rs
- src/hotkeys/mod.rs
- src/config.rs (hotkey configuration)

### Dependencies

```toml
[dependencies]
global-hotkey = "0.5"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.52", features = [
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_Foundation"
]}
```

### Implementation Notes
- Utiliser `global-hotkey` crate (cross-platform)
- Fallback vers WinAPI si nécessaire
- Gérer les modificateurs: Ctrl, Shift, Alt, Win
- Détecter les conflits avant enregistrement

---

## Tasks Breakdown

- [ ] Créer module `src/hotkeys/windows.rs`
- [ ] Implémenter struct GlobalHotkey
- [ ] Parser configuration du hotkey (string → keycodes)
- [ ] Enregistrer le hotkey global avec WinAPI
- [ ] Implémenter event loop pour écouter le hotkey
- [ ] Gérer les erreurs (hotkey déjà utilisé)
- [ ] Implémenter désenregistrement propre
- [ ] Créer toggle logic (start/stop)
- [ ] Ajouter logging pour debug
- [ ] Tester avec différentes combinaisons

---

## Test Plan

### Unit Tests
- [ ] Test de parsing de hotkey string
- [ ] Test de conversion key → virtual keycode

### Integration Tests
- [ ] Enregistrer et déclencher hotkey
- [ ] Test de conflit (enregistrer 2x le même)
- [ ] Test de désenregistrement

### Manual Testing
- [ ] Appuyer sur Ctrl+Shift+Space déclenche l'enregistrement
- [ ] 2ème appui arrête l'enregistrement
- [ ] Hotkey fonctionne dans n'importe quelle application
- [ ] Fermer l'app désenregistre le hotkey
- [ ] Tester avec Ctrl+Alt+Space, Win+Space, etc.

---

## Documentation Updates

- [ ] Documenter les combinaisons supportées
- [ ] Expliquer comment changer le raccourci
- [ ] Lister les raccourcis Windows à éviter

---

## Related Issues

- Blocked by: #001.2 (Cross-platform build)
- Related to: #006.2 (macOS), #006.3 (Linux), #006.4 (Hotkey config UI)
- Part of: EPIC 6 - Raccourcis Clavier

---

## Notes

**Structure du code** :

```rust
// src/hotkeys/windows.rs
use global_hotkey::{GlobalHotKeyManager, HotKeyState, hotkey::{HotKey, Modifiers, Code}};

pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
    callback: Box<dyn Fn(HotKeyState) + Send>,
}

impl HotkeyManager {
    pub fn new(hotkey_str: &str, callback: impl Fn(HotKeyState) + Send + 'static) -> Result<Self> {
        let manager = GlobalHotKeyManager::new()?;
        let hotkey = Self::parse_hotkey(hotkey_str)?;

        manager.register(hotkey)?;

        Ok(Self {
            manager,
            hotkey,
            callback: Box::new(callback),
        })
    }

    fn parse_hotkey(s: &str) -> Result<HotKey> {
        // "Ctrl+Shift+Space" → HotKey
        let parts: Vec<&str> = s.split('+').collect();
        let mut modifiers = Modifiers::empty();
        let mut key_code = None;

        for part in parts {
            match part.to_lowercase().as_str() {
                "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
                "shift" => modifiers |= Modifiers::SHIFT,
                "alt" => modifiers |= Modifiers::ALT,
                "win" | "super" => modifiers |= Modifiers::SUPER,
                "space" => key_code = Some(Code::Space),
                "a" => key_code = Some(Code::KeyA),
                // ... autres touches
                _ => return Err(anyhow!("Unknown key: {}", part)),
            }
        }

        let key_code = key_code.ok_or_else(|| anyhow!("No key specified"))?;
        Ok(HotKey::new(Some(modifiers), key_code))
    }

    pub fn listen(&self) -> Result<()> {
        let receiver = self.manager.receiver();

        loop {
            if let Ok(event) = receiver.recv() {
                if event.id == self.hotkey.id() {
                    (self.callback)(event.state);
                }
            }
        }
    }

    pub fn unregister(&mut self) -> Result<()> {
        self.manager.unregister(self.hotkey)?;
        Ok(())
    }
}
```

**Utilisation** :

```rust
// Dans main.rs
let hotkey_manager = HotkeyManager::new("Ctrl+Shift+Space", |state| {
    if state == HotKeyState::Pressed {
        toggle_recording();
    }
})?;

// Lancer l'event loop dans un thread
thread::spawn(move || {
    hotkey_manager.listen().unwrap();
});
```

**Raccourcis Windows à éviter** :
- `Win+L` (Lock screen)
- `Ctrl+Alt+Del` (Security screen)
- `Alt+Tab` (Task switcher)
- `Win+D` (Show desktop)
- `Win+E` (Explorer)

**Raccourci par défaut suggéré** :
- `Ctrl+Shift+Space` (généralement libre, intuitif)

**Alternatives** :
- `Ctrl+Alt+V` (V pour Voice)
- `Win+Shift+V`

---

## Definition of Done

- [ ] Hotkey global fonctionnel
- [ ] Toggle start/stop implémenté
- [ ] Gestion d'erreurs robuste
- [ ] Tests manuels passés
- [ ] Documentation complète
- [ ] Issue moved to done folder

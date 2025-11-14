# [ISSUE-008.1] Injection du texte transcrit (Windows)

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: 6h
**EPIC**: 8 - Sortie et Injection du Texte

---

## Description

Simuler la frappe clavier avec l'API Windows SendInput pour injecter le texte transcrit dans l'application active, avec support Unicode complet et option de copie dans le presse-papiers.

---

## Context

Une fois le texte transcrit, il doit être injecté dans l'application qui a le focus (éditeur de texte, navigateur, etc.) comme si l'utilisateur l'avait tapé au clavier.

---

## Acceptance Criteria

- [ ] Texte injecté dans l'application active (n'importe laquelle)
- [ ] Support Unicode complet (caractères spéciaux, emojis, etc.)
- [ ] Option copie dans le presse-papiers activable
- [ ] Pas de perte de focus de l'application cible
- [ ] Délai configurable entre caractères si nécessaire

---

## Technical Details

### Affected Components
- src/output/text_injector_windows.rs
- src/output/clipboard.rs

### Dependencies

```toml
[target.'cfg(windows)'.dependencies]
windows = { version = "0.52", features = [
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_UI_WindowsAndMessaging",
    "Win32_Foundation"
]}
enigo = "0.2"  # Alternative plus simple
arboard = "3.3"  # Pour clipboard
```

### Implementation Notes
- Utiliser `SendInput` Windows API pour simulation clavier
- Alternative: crate `enigo` (cross-platform)
- Unicode nécessite `KEYEVENTF_UNICODE` flag
- Délai de 1-5ms entre caractères pour fiabilité
- Gérer les caractères spéciaux (newlines, tabs, etc.)

---

## Tasks Breakdown

- [ ] Créer module `src/output/text_injector_windows.rs`
- [ ] Implémenter struct TextInjector
- [ ] Wrapper autour de SendInput API
- [ ] Implémenter injection Unicode
- [ ] Gérer caractères spéciaux (\n, \t, etc.)
- [ ] Ajouter délai configurable entre chars
- [ ] Implémenter copie presse-papiers
- [ ] Tester avec diverses applications (Notepad, Word, browsers)
- [ ] Gérer erreurs (app fermée, permissions, etc.)
- [ ] Créer tests

---

## Test Plan

### Integration Tests
- [ ] Injecter dans Notepad
- [ ] Injecter dans VS Code
- [ ] Injecter dans navigateur (text field)
- [ ] Injecter dans Word
- [ ] Tester caractères Unicode (é, ñ, 中, 🎉)

### Manual Testing
- [ ] Transcrire et injecter dans diverses apps
- [ ] Vérifier que le focus reste sur l'app cible
- [ ] Tester copie presse-papiers
- [ ] Tester avec texte long (>500 caractères)
- [ ] Tester newlines et formatting

---

## Documentation Updates

- [ ] Documenter l'API d'injection
- [ ] Expliquer les limitations éventuelles
- [ ] Documenter les options de configuration

---

## Related Issues

- Blocked by: #003.1 (Transcription pipeline)
- Related to: #008.2 (macOS), #008.3 (Linux), #008.4 (Text formatting)
- Part of: EPIC 8 - Text Output

---

## Notes

**Structure du code** :

```rust
// src/output/text_injector_windows.rs
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_UNICODE
};

pub struct TextInjector {
    delay_between_chars_ms: u64,
    also_copy_to_clipboard: bool,
}

impl TextInjector {
    pub fn new() -> Self {
        Self {
            delay_between_chars_ms: 1,
            also_copy_to_clipboard: false,
        }
    }

    pub fn inject_text(&self, text: &str) -> Result<()> {
        // Option: copier dans le clipboard
        if self.also_copy_to_clipboard {
            self.copy_to_clipboard(text)?;
        }

        // Injecter caractère par caractère
        for ch in text.chars() {
            self.send_unicode_char(ch)?;

            if self.delay_between_chars_ms > 0 {
                std::thread::sleep(Duration::from_millis(self.delay_between_chars_ms));
            }
        }

        Ok(())
    }

    fn send_unicode_char(&self, ch: char) -> Result<()> {
        unsafe {
            let mut input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: 0,
                        wScan: ch as u16,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }

        Ok(())
    }

    fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        use arboard::Clipboard;
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(text)?;
        Ok(())
    }

    pub fn set_delay(&mut self, ms: u64) {
        self.delay_between_chars_ms = ms;
    }

    pub fn set_clipboard_copy(&mut self, enabled: bool) {
        self.also_copy_to_clipboard = enabled;
    }
}
```

**Alternative avec enigo** (plus simple) :

```rust
use enigo::{Enigo, KeyboardControllable};

pub struct TextInjector {
    enigo: Enigo,
}

impl TextInjector {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(),
        }
    }

    pub fn inject_text(&mut self, text: &str) -> Result<()> {
        self.enigo.key_sequence(text);
        Ok(())
    }
}
```

**Gestion des caractères spéciaux** :

```rust
fn handle_special_char(&self, ch: char) -> Result<()> {
    match ch {
        '\n' => self.send_key(VK_RETURN),
        '\t' => self.send_key(VK_TAB),
        _ => self.send_unicode_char(ch),
    }
}
```

**Applications testées** :
- Notepad / Notepad++ (basique)
- VS Code / Visual Studio (IDE)
- Microsoft Word (rich text)
- Chrome / Edge / Firefox (text fields)
- Discord / Slack (chat)

**Limitations connues** :
- Certaines applications avec sécurité renforcée peuvent bloquer SendInput
- Jeux en fullscreen peuvent ignorer les inputs
- Applications UAC élevées peuvent nécessiter élévation

---

## Definition of Done

- [ ] Injection fonctionnelle dans applications principales
- [ ] Unicode supporté
- [ ] Clipboard option implémentée
- [ ] Tests manuels passés
- [ ] Documentation complète
- [ ] Issue moved to done folder

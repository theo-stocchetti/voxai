# [ISSUE-013-4] Profiles and Context Switching (Phase 2)

**Created**: 2025-11-14
**Priority**: Basse
**Type**: Feature
**Status**: Todo
**Estimated Effort**: L
**EPIC**: EPIC 13 - Fonctionnalités Avancées (Phase 2)

---

## Description

Allow users to create multiple configuration profiles for different contexts (work, personal, coding, writing) and quickly switch between them.

---

## Context

Phase 2 feature. Profiles are useful for users who use VoxAI in different contexts with different settings (model, language, formatting, macros).

---

## Acceptance Criteria

- [ ] Users can create multiple profiles
- [ ] Each profile has independent settings
- [ ] Quick profile switching (tray menu or hotkey)
- [ ] Profile-specific macros and formatting rules
- [ ] Import/export profiles
- [ ] Default profile setting

---

## Technical Details

**Profile Structure**:
```rust
struct Profile {
    name: String,
    config: Config,
    macros: Vec<Macro>,
    formatting_rules: FormattingConfig,
}

struct ProfileManager {
    profiles: Vec<Profile>,
    active_profile: String,
}
```

**Example Profiles**:
- **Work**: English, formal formatting, email macros, no filler removal
- **Personal**: Casual formatting, smart quotes, filler removal
- **Coding**: No auto-capitalize, no punctuation fixes, code macros
- **Writing**: Auto-capitalize, smart quotes, grammar-aware formatting

**Profile UI**:
- Profile selector in settings
- "New Profile" button
- "Duplicate Profile" button
- Quick switcher in tray menu

---

## Tasks Breakdown

- [ ] Design profile configuration schema
- [ ] Implement profile manager
- [ ] Add profile selector to settings
- [ ] Implement profile switching
- [ ] Add profile-specific macros
- [ ] Add quick switcher to tray menu
- [ ] Implement import/export
- [ ] Test profile switching

---

## Notes

**Phase 2 Feature** - Not required for MVP.

**Use Cases**:
- Work vs. personal
- Different languages
- Different writing styles
- Different applications (email, chat, documents, code)

---

## Definition of Done

- [ ] Profiles implemented
- [ ] Profile switching working
- [ ] Profile UI complete
- [ ] Documentation updated
- [ ] Issue moved to done folder

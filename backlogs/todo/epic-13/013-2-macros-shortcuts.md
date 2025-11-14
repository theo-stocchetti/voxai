# [ISSUE-013-2] Custom Macros and Text Shortcuts (Phase 2)

**Created**: 2025-11-14
**Priority**: Basse
**Type**: Feature
**Status**: Todo
**Estimated Effort**: L
**EPIC**: EPIC 13 - Fonctionnalités Avancées (Phase 2)

---

## Description

Allow users to create custom text macros and shortcuts that expand to predefined text. For example, saying "my email" expands to "user@example.com".

---

## Context

Phase 2 feature. Macros improve productivity for frequently used text snippets like email addresses, phone numbers, common phrases, signatures, etc.

---

## Acceptance Criteria

- [ ] Users can define custom macros in settings
- [ ] Macros expand during transcription
- [ ] Support variables (date, time, clipboard)
- [ ] Import/export macro sets
- [ ] Macros work across all applications

---

## Technical Details

**Macro Definition**:
```rust
struct Macro {
    trigger: String,       // "my email"
    expansion: String,     // "user@example.com"
    variables: Vec<Variable>,
}

enum Variable {
    Date,           // {date}
    Time,           // {time}
    Clipboard,      // {clipboard}
    Custom(String), // {name}
}
```

**Example Macros**:
- "my email" → "john@example.com"
- "my address" → "123 Main St, City, State 12345"
- "signature" → "Best regards,\nJohn Doe\n{date}"
- "meeting link" → "https://zoom.us/j/1234567890"

---

## Tasks Breakdown

- [ ] Design macro configuration schema
- [ ] Implement macro parser
- [ ] Implement variable substitution
- [ ] Add macro editor to settings UI
- [ ] Implement import/export
- [ ] Test macro expansion

---

## Notes

**Phase 2 Feature** - Not required for MVP.

---

## Definition of Done

- [ ] Macros implemented
- [ ] Macro editor in settings
- [ ] Variables working
- [ ] Documentation updated
- [ ] Issue moved to done folder

# [ISSUE-013-3] Transcription History and Management (Phase 2)

**Created**: 2025-11-14
**Priority**: Basse
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 13 - Fonctionnalités Avancées (Phase 2)

---

## Description

Implement transcription history to save all past transcriptions, allow searching, copying, and re-using previous transcriptions.

---

## Context

Phase 2 feature. History is useful for retrieving past transcriptions, building a personal knowledge base, and avoiding re-typing.

---

## Acceptance Criteria

- [ ] All transcriptions automatically saved to history
- [ ] History viewer UI (searchable, sortable)
- [ ] Export history (JSON, CSV, text)
- [ ] Clear history option
- [ ] Copy transcription from history
- [ ] Delete individual entries
- [ ] Configurable retention period

---

## Technical Details

**History Storage**:
```rust
struct HistoryEntry {
    id: String,
    timestamp: DateTime<Utc>,
    text: String,
    duration: f32,      // Audio duration in seconds
    model: String,      // Model used (tiny, base, etc.)
    language: String,   // Detected language
}
```

**Storage Location**:
- `~/.voxai/history.db` (SQLite database)
- Or JSON file: `~/.voxai/history.json`

**History UI**:
- List view with timestamps
- Search box
- Sort by date, length, language
- Export button
- Clear history button

---

## Tasks Breakdown

- [ ] Design history storage schema
- [ ] Implement history database (SQLite or JSON)
- [ ] Save transcriptions to history
- [ ] Create history viewer UI
- [ ] Implement search functionality
- [ ] Implement export (JSON, CSV, text)
- [ ] Add clear/delete options
- [ ] Add retention settings

---

## Notes

**Phase 2 Feature** - Not required for MVP.

**Privacy Consideration**: Allow users to disable history if desired.

---

## Definition of Done

- [ ] History implemented
- [ ] History viewer UI working
- [ ] Search and export functional
- [ ] Documentation updated
- [ ] Issue moved to done folder

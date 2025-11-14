# [ISSUE-003-3] Multi-Language Support for Transcription

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 3 - Transcription avec Whisper

---

## Description

Implement multi-language detection and transcription support to allow users to transcribe audio in languages other than English. Whisper natively supports 99+ languages, and we need to expose this capability through configuration and automatic language detection.

---

## Context

Whisper models support multiple languages out of the box. Users may want to transcribe in their native language or switch between languages. This feature is essential for international users and makes VoxAI more accessible globally.

---

## Acceptance Criteria

- [ ] Language can be configured in settings (auto-detect or specific language)
- [ ] Auto-detection works correctly for common languages
- [ ] Manual language selection includes all Whisper-supported languages
- [ ] Language setting persists across sessions
- [ ] Transcription quality is maintained across languages
- [ ] UI displays current language setting

---

## Technical Details

### Affected Components
- `src/transcription/whisper.rs` - Add language parameter to transcription
- `src/config/schema.rs` - Add language configuration field
- `src/ui/settings.rs` - Add language selector in settings UI

### Dependencies
- [ ] Persistent configuration (007-1)
- [ ] Settings UI (007-2)

### Implementation Notes
- Whisper supports automatic language detection via `detect_language()` API
- Language codes follow ISO 639-1 standard (en, fr, es, de, etc.)
- Auto-detection adds slight latency (~50-100ms) but improves UX
- Some languages perform better with specific models (e.g., Chinese with medium+)
- Need to handle language switching without reloading model

**Supported Languages** (partial list):
- English, Spanish, French, German, Italian, Portuguese
- Chinese (Mandarin), Japanese, Korean
- Russian, Arabic, Hindi
- And 80+ more languages

**API Usage**:
```rust
let ctx = WhisperContext::new("model.bin")?;
let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

// Auto-detect
params.set_language(None); // Will auto-detect

// Or specific language
params.set_language(Some("fr")); // French

ctx.full(params, &audio_data)?;
```

---

## Tasks Breakdown

- [ ] Add `language` field to config schema (String, default: "auto")
- [ ] Create language selector component in settings UI
- [ ] Implement language dropdown with all supported languages
- [ ] Add language parameter to Whisper transcription function
- [ ] Implement auto-detection logic
- [ ] Handle language code validation
- [ ] Add language indicator to system tray menu
- [ ] Test transcription in 5+ different languages
- [ ] Document language support in user docs

---

## Test Plan

### Unit Tests
- [ ] Test language code validation (valid/invalid codes)
- [ ] Test auto-detection flag handling
- [ ] Test language parameter passing to Whisper

### Integration Tests
- [ ] Test transcription in English, French, Spanish, German, Chinese
- [ ] Test auto-detection with mixed language audio
- [ ] Test language switching without app restart
- [ ] Test language persistence across sessions

### Manual Testing
- [ ] Verify language dropdown shows all supported languages
- [ ] Test transcription accuracy in multiple languages
- [ ] Verify auto-detection works correctly
- [ ] Test edge cases (very short audio, mixed languages)

---

## Documentation Updates

- [ ] Update README.md with supported languages list
- [ ] Update CLAUDE.md with language configuration details
- [ ] Add user guide for language selection
- [ ] Document auto-detection behavior and limitations

---

## Related Issues

- Depends on: #007-1 (Persistent Configuration)
- Depends on: #007-2 (Settings UI)
- Related to: #003-2 (Model Management - some models better for certain languages)

---

## Notes

**Language Performance**:
- Tiny/Base models: Good for English, Spanish, French, German
- Small+ models: Required for Asian languages (Chinese, Japanese, Korean)
- Medium+ models: Best for Arabic, Hindi, and complex scripts

**Auto-detection considerations**:
- Adds ~50-100ms latency
- Very accurate for audio > 3 seconds
- May struggle with very short utterances (< 1 second)
- Consider caching detected language to avoid re-detection

**Future enhancements** (Phase 2):
- Language-specific post-processing rules
- Custom vocabulary per language
- Mixed-language transcription (code-switching)

---

## Definition of Done

- [ ] Language configuration implemented in config schema
- [ ] Language selector added to settings UI
- [ ] Whisper integration supports language parameter
- [ ] Auto-detection implemented and tested
- [ ] Transcription works in at least 5 different languages
- [ ] Tests written and passing
- [ ] Documentation updated
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

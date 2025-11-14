# [ISSUE-010-1] Unit Tests Implementation

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Chore
**Status**: Todo
**Estimated Effort**: L
**EPIC**: EPIC 10 - Tests et Qualité

---

## Description

Implement comprehensive unit tests for all core modules to ensure code quality, catch regressions, and facilitate refactoring. Target 80%+ code coverage.

---

## Context

Unit tests are essential for maintaining code quality as the project grows. They catch bugs early, document expected behavior, and enable confident refactoring.

---

## Acceptance Criteria

- [ ] Unit tests for audio capture module
- [ ] Unit tests for transcription module
- [ ] Unit tests for configuration module
- [ ] Unit tests for text formatting module
- [ ] Unit tests for hotkey modules
- [ ] Code coverage > 80%
- [ ] All tests pass on `cargo test`
- [ ] Tests run in CI pipeline

---

## Technical Details

### Affected Components
- All `src/*/mod.rs` modules
- Add `#[cfg(test)] mod tests` blocks

### Implementation Notes

**Test Structure**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_capture_initialization() {
        let capture = AudioCapture::new();
        assert!(capture.is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }
}
```

**Modules to Test**:
- `src/audio/*` - Audio capture, VAD, noise reduction
- `src/transcription/*` - Whisper integration, model management
- `src/config/*` - Configuration parsing, validation
- `src/output/formatter.rs` - Text post-processing
- `src/hotkeys/*` - Hotkey parsing (mocking actual registration)

---

## Tasks Breakdown

- [ ] Add test dependencies (mockall, proptest, etc.)
- [ ] Write unit tests for audio module
- [ ] Write unit tests for transcription module
- [ ] Write unit tests for config module
- [ ] Write unit tests for output/formatter module
- [ ] Write unit tests for hotkey modules
- [ ] Set up test coverage reporting (tarpaulin)
- [ ] Ensure all tests pass
- [ ] Document testing strategy in CLAUDE.md

---

## Test Plan

Run: `cargo test --all-features`

---

## Documentation Updates

- [ ] Update CLAUDE.md with testing strategy
- [ ] Document how to run tests

---

## Definition of Done

- [ ] Unit tests implemented for all core modules
- [ ] Code coverage > 80%
- [ ] All tests passing
- [ ] Tests run in CI
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

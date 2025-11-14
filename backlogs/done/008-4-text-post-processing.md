# [ISSUE-008-4] Text Post-Processing and Formatting

**Created**: 2025-11-14
**Priority**: Moyenne
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 8 - Sortie et Injection du Texte

---

## Description

Implement text post-processing to automatically improve transcribed text quality before injection. This includes capitalization, punctuation cleanup, smart quotes, and custom formatting rules.

---

## Context

Whisper transcription output may need refinement before insertion into documents. Automatic formatting improves usability and reduces manual editing. Users should have control over which post-processing steps to apply.

---

## Acceptance Criteria

- [ ] Auto-capitalize first letter of sentences
- [ ] Add period at end if missing
- [ ] Convert straight quotes to smart quotes (optional)
- [ ] Trim leading/trailing whitespace
- [ ] Remove filler words (optional: "um", "uh", "like")
- [ ] Replace numbers with words or vice versa (optional)
- [ ] User can enable/disable each post-processing step
- [ ] Post-processing is configurable in settings

---

## Technical Details

### Affected Components
- `src/output/formatter.rs` - New text formatting module
- `src/output/mod.rs` - Apply formatting before injection
- `src/config/schema.rs` - Add formatting configuration options

### Dependencies
- [ ] Settings UI (007-2) - for configuration
- [ ] Text injection implementations (008-1, 008-2, 008-3)

### Implementation Notes

**Formatting Pipeline**:
```rust
pub struct TextFormatter {
    config: FormattingConfig,
}

#[derive(Serialize, Deserialize)]
pub struct FormattingConfig {
    auto_capitalize: bool,
    smart_quotes: bool,
    remove_filler_words: bool,
    add_period_at_end: bool,
    trim_whitespace: bool,
}

impl TextFormatter {
    pub fn format(&self, text: &str) -> String {
        let mut result = text.to_string();

        if self.config.trim_whitespace {
            result = result.trim().to_string();
        }

        if self.config.remove_filler_words {
            result = self.remove_fillers(&result);
        }

        if self.config.auto_capitalize {
            result = self.capitalize_sentences(&result);
        }

        if self.config.smart_quotes {
            result = self.convert_smart_quotes(&result);
        }

        if self.config.add_period_at_end {
            if !result.ends_with(['.', '!', '?']) {
                result.push('.');
            }
        }

        result
    }

    fn remove_fillers(&self, text: &str) -> String {
        let fillers = ["um", "uh", "like", "you know", "I mean"];
        let mut result = text.to_string();

        for filler in &fillers {
            let pattern = format!(r"\b{}\b", regex::escape(filler));
            let re = regex::Regex::new(&pattern).unwrap();
            result = re.replace_all(&result, "").to_string();
        }

        // Clean up extra spaces
        let re = regex::Regex::new(r"\s+").unwrap();
        re.replace_all(&result, " ").trim().to_string()
    }

    fn capitalize_sentences(&self, text: &str) -> String {
        let mut result = String::new();
        let mut new_sentence = true;

        for c in text.chars() {
            if new_sentence && c.is_alphabetic() {
                result.push(c.to_uppercase().next().unwrap());
                new_sentence = false;
            } else {
                result.push(c);
            }

            if c == '.' || c == '!' || c == '?' {
                new_sentence = true;
            }
        }

        result
    }

    fn convert_smart_quotes(&self, text: &str) -> String {
        text.replace('"', """)
            .replace(" '", " '")
            .replace("'", "'")
    }
}
```

**Configurable Options**:
- Auto-capitalize: True/False (default: True)
- Smart quotes: True/False (default: False)
- Remove fillers: True/False (default: False)
- Add period: True/False (default: True)
- Trim whitespace: True/False (default: True)

---

## Tasks Breakdown

- [ ] Create `src/output/formatter.rs` module
- [ ] Implement TextFormatter struct
- [ ] Implement trim_whitespace function
- [ ] Implement auto-capitalization of sentences
- [ ] Implement add_period_at_end function
- [ ] Implement smart quotes conversion
- [ ] Implement filler word removal (optional)
- [ ] Add FormattingConfig to config schema
- [ ] Add formatting options to settings UI
- [ ] Integrate formatter into text injection pipeline
- [ ] Add unit tests for each formatting function
- [ ] Test with various text inputs
- [ ] Document formatting options in settings

---

## Test Plan

### Unit Tests
- [ ] Test trim whitespace
- [ ] Test sentence capitalization
- [ ] Test period addition
- [ ] Test smart quotes conversion
- [ ] Test filler word removal
- [ ] Test edge cases (empty string, single word, etc.)

### Integration Tests
- [ ] Test full formatting pipeline
- [ ] Test with real Whisper transcription output
- [ ] Test with various config combinations

### Manual Testing
- [ ] Transcribe text and verify auto-capitalization
- [ ] Verify period is added at end
- [ ] Test smart quotes in various contexts
- [ ] Test filler word removal (say "um", "uh" in recording)
- [ ] Disable all formatting and verify raw text
- [ ] Test with multiple sentences

---

## Documentation Updates

- [ ] Update README.md with formatting features
- [ ] Update CLAUDE.md with formatter implementation
- [ ] Document all formatting options in user guide
- [ ] Add examples of before/after formatting

---

## Related Issues

- Related to: #008-1, #008-2, #008-3 (Text injection - uses formatter)
- Depends on: #007-2 (Settings UI - formatting config)

---

## Notes

**Text Formatting Best Practices**:
- Keep original meaning intact
- Provide clear on/off toggles for each feature
- Show preview of formatting (future enhancement)
- Don't be overly aggressive (preserve user intent)

**Common Filler Words**:
- English: "um", "uh", "like", "you know", "I mean", "sort of", "kind of"
- May vary by language (future: language-specific fillers)

**Smart Quotes**:
- Use curly quotes (" " ' ') instead of straight quotes (" ')
- Improves typography in documents
- Some applications auto-convert anyway

**Capitalization Rules**:
- First letter of sentence (after . ! ?)
- Proper nouns (future: NER for proper capitalization)
- Acronyms (future: detect and preserve)

**Future Enhancements**:
- Custom formatting rules (user-defined replacements)
- Language-specific formatting (French spacing, etc.)
- Number formatting (1000 → 1,000 or "one thousand")
- Date/time formatting
- Formatting templates/presets
- Preview mode (show before applying)
- Undo formatting (revert to raw transcription)
- Named entity recognition for proper capitalization
- Context-aware formatting (email vs. document vs. code)

---

## Definition of Done

- [ ] Text formatter implemented with configurable options
- [ ] All formatting functions working correctly
- [ ] Formatting configuration in settings UI
- [ ] Formatter integrated into text injection
- [ ] Unit tests passing for all formatting functions
- [ ] Documentation updated
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

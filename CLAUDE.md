# CLAUDE.md - AI Assistant Guide for VoxAI

**Last Updated**: 2025-11-14
**Repository**: theo-stocchetti/voxai
**Status**: New Repository - Initial Setup Phase

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Repository Structure](#repository-structure)
3. [Development Workflows](#development-workflows)
4. [Code Conventions & Standards](#code-conventions--standards)
5. [AI Assistant Guidelines](#ai-assistant-guidelines)
6. [Testing Strategy](#testing-strategy)
7. [Deployment & Infrastructure](#deployment--infrastructure)
8. [Common Tasks & Patterns](#common-tasks--patterns)
9. [Troubleshooting](#troubleshooting)

---

## Project Overview

### About VoxAI

**Status**: This is a new repository currently in the initial setup phase.

**Purpose**: VoxAI is a desktop application for real-time audio transcription, inspired by VoiceInk and WisprFlow. It allows users to transcribe speech to text using a global keyboard shortcut, with support for multiple platforms (Windows 11, macOS, Linux) and hardware acceleration.

**Key Technologies**:
- **Language**: Rust (edition 2021)
- **Transcription Engine**: Whisper.cpp with whisper-rs bindings
- **Audio Capture**: CPAL (cross-platform audio library)
- **Async Runtime**: Tokio
- **UI**: System tray application (tray-icon, egui for settings)

**Architecture**: Desktop application with system tray interface, global hotkey support, and local audio processing pipeline.

**Core Features**:
- Real-time audio transcription using Whisper AI
- Global keyboard shortcut activation
- Multi-platform support (Windows, macOS Intel/ARM, Linux)
- Hardware acceleration (CUDA, Metal, OpenCL)
- Noise reduction and voice activity detection
- Text injection into active application
- Fully local processing (no cloud dependency)

### Project Goals

1. **Create a fast, accurate voice-to-text application** with latency < 2 seconds
2. **Ensure cross-platform compatibility** on Windows, macOS (Intel & ARM), and Linux
3. **Optimize performance** with GPU acceleration and efficient CPU usage
4. **Provide excellent UX** with minimal configuration and intuitive operation
5. **Maintain privacy** with 100% local processing, no data sent to cloud

---

## Repository Structure

VoxAI follows a standard Rust project structure with modular organization:

```
voxai/
├── src/                        # Source code
│   ├── main.rs                # Application entry point
│   ├── audio/                 # Audio capture and processing
│   │   ├── mod.rs
│   │   ├── capture.rs        # CPAL audio capture
│   │   ├── device.rs         # Device enumeration
│   │   ├── buffer.rs         # Ring buffer management
│   │   ├── noise_reduction.rs # RNNoise integration
│   │   └── vad.rs            # Voice Activity Detection
│   ├── transcription/         # Whisper integration
│   │   ├── mod.rs
│   │   ├── whisper.rs        # Whisper context wrapper
│   │   ├── models.rs         # Model management
│   │   ├── downloader.rs     # Model downloading
│   │   ├── pipeline.rs       # Transcription pipeline
│   │   └── chunking.rs       # Audio chunking logic
│   ├── ui/                    # User interface
│   │   ├── mod.rs
│   │   ├── tray_windows.rs   # Windows system tray
│   │   ├── tray_macos.rs     # macOS menu bar
│   │   ├── tray_linux.rs     # Linux system tray
│   │   ├── menu.rs           # Menu management
│   │   └── settings.rs       # Settings window (egui)
│   ├── hotkeys/               # Global keyboard shortcuts
│   │   ├── mod.rs
│   │   ├── windows.rs        # Windows hotkey implementation
│   │   ├── macos.rs          # macOS hotkey implementation
│   │   └── linux.rs          # Linux hotkey implementation
│   ├── output/                # Text output
│   │   ├── mod.rs
│   │   ├── text_injector_windows.rs
│   │   ├── text_injector_macos.rs
│   │   ├── text_injector_linux.rs
│   │   ├── clipboard.rs      # Clipboard management
│   │   └── formatter.rs      # Text formatting
│   ├── config/                # Configuration management
│   │   ├── mod.rs
│   │   └── schema.rs         # Config schema
│   └── gpu/                   # GPU acceleration (optional)
│       ├── mod.rs
│       ├── cuda.rs           # CUDA support
│       ├── metal.rs          # Metal support (Apple Silicon)
│       └── opencl.rs         # OpenCL support
├── tests/                     # Integration tests
│   ├── audio_tests.rs
│   ├── transcription_tests.rs
│   └── integration_tests.rs
├── benches/                   # Performance benchmarks
│   └── transcription_bench.rs
├── assets/                    # Assets (icons, etc.)
│   └── icons/
│       ├── idle.ico
│       ├── recording.ico
│       └── processing.ico
├── scripts/                   # Build scripts
│   ├── build-windows.sh
│   ├── build-macos.sh
│   └── build-linux.sh
├── backlogs/                  # Issue tracking (Markdown)
│   ├── README.md
│   ├── templates/
│   ├── todo/
│   ├── in-progress/
│   └── done/
├── .github/                   # GitHub workflows
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
├── Cargo.toml                 # Rust dependencies
├── Cargo.lock                 # Locked dependencies
├── build.rs                   # Build script
├── .gitignore
├── CLAUDE.md                  # This file
├── README.md                  # Project README
└── LICENSE                    # Project license
```

### Key Directories

- **src/audio/**: All audio capture, processing, noise reduction, and VAD logic
- **src/transcription/**: Whisper integration, model management, and transcription pipeline
- **src/ui/**: System tray implementation for each platform and settings UI
- **src/hotkeys/**: Global keyboard shortcut handling per platform
- **src/output/**: Text injection and clipboard management
- **src/config/**: Configuration persistence and management
- **backlogs/**: Local issue tracking system with Markdown files (see [Backlog Management](#backlog-management))

---

## Development Workflows

### Branch Strategy

**Main Branch**: [To be determined - typically `main` or `master`]

**Feature Branches**: Use the following naming convention:
- `claude/[feature-name]-[session-id]` - For AI-assisted development
- `feature/[feature-name]` - For human-led feature development
- `bugfix/[bug-description]` - For bug fixes
- `hotfix/[issue]` - For urgent production fixes

**Current Working Branch**: `claude/claude-md-mhy5cd8x53r818fa-011Pm5UebLDmzNzS3y7Djcpx`

### Git Workflow

1. **Create Branch**: Always create feature branches from the main branch
2. **Develop**: Make atomic commits with clear, descriptive messages
3. **Test**: Ensure all tests pass before committing
4. **Push**: Use `git push -u origin <branch-name>` for first push
5. **Pull Request**: Create PR with comprehensive description and test plan

### Commit Message Convention

Follow conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(api): add user authentication endpoint

Implements JWT-based authentication with refresh tokens.
Includes middleware for protected routes.

Closes #123
```

---

## Code Conventions & Standards

### General Principles

1. **Clarity over Cleverness**: Write code that is easy to understand
2. **DRY (Don't Repeat Yourself)**: Extract common patterns into reusable functions
3. **SOLID Principles**: Follow SOLID design principles
4. **Security First**: Never commit secrets, API keys, or sensitive data
5. **Error Handling**: Always handle errors gracefully with appropriate logging

### Naming Conventions

**Rust Conventions** (following official Rust style guide):

- **Variables**: `snake_case`
- **Functions**: `snake_case`
- **Structs/Enums/Traits**: `PascalCase`
- **Constants**: `UPPER_SNAKE_CASE`
- **Modules**: `snake_case`
- **Files**: `snake_case.rs`
- **Type Parameters**: Single uppercase letter or `PascalCase`

**Examples**:
```rust
const MAX_BUFFER_SIZE: usize = 80000;
struct AudioCapture { /* ... */ }
fn capture_audio() -> Result<Vec<f32>> { /* ... */ }
let sample_rate = 16000;
```

### Code Style

**Rust Style**:
- Use `rustfmt` for code formatting (configured in `rustfmt.toml`)
- Use `clippy` for linting and best practices
- Maximum line length: 100 characters
- Use meaningful variable names (avoid single letters except in closures)
- Add rustdoc comments (`///`) for public API
- Use `#[derive(Debug)]` on all structs for debugging
- Prefer explicit error types over `.unwrap()`
- Use `Result<T, Error>` and `anyhow` for error handling

**Code organization**:
```rust
// Module organization
mod audio {
    mod capture;  // Private by default
    pub mod vad;  // Public when needed
}

// Public API with docs
/// Captures audio from the default input device.
///
/// # Errors
/// Returns an error if no input device is available.
pub fn start_capture() -> Result<()> { /* ... */ }
```

### Security Guidelines

**CRITICAL - Always Follow These Rules**:

1. **Never commit**:
   - API keys, tokens, or credentials
   - `.env` files with real secrets
   - Private keys or certificates
   - Database passwords
   - OAuth client secrets

2. **Use environment variables** for all configuration
3. **Validate all inputs** to prevent injection attacks
4. **Sanitize user data** before display (prevent XSS)
5. **Use parameterized queries** to prevent SQL injection
6. **Keep dependencies updated** to patch security vulnerabilities
7. **Implement proper authentication and authorization**
8. **Use HTTPS** for all production endpoints

---

## AI Assistant Guidelines

### When Working on This Codebase

1. **Always Read First**: Use the `Read` tool to examine existing files before making changes
2. **Prefer Editing**: Always edit existing files rather than creating new ones when possible
3. **Use TodoWrite**: For complex tasks, use the TodoWrite tool to plan and track progress
4. **Parallel Operations**: Run independent operations in parallel for efficiency
5. **Context Awareness**: Understand the full context before making changes
6. **Test Changes**: Verify changes work before committing

### Task Planning

For complex tasks:
1. Use TodoWrite to create a task list
2. Break down large tasks into smaller, manageable steps
3. Mark tasks as `in_progress` before starting
4. Mark as `completed` immediately after finishing
5. Only one task should be `in_progress` at a time

### Code Exploration

When exploring the codebase:
- Use the `Task` tool with `subagent_type=Explore` for open-ended searches
- Use `Grep` for specific pattern searches
- Use `Glob` for finding files by name pattern
- Use `Read` for examining specific files

### Making Changes

1. **Research**: Understand existing patterns in the codebase
2. **Plan**: Use TodoWrite for multi-step changes
3. **Implement**: Make changes following established conventions
4. **Test**: Run tests to verify changes
5. **Review**: Check for security issues, code quality
6. **Commit**: Create atomic commits with clear messages
7. **Push**: Push to the designated branch

### Communication

- **Be concise**: Keep responses short and to the point
- **No emojis**: Unless explicitly requested by the user
- **Professional tone**: Focus on technical accuracy
- **Show progress**: Use TodoWrite to give visibility into progress
- **Reference code**: Use `file:line` format for code references

### Common Pitfalls to Avoid

1. **Don't** create files unnecessarily
2. **Don't** use bash commands for file operations (use Read/Edit/Write tools)
3. **Don't** guess at URLs or make assumptions
4. **Don't** commit without testing
5. **Don't** push to main/master directly
6. **Don't** include secrets in code or commits
7. **Don't** forget to update documentation when making changes

---

## Testing Strategy

### Test Philosophy

**To be defined based on project needs**

- Aim for high test coverage (80%+ recommended)
- Write tests before fixing bugs (TDD for bug fixes)
- Test edge cases and error conditions
- Keep tests fast and independent

### Test Types

1. **Unit Tests**: Test individual functions/methods
2. **Integration Tests**: Test component interactions
3. **End-to-End Tests**: Test complete user workflows
4. **Performance Tests**: Test system performance under load

### Running Tests

**Rust Testing with Cargo**:

```bash
# Run all tests
cargo test

# Run specific test module
cargo test audio::tests

# Run specific test
cargo test test_audio_capture

# Run with output (show println! in tests)
cargo test -- --nocapture

# Run benchmarks
cargo bench

# Run tests with code coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

**Test organization**:
```rust
// Unit tests in same file as code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_capture() {
        // Test logic
    }
}

// Integration tests in tests/ directory
// tests/transcription_tests.rs
#[test]
fn test_full_transcription_pipeline() {
    // Test logic
}
```

---

## Deployment & Infrastructure

### Environments

**To be defined**:
- **Development**: Local development environment
- **Staging**: Pre-production testing environment
- **Production**: Live production environment

### CI/CD Pipeline

**To be configured in .github/workflows/**

Recommended pipeline stages:
1. Lint and format check
2. Run tests
3. Build application
4. Security scanning
5. Deploy to staging
6. Deploy to production (manual approval)

### Configuration Management

- Use environment variables for configuration
- Store secrets in secure secret management system
- Document all required environment variables
- Provide `.env.example` template

---

## Common Tasks & Patterns

### Adding a New Feature

1. Create feature branch: `git checkout -b feature/feature-name`
2. Plan the feature using TodoWrite
3. Implement following code conventions
4. Write tests for the new feature
5. Update documentation
6. Create pull request

### Fixing a Bug

1. Write a failing test that reproduces the bug
2. Fix the bug
3. Verify the test passes
4. Check for similar bugs elsewhere
5. Commit with clear description

### Refactoring Code

1. Ensure existing tests pass
2. Make refactoring changes
3. Verify tests still pass
4. Update documentation if needed
5. Commit with `refactor:` prefix

### Adding Dependencies

1. Research the dependency (license, maintenance, security)
2. Add to package.json/requirements.txt/etc.
3. Document why the dependency is needed
4. Lock to specific version
5. Update documentation

---

## Backlog Management

VoxAI uses a local Markdown-based issue tracking system in the `backlogs/` directory. This allows for detailed task tracking, specifications, and progress monitoring without relying solely on GitHub Issues.

### Structure

```
backlogs/
├── README.md              # Documentation for the backlog system
├── templates/
│   └── issue-template.md  # Template for creating new issues
├── todo/                  # Issues to be done
│   ├── epic-01/          # Infrastructure & Setup
│   ├── epic-02/          # Audio Capture
│   ├── epic-03/          # Transcription
│   └── ...               # Other EPICs
├── in-progress/           # Currently being worked on
└── done/                  # Completed issues
```

### EPICs Overview

The project is organized into 13 EPICs:

1. **Infrastructure & Configuration** - Project setup, build system, Whisper integration
2. **Audio Capture** - CPAL integration, noise reduction, VAD
3. **Transcription** - Whisper pipeline, model management, multi-language
4. **Hardware Acceleration** - GPU support (CUDA, Metal, OpenCL)
5. **System Tray** - UI for Windows, macOS, Linux
6. **Keyboard Shortcuts** - Global hotkeys per platform
7. **Configuration** - Persistent settings, UI
8. **Text Output** - Text injection per platform
9. **Visual Feedback** - Overlays, notifications
10. **Testing & Quality** - Unit, integration, performance tests
11. **Deployment** - Packaging and distribution
12. **Documentation** - User and developer docs
13. **Advanced Features** - Voice commands, macros, history (Phase 2)

### Workflow for AI Assistants

When working on VoxAI issues:

1. **Read the issue**: Use `Read` tool to examine the relevant issue in `backlogs/todo/`
2. **Understand requirements**: Check acceptance criteria, technical details, and tasks
3. **Implement**: Follow the specifications in the issue
4. **Test**: Run the tests specified in the Test Plan section
5. **Update issue**: Check off completed tasks and acceptance criteria
6. **Move issue**: When complete, move from `todo/` → `in-progress/` → `done/`

**Example workflow**:
```bash
# Read an issue
cat backlogs/todo/epic-01/001-1-project-initialization.md

# After implementing, move to done
mv backlogs/todo/epic-01/001-1-project-initialization.md backlogs/done/
```

### Creating New Issues

Use the template:
```bash
cp backlogs/templates/issue-template.md backlogs/todo/epic-XX/XXX-issue-name.md
```

Fill in all sections:
- Description, Context, Acceptance Criteria
- Technical Details, Tasks Breakdown
- Test Plan, Documentation Updates
- Related Issues, Notes, Definition of Done

### Issue Format

Each issue includes:
- **Metadata**: Priority, Type, Status, Effort estimate, EPIC
- **Description**: What needs to be done
- **Context**: Why it's needed
- **Acceptance Criteria**: Measurable completion criteria
- **Technical Details**: Components, dependencies, implementation notes
- **Tasks Breakdown**: Specific sub-tasks
- **Test Plan**: Unit, integration, manual tests
- **Documentation Updates**: What docs to update
- **Related Issues**: Dependencies and blockers
- **Notes**: Code examples, references
- **Definition of Done**: Final checklist

### Priority Levels

- 🔴 **Critique**: Blocking for MVP, must be done first
- 🟠 **Haute**: Important for good UX, needed for MVP
- 🟡 **Moyenne**: Nice-to-have, can wait for Phase 2
- 🟢 **Basse**: Minor optimizations and enhancements

### Tracking Progress

AI assistants should:
- Check `backlogs/todo/` for available work
- Start with Critique and Haute priority issues
- Follow dependency chains (check "Blocked by" field)
- Update TodoWrite to track multi-step implementations
- Move completed issues to `done/` folder

See `backlogs/README.md` for complete documentation.

---

## Troubleshooting

### Common Issues

**To be documented as issues are encountered**

#### Issue: [Problem Description]
**Symptoms**: [What you see]
**Solution**: [How to fix]

---

## Project-Specific Notes

### Technology Stack

**Complete Stack**:

- **Programming Language**: Rust (edition 2021)
- **Async Runtime**: Tokio 1.35+
- **Transcription**: Whisper.cpp + whisper-rs bindings

**Core Libraries**:
- `cpal` 0.15 - Cross-platform audio capture
- `whisper-rs` 0.10 - Whisper.cpp bindings
- `tokio` 1.35 - Async runtime
- `anyhow` / `thiserror` - Error handling
- `serde` / `serde_json` - Serialization

**Audio Processing**:
- `ringbuf` 0.3 - Ring buffer for audio streaming
- `nnnoiseless` 0.5 - Noise reduction (RNNoise)
- `webrtc-vad` 0.4 - Voice Activity Detection
- `rubato` 0.14 - Audio resampling

**UI & System Integration**:
- `tray-icon` 0.14 - System tray (cross-platform)
- `global-hotkey` 0.5 - Global keyboard shortcuts
- `egui` / `eframe` 0.24 - Settings window
- `arboard` 3.3 - Clipboard management
- `enigo` 0.2 - Keyboard simulation

**Platform-Specific**:
- `windows` 0.52 - Windows APIs (SendInput, hotkeys)
- Platform-specific APIs for macOS and Linux

**GPU Acceleration** (optional features):
- CUDA - NVIDIA GPU support
- Metal - Apple Silicon GPU support
- OpenCL - AMD/Intel GPU support

**Utilities**:
- `dirs` 5.0 - Standard directories
- `reqwest` 0.11 - HTTP client (model downloading)
- `sha2` 0.10 - Checksum verification
- `log` / `env_logger` - Logging
- `sysinfo` 0.30 - System information

### External Services

**No Cloud Dependencies**:
- All processing is done locally
- Models downloaded from HuggingFace (one-time, optional)
- No API keys or external accounts required
- No telemetry or analytics

**Model Source**:
- HuggingFace: `https://huggingface.co/ggerganov/whisper.cpp`
- Models: tiny (75MB), base (142MB), small (466MB), medium (1.5GB)

### Performance Considerations

**Performance Targets**:
- **Latency**: < 2 seconds (speech end → text displayed)
- **CPU Usage**: < 25% on modern CPU (idle < 5%)
- **RAM Usage**: < 500 MB (varies with model size)
- **Transcription Speed**:
  - Tiny model: ~10x realtime (CPU)
  - Base model: ~7x realtime (CPU)
  - Small model: ~4x realtime (CPU or GPU)
  - Medium model: ~2x realtime (GPU required)

**Known Bottlenecks**:
- Whisper inference is the main bottleneck
- GPU acceleration recommended for medium+ models
- VAD helps reduce unnecessary transcriptions by 60%+

**Optimization Strategies**:
1. Use smallest model that meets accuracy needs
2. Enable GPU acceleration if available
3. Use Voice Activity Detection to skip silence
4. Optimize chunk size for balance between latency and accuracy
5. Use ring buffer to avoid memory allocations
6. Parallelize transcription of multiple chunks
7. Profile with `cargo flamegraph` to identify hotspots

---

## Additional Resources

### Documentation Links

- Project README: [To be created]
- API Documentation: [To be created]
- Architecture Decision Records: [To be created]

### External Resources

- [Project-specific resources to be added]

---

## Updating This Document

This document should be updated whenever:
- Project structure changes significantly
- New conventions are adopted
- New patterns emerge in the codebase
- Technology stack changes
- Common issues and solutions are discovered

**How to Update**:
1. Make changes to CLAUDE.md
2. Update the "Last Updated" date at the top
3. Add a note in git commit describing what was updated
4. Ensure changes are reviewed as part of PR process

---

## Version History

| Date       | Changes                                    | Updated By |
|------------|-------------------------------------------|------------|
| 2025-11-14 | Initial CLAUDE.md creation for new repository | AI Assistant |
| 2025-11-14 | Updated with VoxAI project details:<br>- Added complete project description and goals<br>- Updated repository structure for Rust project<br>- Filled in technology stack (Rust, Whisper, CPAL, etc.)<br>- Updated code conventions for Rust<br>- Updated testing section for Cargo<br>- Added backlog management system documentation<br>- Added performance targets and optimization strategies | AI Assistant |

---

**Note**: This is a living document that will evolve as the VoxAI project develops. AI assistants should consult this file before beginning work and update it as the project grows.

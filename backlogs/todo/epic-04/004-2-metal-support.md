# [ISSUE-004-2] Metal GPU Acceleration Support (Apple Silicon)

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 4 - Accélération Matérielle

---

## Description

Implement Metal GPU acceleration support for Whisper transcription on Apple Silicon (M1/M2/M3) Macs. This will provide significant performance improvements on macOS devices with Apple GPUs.

---

## Context

Whisper.cpp supports Metal acceleration for Apple Silicon Macs. M1/M2/M3 chips have powerful integrated GPUs that can accelerate transcription by 5-15x compared to CPU-only processing. This is essential for good performance on macOS.

---

## Acceptance Criteria

- [ ] Metal support enabled on macOS builds
- [ ] Application detects Apple Silicon at startup
- [ ] Transcription uses Metal GPU automatically on Apple Silicon
- [ ] Performance improvement of 5-15x for medium+ models
- [ ] Graceful fallback to CPU on Intel Macs

---

## Technical Details

### Affected Components
- `src/gpu/metal.rs` - New Metal integration module
- `src/transcription/whisper.rs` - Enable Metal backend
- `build.rs` - Add Metal compilation flags (macOS only)

### Dependencies
- [ ] Whisper.cpp compiled with Metal support
- [ ] macOS 12.3+ (Monterey or later)
- [ ] Apple Silicon Mac (M1/M2/M3)

### Implementation Notes

**Build Configuration**:
- Add `metal` feature flag to Cargo.toml
- Enable Metal backend in whisper-rs for macOS builds
- Link against Metal framework

**Platform Detection**:
```rust
#[cfg(target_os = "macos")]
pub fn is_apple_silicon() -> bool {
    std::env::consts::ARCH == "aarch64"
}

pub fn is_metal_available() -> bool {
    is_apple_silicon() && check_macos_version() >= (12, 3)
}
```

**Performance Expectations**:
- M1: 5-10x faster than CPU-only
- M2: 8-12x faster than CPU-only
- M3: 10-15x faster than CPU-only

**Memory Considerations**:
- Unified memory architecture (shared RAM/VRAM)
- No separate VRAM limit
- Limited only by total system RAM

---

## Tasks Breakdown

- [ ] Add Metal feature flag for macOS builds
- [ ] Create `src/gpu/metal.rs` module
- [ ] Implement Apple Silicon detection
- [ ] Implement macOS version check (12.3+)
- [ ] Integrate Metal support in Whisper transcription
- [ ] Add conditional compilation for macOS-only code
- [ ] Test on M1 Mac
- [ ] Test on M2 Mac (if available)
- [ ] Test fallback on Intel Mac
- [ ] Document Metal requirements
- [ ] Benchmark performance vs CPU

---

## Test Plan

### Unit Tests
- [ ] Test Apple Silicon detection
- [ ] Test macOS version check
- [ ] Test Metal availability detection

### Integration Tests
- [ ] Test transcription on M1 Mac with Metal
- [ ] Test transcription on Intel Mac (CPU fallback)
- [ ] Benchmark performance improvement
- [ ] Test with different model sizes

### Manual Testing
- [ ] Test on M1 MacBook Air/Pro
- [ ] Test on M2 MacBook Air/Pro
- [ ] Test on M3 MacBook Pro (if available)
- [ ] Test on Intel Mac (should use CPU)
- [ ] Monitor GPU usage with Activity Monitor

---

## Documentation Updates

- [ ] Update README.md with Metal support information
- [ ] Update CLAUDE.md with macOS GPU details
- [ ] Document macOS version requirements
- [ ] Add performance benchmarks for M1/M2/M3
- [ ] Note that Metal is automatic on Apple Silicon

---

## Related Issues

- Related to: #004-1 (CUDA Support - NVIDIA GPU)
- Related to: #004-3 (OpenCL Support - AMD/Intel GPU)
- Related to: #005-2 (macOS System Tray)
- Related to: #006-2 (macOS Hotkeys)

---

## Notes

**Apple Silicon Advantages**:
- Unified memory architecture (no CPU↔GPU transfers)
- Very efficient for ML workloads
- Low power consumption
- Excellent performance per watt

**macOS Version Requirements**:
- macOS 12.3+ required for optimal Metal support
- Earlier versions may have degraded performance

**Intel Mac Support**:
- Intel Macs do not support Metal acceleration for ML
- Will automatically fall back to CPU-only mode
- Consider adding warning in logs for Intel Macs

**Future Enhancements**:
- Apple Neural Engine (ANE) acceleration (CoreML)
- Optimize for M3 Max/Ultra (more GPU cores)

---

## Definition of Done

- [ ] Metal support implemented for macOS builds
- [ ] Apple Silicon detection working
- [ ] Transcription uses Metal on M1/M2/M3 Macs
- [ ] Fallback to CPU on Intel Macs
- [ ] Tests passing on Apple Silicon hardware
- [ ] Performance benchmarks documented (5-15x improvement)
- [ ] Documentation updated
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

# [ISSUE-004-3] OpenCL GPU Acceleration Support

**Created**: 2025-11-14
**Priority**: Moyenne
**Type**: Feature
**Status**: Todo
**Estimated Effort**: L
**EPIC**: EPIC 4 - Accélération Matérielle

---

## Description

Implement OpenCL GPU acceleration support for Whisper transcription on AMD and Intel GPUs. This provides GPU acceleration for users without NVIDIA or Apple GPUs.

---

## Context

Whisper.cpp supports OpenCL (CLBlast) acceleration for AMD and Intel GPUs. This allows users with AMD Radeon GPUs or Intel integrated graphics to benefit from GPU acceleration, though typically with less performance gain than CUDA or Metal.

---

## Acceptance Criteria

- [ ] OpenCL support can be enabled via configuration
- [ ] Application detects OpenCL-capable devices at startup
- [ ] Transcription uses OpenCL GPU when available
- [ ] Fallback to CPU if no OpenCL devices found
- [ ] Performance improvement of 2-5x for medium+ models
- [ ] Supports both AMD and Intel GPUs

---

## Technical Details

### Affected Components
- `src/gpu/opencl.rs` - New OpenCL integration module
- `src/transcription/whisper.rs` - Enable CLBlast backend
- `build.rs` - Add OpenCL compilation flags

### Dependencies
- [ ] Whisper.cpp compiled with CLBlast support
- [ ] OpenCL runtime installed (user system)
- [ ] CLBlast library

### Implementation Notes

**Build Configuration**:
- Add `opencl` feature flag to Cargo.toml
- Link against OpenCL and CLBlast libraries
- Cross-platform support (Windows, Linux)

**Device Detection**:
```rust
pub fn enumerate_opencl_devices() -> Vec<OpenCLDevice> {
    // Query OpenCL platforms and devices
}

pub fn is_opencl_available() -> bool {
    !enumerate_opencl_devices().is_empty()
}
```

**Performance Expectations**:
- AMD Radeon RX 6000/7000: 3-5x faster than CPU
- AMD Radeon RX 5000: 2-4x faster than CPU
- Intel Iris Xe: 2-3x faster than CPU
- Intel UHD: 1.5-2x faster than CPU

**Platforms**:
- **Windows**: AMD Radeon Software includes OpenCL
- **Linux**: `sudo apt install ocl-icd-opencl-dev clblast`
- **macOS**: OpenCL deprecated, use Metal instead

---

## Tasks Breakdown

- [ ] Add OpenCL/CLBlast feature flag to Cargo.toml
- [ ] Create `src/gpu/opencl.rs` module
- [ ] Implement OpenCL platform and device enumeration
- [ ] Implement device capability checking
- [ ] Add GPU configuration for OpenCL (device selection)
- [ ] Integrate CLBlast support in Whisper transcription
- [ ] Add error handling and CPU fallback
- [ ] Test on AMD GPU (Radeon RX 6000+)
- [ ] Test on Intel integrated GPU (Iris Xe)
- [ ] Document OpenCL setup for Windows/Linux

---

## Test Plan

### Unit Tests
- [ ] Test OpenCL device enumeration
- [ ] Test device capability validation
- [ ] Test graceful fallback when OpenCL unavailable

### Integration Tests
- [ ] Test transcription with AMD GPU
- [ ] Test transcription with Intel GPU
- [ ] Test CPU fallback when no GPU available
- [ ] Benchmark performance vs CPU

### Manual Testing
- [ ] Test on AMD Radeon RX 6700 XT or better
- [ ] Test on AMD Radeon RX 5700
- [ ] Test on Intel Iris Xe (12th gen+)
- [ ] Test on system without OpenCL (CPU fallback)
- [ ] Monitor GPU usage with GPU-Z or similar

---

## Documentation Updates

- [ ] Update README.md with OpenCL support information
- [ ] Update CLAUDE.md with OpenCL configuration
- [ ] Add OpenCL installation guide for Windows/Linux
- [ ] Document supported AMD/Intel GPUs
- [ ] Add troubleshooting for OpenCL issues

---

## Related Issues

- Related to: #004-1 (CUDA Support)
- Related to: #004-2 (Metal Support)
- Alternative to: CUDA/Metal for AMD/Intel users

---

## Notes

**OpenCL vs CUDA/Metal**:
- Generally slower than CUDA (NVIDIA) or Metal (Apple)
- Still provides meaningful speedup (2-5x) vs CPU
- Good fallback for non-NVIDIA/Apple hardware
- More compatibility issues than CUDA/Metal

**Supported GPUs**:
- **AMD**: RX 5000+ series recommended
- **Intel**: Iris Xe (11th gen+) recommended
- Older GPUs may have limited performance benefit

**Installation Complexity**:
- Windows: Usually works out-of-box with GPU drivers
- Linux: Requires manual installation of OpenCL runtime
- May require user to install CLBlast library

**Limitations**:
- macOS: OpenCL deprecated since macOS 10.14, use Metal instead
- Performance varies significantly by GPU model
- Some driver versions have bugs

**Future Enhancements**:
- Auto-detect best OpenCL device (if multiple)
- Allow manual device selection in UI
- Cache device selection to avoid re-enumeration

---

## Definition of Done

- [ ] OpenCL support implemented as optional feature
- [ ] Device detection and enumeration working
- [ ] Transcription uses OpenCL GPU when enabled
- [ ] Fallback to CPU implemented
- [ ] Tests passing on AMD and Intel GPUs
- [ ] Performance benchmarks documented (2-5x improvement)
- [ ] Documentation updated with setup instructions
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

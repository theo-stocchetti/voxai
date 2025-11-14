# [ISSUE-004-1] CUDA GPU Acceleration Support

**Created**: 2025-11-14
**Priority**: Haute
**Type**: Feature
**Status**: Todo
**Estimated Effort**: L
**EPIC**: EPIC 4 - Accélération Matérielle

---

## Description

Implement CUDA GPU acceleration support for Whisper transcription on NVIDIA GPUs. This will significantly improve transcription speed (3-10x faster) and reduce CPU usage, especially for larger models.

---

## Context

Whisper.cpp supports CUDA acceleration for NVIDIA GPUs. Many desktop users have NVIDIA GPUs that could accelerate transcription significantly. This is critical for real-time performance with medium/large models.

---

## Acceptance Criteria

- [ ] CUDA support can be enabled via configuration
- [ ] Application detects NVIDIA GPU availability at startup
- [ ] Transcription uses GPU when available and enabled
- [ ] Fallback to CPU if GPU unavailable or CUDA fails
- [ ] GPU usage is monitored and displayed (optional)
- [ ] Performance improvement of 3-10x for medium+ models

---

## Technical Details

### Affected Components
- `src/gpu/cuda.rs` - New CUDA integration module
- `src/transcription/whisper.rs` - Enable CUDA backend
- `src/config/schema.rs` - Add GPU configuration options
- `build.rs` - Add CUDA compilation flags

### Dependencies
- [ ] Whisper.cpp compiled with CUDA support
- [ ] CUDA Toolkit 11.0+ installed (user system)
- [ ] NVIDIA driver 450+ installed (user system)

### Implementation Notes

**Build Configuration**:
- Add `cuda` feature flag to Cargo.toml
- Whisper-rs must be built with CUBLAS support
- Link against CUDA libraries (cublas, cudart)

**Runtime Requirements**:
- Detect CUDA availability at startup
- Query GPU memory and compute capability
- Validate minimum CUDA version (11.0+)

**Performance Expectations**:
- Tiny model: 2-3x faster vs CPU
- Base model: 3-5x faster vs CPU
- Small model: 5-8x faster vs CPU
- Medium model: 7-10x faster vs CPU

**Code Example**:
```rust
// In Cargo.toml
[features]
cuda = ["whisper-rs/cuda"]

// In src/gpu/cuda.rs
pub fn is_cuda_available() -> bool {
    // Check for CUDA runtime
}

pub fn get_cuda_device_info() -> Option<CudaDeviceInfo> {
    // Query GPU properties
}

// In whisper.rs
let mut ctx = WhisperContext::new("model.bin")?;
if config.gpu.cuda_enabled && cuda::is_cuda_available() {
    ctx.enable_gpu(GpuBackend::Cuda)?;
}
```

---

## Tasks Breakdown

- [ ] Add CUDA feature flag to Cargo.toml
- [ ] Create `src/gpu/cuda.rs` module
- [ ] Implement CUDA availability detection
- [ ] Implement GPU device enumeration and info query
- [ ] Add GPU configuration fields (enabled, device_id)
- [ ] Integrate CUDA support in Whisper transcription
- [ ] Add error handling and fallback to CPU
- [ ] Implement GPU memory monitoring (optional)
- [ ] Add build script modifications for CUDA linking
- [ ] Test on NVIDIA GPU (GTX 1060+, RTX series)
- [ ] Document CUDA requirements and setup

---

## Test Plan

### Unit Tests
- [ ] Test CUDA availability detection
- [ ] Test GPU device enumeration
- [ ] Test graceful fallback to CPU when CUDA unavailable

### Integration Tests
- [ ] Test transcription with CUDA enabled (on NVIDIA GPU)
- [ ] Test transcription falls back to CPU without GPU
- [ ] Benchmark performance improvement vs CPU
- [ ] Test with different model sizes (tiny, base, small, medium)

### Manual Testing
- [ ] Test on RTX 3060/3070/3080/4060/4070/4080
- [ ] Test on older GTX 1060/1070/1080
- [ ] Verify error messages when CUDA unavailable
- [ ] Monitor GPU usage during transcription (nvidia-smi)

---

## Documentation Updates

- [ ] Update README.md with CUDA requirements
- [ ] Update CLAUDE.md with GPU configuration options
- [ ] Add CUDA installation guide for Windows/Linux
- [ ] Document performance benchmarks
- [ ] Add troubleshooting section for CUDA issues

---

## Related Issues

- Related to: #004-2 (Metal Support - macOS GPU)
- Related to: #004-3 (OpenCL Support - AMD/Intel GPU)
- Related to: #004-4 (CPU Optimizations - fallback performance)
- Blocks: #010-3 (Performance Benchmarks)

---

## Notes

**CUDA Installation**:
- Windows: Install CUDA Toolkit from NVIDIA website
- Linux: `sudo apt install nvidia-cuda-toolkit` or download from NVIDIA
- Requires compatible NVIDIA driver

**Minimum GPU Requirements**:
- Compute Capability 6.0+ (Pascal architecture or newer)
- 2GB+ VRAM for tiny/base models
- 4GB+ VRAM for small model
- 6GB+ VRAM for medium model

**Known Issues**:
- CUDA may fail on laptops with Optimus (hybrid graphics)
- Some WSL2 setups require additional configuration
- CUDA initialization adds ~500ms startup time

**Future Enhancements**:
- Multi-GPU support for parallel transcription
- Dynamic GPU memory allocation
- GPU utilization metrics in UI

---

## Definition of Done

- [ ] CUDA support implemented as optional feature
- [ ] GPU detection and initialization working
- [ ] Transcription uses GPU when enabled
- [ ] Fallback to CPU implemented
- [ ] Tests passing on NVIDIA GPU hardware
- [ ] Performance benchmarks documented (3-10x improvement)
- [ ] Documentation updated with requirements
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

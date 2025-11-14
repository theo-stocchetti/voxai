# [ISSUE-004-4] CPU Performance Optimizations

**Created**: 2025-11-14
**Priority**: Moyenne
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 4 - Accélération Matérielle

---

## Description

Implement CPU-specific performance optimizations for Whisper transcription to ensure good performance even without GPU acceleration. This includes SIMD optimizations, multi-threading, and CPU-specific instruction sets.

---

## Context

Not all users have GPUs, and GPU acceleration may fail or be unavailable. CPU performance should be optimized to provide acceptable transcription speed (2-5x realtime) even on modest hardware. Whisper.cpp supports various CPU optimizations.

---

## Acceptance Criteria

- [ ] AVX/AVX2 optimizations enabled for x86-64 CPUs
- [ ] ARM NEON optimizations enabled for ARM64 CPUs
- [ ] Multi-threading configured for optimal core usage
- [ ] CPU performance improved by 2-3x vs baseline
- [ ] Tiny/Base models run at 5-10x realtime on modern CPUs
- [ ] CPU usage stays under 50% for tiny/base models

---

## Technical Details

### Affected Components
- `build.rs` - Add CPU-specific compilation flags
- `src/transcription/whisper.rs` - Configure threading
- `Cargo.toml` - Add CPU optimization features

### Dependencies
- [ ] Whisper.cpp compiled with CPU optimizations

### Implementation Notes

**CPU Instruction Sets**:
- **x86-64**: AVX, AVX2, FMA (most Intel/AMD CPUs 2013+)
- **ARM64**: NEON (all ARM64 CPUs, including Apple Silicon)
- **WASM**: SIMD128 (for future web version)

**Threading Configuration**:
```rust
// Detect logical CPU count
let num_threads = num_cpus::get();

// Configure Whisper threading
let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
params.set_n_threads(num_threads.min(8)); // Cap at 8 threads for efficiency
```

**Build Flags**:
```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=native", "-C", "target-feature=+avx2,+fma"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "target-cpu=native"]
```

**Performance Targets**:
- Tiny model: 10-20x realtime (CPU-only)
- Base model: 7-15x realtime (CPU-only)
- Small model: 4-8x realtime (CPU-only)
- Medium model: 2-4x realtime (CPU-only, modern CPU)

---

## Tasks Breakdown

- [ ] Add CPU detection utility (AVX2, NEON support)
- [ ] Configure build flags for CPU optimizations
- [ ] Enable AVX2 for x86-64 builds
- [ ] Enable NEON for ARM64 builds
- [ ] Implement optimal thread count detection
- [ ] Configure Whisper threading parameters
- [ ] Add CPU feature detection at runtime
- [ ] Add warning if CPU missing required features
- [ ] Test on various CPUs (Intel, AMD, ARM)
- [ ] Benchmark performance across CPU types
- [ ] Document CPU requirements and recommendations

---

## Test Plan

### Unit Tests
- [ ] Test CPU feature detection (AVX2, NEON)
- [ ] Test thread count calculation
- [ ] Test threading configuration

### Integration Tests
- [ ] Benchmark transcription on Intel CPU (i5/i7/i9)
- [ ] Benchmark transcription on AMD CPU (Ryzen 5/7/9)
- [ ] Benchmark transcription on ARM CPU (M1/M2, Raspberry Pi)
- [ ] Test with different thread counts (1, 2, 4, 8, 16)
- [ ] Verify CPU usage stays within limits

### Manual Testing
- [ ] Test on older CPU without AVX2 (should still work)
- [ ] Test on high-core-count CPU (16+ cores)
- [ ] Test on low-power CPU (mobile, embedded)
- [ ] Monitor CPU usage and temperature

---

## Documentation Updates

- [ ] Update README.md with CPU requirements
- [ ] Update CLAUDE.md with CPU optimization details
- [ ] Document minimum/recommended CPU specs
- [ ] Add performance benchmarks for various CPUs
- [ ] Document threading configuration

---

## Related Issues

- Related to: #004-1 (CUDA - GPU fallback if CUDA fails)
- Related to: #004-2 (Metal - CPU fallback on Intel Macs)
- Related to: #004-3 (OpenCL - CPU fallback if no GPU)
- Related to: #010-3 (Performance Benchmarks)

---

## Notes

**Minimum CPU Requirements**:
- **x86-64**: SSE4.2+ (2008 or later CPUs)
- **ARM64**: Any ARM64 CPU with NEON
- **Recommended**: AVX2+ (Intel Haswell 2013+ or AMD Excavator 2015+)

**CPU Selection Guide**:
| Model Size | Min CPU | Recommended CPU |
|------------|---------|-----------------|
| Tiny | Any 2-core | 4-core 2.0 GHz+ |
| Base | 2-core 2.0 GHz | 4-core 2.5 GHz+ |
| Small | 4-core 2.5 GHz | 6-core 3.0 GHz+ |
| Medium | 6-core 3.0 GHz | 8-core 3.5 GHz+ or GPU |

**Threading Considerations**:
- More threads != better performance (diminishing returns after 8)
- Hyperthreading provides minimal benefit (~10%)
- Prefer performance cores over efficiency cores (Intel 12th+ gen)

**Known Issues**:
- Very old CPUs (pre-2010) may lack SSE4.2
- ARM CPUs vary widely in performance
- Windows on ARM has compatibility issues

**Future Enhancements**:
- AVX-512 support for newer Intel CPUs (Xeon, i9)
- Cache optimization for large models
- NUMA awareness for multi-socket systems

---

## Definition of Done

- [ ] CPU optimizations implemented (AVX2, NEON)
- [ ] Threading configuration optimized
- [ ] CPU feature detection working
- [ ] Performance improved 2-3x vs baseline
- [ ] Tests passing on Intel, AMD, and ARM CPUs
- [ ] Benchmarks documented for various CPU types
- [ ] Documentation updated with CPU requirements
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

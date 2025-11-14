# [ISSUE-009-3] Performance Indicators and Metrics

**Created**: 2025-11-14
**Priority**: Basse
**Type**: Feature
**Status**: Todo
**Estimated Effort**: M
**EPIC**: EPIC 9 - Indicateurs Visuels et Feedback

---

## Description

Add performance indicators to display transcription speed, CPU/GPU usage, latency, and other metrics. Useful for debugging and optimization.

---

## Context

Advanced users and developers benefit from performance metrics. This helps:
- Verify GPU acceleration is working
- Monitor system resource usage
- Identify performance bottlenecks
- Validate real-time performance (latency < 2s)

---

## Acceptance Criteria

- [ ] Display transcription speed (e.g., "10x realtime")
- [ ] Show CPU/GPU usage
- [ ] Show latency (time from speech end to text injection)
- [ ] Show current backend (CPU/CUDA/Metal/OpenCL)
- [ ] Accessible via settings or tray menu
- [ ] Optional display in overlay

---

## Technical Details

### Affected Components
- `src/ui/metrics.rs` - Metrics collection and display
- `src/ui/settings.rs` - Metrics viewer in settings

### Implementation Notes

**Metrics to Track**:
- Transcription speed (audio duration / processing time)
- CPU usage (%)
- GPU usage (% if available)
- Memory usage (MB)
- Latency (end-to-text time in seconds)
- Backend in use (CPU, CUDA, Metal, OpenCL)

**Code Sketch**:
```rust
pub struct PerformanceMetrics {
    transcription_speed: f32, // e.g., 10.5 (10.5x realtime)
    cpu_usage: f32,            // Percentage
    gpu_usage: Option<f32>,    // Percentage if GPU enabled
    latency_ms: u64,           // Milliseconds
    backend: Backend,          // CPU/CUDA/Metal/OpenCL
}
```

---

## Tasks Breakdown

- [ ] Implement metrics collection
- [ ] Track transcription speed
- [ ] Monitor CPU/GPU usage (using sysinfo crate)
- [ ] Measure latency
- [ ] Add metrics viewer to settings UI
- [ ] (Optional) Display in overlay

---

## Test Plan

### Manual Testing
- [ ] Run transcription and verify metrics update
- [ ] Verify GPU usage shown when GPU enabled
- [ ] Verify latency is accurate

---

## Documentation Updates

- [ ] Document performance metrics in CLAUDE.md

---

## Definition of Done

- [ ] Performance metrics implemented
- [ ] Metrics displayed in settings
- [ ] Code reviewed and merged
- [ ] Issue moved to done folder

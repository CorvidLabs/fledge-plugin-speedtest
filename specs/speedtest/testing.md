---
spec: speedtest.spec.md
---

## Test Plan

### Unit Tests

- Mbps conversion for zero, subsecond, and standard durations.
- Percentile empty, single, interpolation, sorting, and clamping behavior.

### Integration Tests

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`
- Verify help without performing a live network test.

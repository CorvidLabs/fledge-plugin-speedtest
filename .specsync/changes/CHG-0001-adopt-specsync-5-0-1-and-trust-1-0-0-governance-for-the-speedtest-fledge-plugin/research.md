---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-speedtest-fledge-plugin
artifact: research
---

# Research

The single Rust implementation has 22 deterministic tests for duration bounds, Mbps conversion, percentile interpolation/clamping, progress fractions, and JSON reports. Live network throughput is intentionally excluded from deterministic CI. Existing CI builds and tests Linux, macOS, and Windows and blocks Clippy and formatting.

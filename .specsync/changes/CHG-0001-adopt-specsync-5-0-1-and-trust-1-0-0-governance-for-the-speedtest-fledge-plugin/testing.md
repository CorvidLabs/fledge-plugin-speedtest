---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-speedtest-fledge-plugin
artifact: testing
---

# Testing

Local acceptance requires the five-step Fledge lane, all 22 deterministic tests, strict 100% coverage, four integrations, healthy Trust doctor, and a clean diff.

Hosted acceptance requires the new `trust` job plus existing Linux/macOS/Windows build-test matrix and Linux lint job. Live throughput remains an operator-run network check.

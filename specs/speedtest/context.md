---
spec: speedtest.spec.md
---

## Context

This Rust plugin offers a dependency-light Fledge command using public Cloudflare endpoints and deterministic local metric calculations.

## Related Modules

- Cloudflare download and upload speed endpoints.
- Fledge plugin command registration.

## Design Decisions

- Use a percentile rather than a single sample to reduce transient noise.
- Keep progress on standard error so JSON remains machine-readable.

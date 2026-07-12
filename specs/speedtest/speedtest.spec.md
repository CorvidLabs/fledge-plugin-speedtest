---
module: speedtest
version: 1
status: active
files:
  - src/main.rs

db_tables: []
depends_on: []
---

# Speedtest

## Purpose

Measure network latency, download throughput, and upload throughput against Cloudflare's speed-test endpoints with bounded duration and human or JSON output.

## Public API

| Surface | Behavior |
|---------|----------|
| default | Measure latency, download, and upload. |
| download | Measure latency and download only. |
| upload | Measure upload only. |
| duration | Bound each selected direction's measurement window to at least one second. |
| JSON | Emit a structured report without progress output contamination. |

## Invariants

1. Progress is emitted only to interactive standard error; report output remains clean.
2. Latency uses the best of three zero-byte probes.
3. Download and upload rotate through committed payload sizes until the duration window ends.
4. Truncated download responses are excluded from throughput samples.
5. Throughput uses decimal megabits per second.
6. The reported direction throughput is the linearly interpolated 90th percentile.
7. Empty sample sets report zero rather than failing or producing NaN.
8. HTTP requests use the configured timeout and rustls TLS.

## Behavioral Examples

```
Given network access to the configured Cloudflare endpoints
When the developer runs a selected direction for a bounded duration
Then the plugin reports sample counts and the computed latency or throughput in human or JSON form
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Client creation failure | TLS or client configuration cannot initialize | Surface the client error. |
| Latency failure | A probe cannot be sent or drained | Report latency context and exit non-zero. |
| Download failure | Requests fail within the measurement loop | Skip failed samples and report from completed full responses. |
| Upload failure | Requests fail within the measurement loop | Skip failed samples and report from completed samples. |
| Serialization failure | JSON report cannot be encoded | Surface the serialization error. |

## Dependencies

- Rust 1.89 or later
- Network access to Cloudflare speed-test endpoints
- `reqwest`, `clap`, `serde`, and `serde_json`

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing bounded latency and throughput measurement behavior for SpecSync 5 adoption. |

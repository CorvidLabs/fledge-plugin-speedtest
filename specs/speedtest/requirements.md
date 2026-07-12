---
spec: speedtest.spec.md
---

## User Stories

- As a developer, I want a quick terminal measurement of current network latency and throughput.

## Acceptance Criteria

### REQ-speedtest-001

The default command SHALL measure latency, download, and upload, while direction subcommands limit the selected work.

### REQ-speedtest-002

Each direction SHALL run for the bounded duration, rotate committed payload sizes, and record completed samples.

### REQ-speedtest-003

Truncated download responses and failed samples SHALL not inflate the throughput percentile.

### REQ-speedtest-004

Reports SHALL use decimal Mbps and a deterministic interpolated 90th percentile, returning zero for no samples.

### REQ-speedtest-005

Human and JSON outputs SHALL represent the same report while interactive progress remains on standard error.

## Constraints

- Results depend on network path, endpoint availability, and runtime load and are not a service-level guarantee.

## Out of Scope

- Server selection, historical storage, ISP diagnostics, and continuous monitoring.

## MODIFIED

### REQUIREMENT REQ-speedtest-001
The default command SHALL measure latency, download, and upload, while direction subcommands limit the selected work.

Acceptance Criteria
- Source inspection confirms the default, download-only, and upload-only dispatch branches select the documented measurements.

### REQUIREMENT REQ-speedtest-002
Each direction SHALL run for the bounded duration, rotate committed payload sizes, and record completed samples.

Acceptance Criteria
- Duration unit tests require a one-second minimum and preserve non-zero values.
- Source inspection confirms both direction loops rotate the committed size list and record only completed samples before the deadline.

### REQUIREMENT REQ-speedtest-003
Truncated download responses and failed samples SHALL not inflate the throughput percentile.

Acceptance Criteria
- Source inspection confirms downloads are sampled only when received bytes equal requested bytes and failed attempts add no sample.

### REQUIREMENT REQ-speedtest-004
Reports SHALL use decimal Mbps and a deterministic interpolated 90th percentile, returning zero for no samples.

Acceptance Criteria
- Unit tests cover decimal Mbps conversion, empty/single/unsorted inputs, clamping, p0/p50/p90/p100, and two-value interpolation.

### REQUIREMENT REQ-speedtest-005
Human and JSON outputs SHALL represent the same report while interactive progress remains on standard error.

Acceptance Criteria
- Report serialization tests cover full, partial, and empty JSON shapes.
- Source inspection confirms both renderers consume the same report and progress writes only to interactive standard error.

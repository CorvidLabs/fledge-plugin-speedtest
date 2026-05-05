# fledge-plugin-speedtest

Measure download and upload bandwidth via Cloudflare's public speed-test endpoints (`speed.cloudflare.com`).

A plugin for [fledge](https://github.com/CorvidLabs/fledge).

## Install

```bash
fledge plugins install CorvidLabs/fledge-plugin-speedtest
```

## Usage

```bash
fledge speedtest                  # latency, download, and upload
fledge speedtest download         # download only
fledge speedtest upload           # upload only
fledge speedtest --duration 15    # cap each direction's test duration (seconds)
fledge speedtest --json           # machine-readable JSON output
```

### Example output

```
  Latency          12.3 ms
  Download        842.5 Mbps
  Upload          421.7 Mbps
```

With `--json`:

```json
{"latency_ms":12.3,"download_mbps":842.5,"upload_mbps":421.7,"samples":{"download":9,"upload":7}}
```

## How it works

1. **Latency** -- three zero-byte requests to Cloudflare; the fastest round-trip is reported.
2. **Download** -- progressively larger payloads (100 KB to 100 MB) are fetched for the configured duration, measuring per-request throughput.
3. **Upload** -- payloads (100 KB to 10 MB) are POSTed for the configured duration.
4. **Result** -- the 90th-percentile sample is reported (Cloudflare's published methodology -- peak under stable conditions, not the mean which is dragged down by ramp-up). Truncated or errored transfers are discarded.

A brief warmup request runs before each direction to prime the connection pool.

## Development

```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## License

MIT

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

Throughput is reported as the 90th-percentile sample (Cloudflare's published methodology — peak under stable conditions, not the mean which is dragged down by ramp-up).

## License

MIT

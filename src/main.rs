use anyhow::{Context, Result};
use clap::Parser;
use reqwest::blocking::Client;
use serde::Serialize;
use std::io::{IsTerminal, Read, Write};
use std::time::{Duration, Instant};

const DOWN_URL: &str = "https://speed.cloudflare.com/__down";
const UP_URL: &str = "https://speed.cloudflare.com/__up";

/// Sizes (in bytes) used for each measurement direction. Smaller payloads
/// dominate when the link is slow; larger ones reveal steady-state throughput
/// on fast links. We rotate through the list, repeating until --duration is up.
/// Upload sizes are capped lower because Cloudflare's `__up` endpoint resets
/// the connection on very large bodies.
const DOWNLOAD_SIZES: &[u64] = &[
    100_000,
    1_000_000,
    10_000_000,
    25_000_000,
    100_000_000,
];

const UPLOAD_SIZES: &[u64] = &[
    100_000,
    1_000_000,
    10_000_000,
];

const WARMUP_BYTES: u64 = 100_000;
const DEFAULT_DURATION_SECS: u64 = 10;
const REQUEST_TIMEOUT_SECS: u64 = 60;
const PROGRESS_TICK: Duration = Duration::from_millis(100);
const BAR_WIDTH: usize = 24;

#[derive(Parser)]
#[command(
    name = "fledge-speedtest",
    version,
    about = "Measure download and upload bandwidth via Cloudflare"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Sub>,

    /// Cap each direction's measurement window (seconds)
    #[arg(short, long, default_value_t = DEFAULT_DURATION_SECS, global = true)]
    duration: u64,

    /// Emit machine-readable JSON instead of a human summary
    #[arg(long, global = true)]
    json: bool,
}

#[derive(clap::Subcommand)]
enum Sub {
    /// Measure download throughput only
    Download,
    /// Measure upload throughput only
    Upload,
}

#[derive(Serialize)]
struct Report {
    #[serde(skip_serializing_if = "Option::is_none")]
    latency_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    download_mbps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_mbps: Option<f64>,
    samples: SampleCounts,
}

#[derive(Serialize, Default)]
struct SampleCounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    download: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload: Option<usize>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()
        .context("building HTTP client")?;
    let window = Duration::from_secs(cli.duration.max(1));
    // Progress to stderr so JSON / piped human output stays clean.
    let show_progress = std::io::stderr().is_terminal();

    let (run_down, run_up) = match cli.command {
        Some(Sub::Download) => (true, false),
        Some(Sub::Upload) => (false, true),
        None => (true, true),
    };

    let mut report = Report {
        latency_ms: None,
        download_mbps: None,
        upload_mbps: None,
        samples: SampleCounts::default(),
    };

    if run_down {
        report.latency_ms =
            Some(measure_latency(&client, show_progress).context("measuring latency")?);
        let mut samples =
            measure_download(&client, window, show_progress).context("running download test")?;
        report.samples.download = Some(samples.len());
        report.download_mbps = Some(percentile(&mut samples, 0.9));
    }

    if run_up {
        let mut samples =
            measure_upload(&client, window, show_progress).context("running upload test")?;
        report.samples.upload = Some(samples.len());
        report.upload_mbps = Some(percentile(&mut samples, 0.9));
    }

    if cli.json {
        println!("{}", serde_json::to_string(&report)?);
    } else {
        print_human(&report);
    }
    Ok(())
}

fn print_human(r: &Report) {
    if let Some(ms) = r.latency_ms {
        println!("  Latency       {ms:>7.1} ms");
    }
    if let Some(mbps) = r.download_mbps {
        println!("  Download      {mbps:>7.1} Mbps");
    }
    if let Some(mbps) = r.upload_mbps {
        println!("  Upload        {mbps:>7.1} Mbps");
    }
}

fn measure_latency(client: &Client, show_progress: bool) -> Result<f64> {
    if show_progress {
        let mut err = std::io::stderr();
        let _ = write!(err, "  Latency       measuring");
        let _ = err.flush();
    }
    let mut best = f64::INFINITY;
    for _ in 0..3 {
        let start = Instant::now();
        let mut resp = client
            .get(format!("{DOWN_URL}?bytes=0"))
            .send()
            .context("latency probe")?;
        let mut sink = Vec::new();
        resp.read_to_end(&mut sink).context("draining latency probe")?;
        let ms = start.elapsed().as_secs_f64() * 1000.0;
        if ms < best {
            best = ms;
        }
        if show_progress {
            let mut err = std::io::stderr();
            let _ = write!(err, ".");
            let _ = err.flush();
        }
    }
    if show_progress {
        clear_line();
    }
    Ok(best)
}

fn measure_download(client: &Client, window: Duration, show_progress: bool) -> Result<Vec<f64>> {
    let _ = download_once(client, WARMUP_BYTES, |_, _| {});

    let mut samples = Vec::new();
    let start = Instant::now();
    let deadline = start + window;
    let mut idx = 0;
    let mut last_draw = Instant::now();
    if show_progress {
        draw_progress("Download", Duration::ZERO, window, 0.0);
    }
    while Instant::now() < deadline {
        let bytes = DOWNLOAD_SIZES[idx % DOWNLOAD_SIZES.len()];
        let result = download_once(client, bytes, |progress_bytes, req_elapsed| {
            if show_progress && last_draw.elapsed() >= PROGRESS_TICK {
                let inst = mbps(progress_bytes, req_elapsed);
                draw_progress("Download", start.elapsed().min(window), window, inst);
                last_draw = Instant::now();
            }
        });
        if let Ok(elapsed) = result {
            samples.push(mbps(bytes, elapsed));
        }
        idx += 1;
        if show_progress {
            let mut snapshot = samples.clone();
            let p90 = percentile(&mut snapshot, 0.9);
            draw_progress("Download", start.elapsed().min(window), window, p90);
            last_draw = Instant::now();
        }
    }
    if show_progress {
        clear_line();
    }
    Ok(samples)
}

fn download_once(
    client: &Client,
    bytes: u64,
    mut on_progress: impl FnMut(u64, Duration),
) -> Result<Duration> {
    let mut resp = client
        .get(format!("{DOWN_URL}?bytes={bytes}"))
        .send()
        .context("GET __down")?;
    let start = Instant::now();
    let mut buf = [0u8; 64 * 1024];
    let mut total = 0u64;
    loop {
        let n = resp.read(&mut buf).context("reading download body")?;
        if n == 0 {
            break;
        }
        total += n as u64;
        on_progress(total, start.elapsed());
    }
    Ok(start.elapsed())
}

fn measure_upload(client: &Client, window: Duration, show_progress: bool) -> Result<Vec<f64>> {
    let _ = upload_once(client, WARMUP_BYTES);

    let mut samples = Vec::new();
    let start = Instant::now();
    let deadline = start + window;
    let mut idx = 0;
    if show_progress {
        draw_progress("Upload", Duration::ZERO, window, 0.0);
    }
    while Instant::now() < deadline {
        let bytes = UPLOAD_SIZES[idx % UPLOAD_SIZES.len()];
        if show_progress {
            // Reqwest's blocking body is not streamed in a way we can hook
            // mid-flight, so the bar can't move during the request itself.
            // Surface the in-flight payload size so the user knows why the
            // bar pauses on slow links.
            draw_progress_label(
                "Upload",
                start.elapsed().min(window),
                window,
                &format!("uploading {:.1} MB", bytes as f64 / 1_000_000.0),
            );
        }
        if let Ok(elapsed) = upload_once(client, bytes) {
            samples.push(mbps(bytes, elapsed));
        }
        idx += 1;
        if show_progress {
            let mut snapshot = samples.clone();
            let p90 = percentile(&mut snapshot, 0.9);
            draw_progress("Upload", start.elapsed().min(window), window, p90);
        }
    }
    if show_progress {
        clear_line();
    }
    Ok(samples)
}

fn upload_once(client: &Client, bytes: u64) -> Result<Duration> {
    let payload: Vec<u8> = (0..bytes).map(|i| i as u8).collect();
    let start = Instant::now();
    let resp = client
        .post(UP_URL)
        .header("Content-Type", "application/octet-stream")
        .body(payload)
        .send()
        .context("POST __up")?;
    // Drain any response so the connection returns to the pool cleanly.
    let _ = resp.bytes();
    Ok(start.elapsed())
}

/// Throughput in megabits per second (decimal Mbps, not MiB/s).
fn mbps(bytes: u64, elapsed: Duration) -> f64 {
    let secs = elapsed.as_secs_f64();
    if secs <= 0.0 {
        return 0.0;
    }
    (bytes as f64 * 8.0) / 1_000_000.0 / secs
}

/// Linear-interpolated percentile (0.0..=1.0). Sorts `samples` in place.
fn percentile(samples: &mut [f64], p: f64) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p = p.clamp(0.0, 1.0);
    let rank = p * (samples.len() - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    if lo == hi {
        samples[lo]
    } else {
        let frac = rank - lo as f64;
        samples[lo] + (samples[hi] - samples[lo]) * frac
    }
}

fn draw_progress(label: &str, elapsed: Duration, window: Duration, mbps_value: f64) {
    draw_progress_label(label, elapsed, window, &format!("{mbps_value:>7.1} Mbps"));
}

fn draw_progress_label(label: &str, elapsed: Duration, window: Duration, trailing: &str) {
    let frac = (elapsed.as_secs_f64() / window.as_secs_f64().max(0.001)).min(1.0);
    let filled = (frac * BAR_WIDTH as f64).round() as usize;
    let filled = filled.min(BAR_WIDTH);
    let bar: String = "=".repeat(filled) + &" ".repeat(BAR_WIDTH - filled);
    let mut err = std::io::stderr();
    // Trailing spaces pad over residue from longer prior strings.
    let _ = write!(
        err,
        "\r  {label:<10} [{bar}] {:>4.1}s  {trailing}      ",
        elapsed.as_secs_f64(),
    );
    let _ = err.flush();
}

fn clear_line() {
    let mut err = std::io::stderr();
    let _ = write!(err, "\r\x1b[K");
    let _ = err.flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mbps_one_megabyte_per_second_is_eight_mbps() {
        let r = mbps(1_000_000, Duration::from_secs(1));
        assert!((r - 8.0).abs() < 1e-9, "got {r}");
    }

    #[test]
    fn mbps_handles_subsecond() {
        let r = mbps(10_000_000, Duration::from_millis(500));
        assert!((r - 160.0).abs() < 1e-9, "got {r}");
    }

    #[test]
    fn mbps_zero_elapsed_returns_zero() {
        assert_eq!(mbps(1_000_000, Duration::from_secs(0)), 0.0);
    }

    #[test]
    fn percentile_empty_is_zero() {
        let mut s: Vec<f64> = vec![];
        assert_eq!(percentile(&mut s, 0.9), 0.0);
    }

    #[test]
    fn percentile_single_value() {
        let mut s = vec![42.0];
        assert_eq!(percentile(&mut s, 0.9), 42.0);
    }

    #[test]
    fn percentile_p90_of_ten_evenly_spaced() {
        let mut s: Vec<f64> = (1..=10).map(|n| n as f64).collect();
        // rank = 0.9 * 9 = 8.1 → between idx 8 (value 9) and idx 9 (value 10)
        let r = percentile(&mut s, 0.9);
        assert!((r - 9.1).abs() < 1e-9, "got {r}");
    }

    #[test]
    fn percentile_unsorted_input_is_sorted() {
        let mut s = vec![5.0, 1.0, 3.0, 2.0, 4.0];
        let r = percentile(&mut s, 1.0);
        assert_eq!(r, 5.0);
    }

    #[test]
    fn percentile_p_clamped_to_unit_range() {
        let mut s = vec![1.0, 2.0, 3.0];
        assert_eq!(percentile(&mut s.clone(), -0.5), 1.0);
        assert_eq!(percentile(&mut s, 1.5), 3.0);
    }

    #[test]
    fn report_json_omits_unset_fields() {
        let r = Report {
            latency_ms: Some(12.5),
            download_mbps: Some(800.0),
            upload_mbps: None,
            samples: SampleCounts {
                download: Some(7),
                upload: None,
            },
        };
        let v: serde_json::Value = serde_json::to_value(&r).unwrap();
        assert_eq!(v["latency_ms"], 12.5);
        assert_eq!(v["download_mbps"], 800.0);
        assert!(v.get("upload_mbps").is_none());
        assert_eq!(v["samples"]["download"], 7);
        assert!(v["samples"].get("upload").is_none());
    }
}

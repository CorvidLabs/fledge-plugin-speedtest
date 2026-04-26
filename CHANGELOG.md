# Changelog

## 0.1.2

- Fix: discard truncated download responses so an early-closed connection cannot fabricate an absurd Mbps reading at the tail of a run.
- Fix: animate the upload progress bar via a side ticker thread so the bar advances during the request instead of freezing on slow links.

## 0.1.1

- Add live progress bar with rolling Mbps for download and upload phases.
- Stream download chunks so the bar advances during a single large request instead of freezing on slow links.
- Suppress progress output when stderr is not a TTY (clean piping for `--json` and redirected runs).

## 0.1.0

- Initial release

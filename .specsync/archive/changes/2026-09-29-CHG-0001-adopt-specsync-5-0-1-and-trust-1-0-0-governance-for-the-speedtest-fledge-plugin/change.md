---
id: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-speedtest-fledge-plugin
state: archived
type: migration
base_commit: fc904d508cefaecf6e69bda7416bb62d4912d352
---

# Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the Speedtest Fledge plugin

## Intent

Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the Speedtest Fledge plugin

## Affected Canonical Specs

- `speedtest`

## Acceptance Criteria

- SpecSync strict file and LOC coverage is a non-vacuous 100%.
- All five canonical requirements have concrete evidence and stable IDs.
- Claude, Cursor, Codex, and Gemini integrations are installed.
- Trust doctor and native verification pass.
- Formatting, Clippy, 22 tests, release build, and portable help smoke pass.
- Governance-only pull requests trigger the preserved Linux, macOS, and Windows CI matrix.

## No-spec Rationale

Not applicable

## Migration Note

Migrated by hand to SpecSync 6 per Leif's decision (2026-09-28); the 6.0.0 tool refused to archive this legacy record (`` exact-only delivery input `.github/workflows/trust.yml` changed after acceptance and requires an audited reopen; run `specsync change reopen CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-speedtest-fledge-plugin` to re-verify the accepted change, or supersede it from a later change under a module granted the path by `owns` in `.specsync/config.toml` ``).

- Workflow v1 (SpecSync 5) record, accepted on 2026-07-14 by the closing approval already stored in `approvals.json`. SpecSync 6.0.0 reports its accepted evidence as stale, for the reason quoted above.
- Moved by hand from `.specsync/changes/CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-speedtest-fledge-plugin/` into the layout `specsync change archive` writes: `accepted-state.json` is the unchanged accepted `state.json`, `state.json` is marked `archived`, and this file's front matter says `archived`.
- `approvals.json`, `verification.json` and every other artifact are the original SpecSync 5 evidence, unchanged. `verification.json` verifies commit `a4d6ce228dfe71a5d67eb68a2cef90704b24bb83`, not the tree this record was archived from.
- There is no `verification-attempts.json`: SpecSync 5 did not write one for this record, and this migration does not invent attempt history.
- This migration added no verification evidence, test result, attempt history, or approval. It is a manual migration, not a fresh re-verification.
- Closing it through the tool takes `specsync change reopen`, `specsync change verify`, then `specsync change accept`, which writes a new closing approval. Per Leif's decision it was archived by hand instead, so no reopen or new approval is recorded.

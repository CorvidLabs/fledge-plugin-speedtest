---
id: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-speedtest-fledge-plugin
state: implementing
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

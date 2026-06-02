# Pulse 02: Minimal neutral descriptor-output profile

## Goal

Implement the first generated artifact profile:
`rune.neutral_descriptor_json`.

## Engineering decision supported

RUNE generation starts with a neutral descriptor-output artifact that preserves
descriptor identity, version, kind, fields, invariants, trace links, and
extensions before targeting external schema or product formats.

## Trace links expected

- `docs/architecture/generator-profile-interface.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VALIDATION.md`

## Evidence produced

- `rune generate --profile rune.neutral_descriptor_json --fixture <path>`.
- Retained generated artifact fixture.
- Pass/fail CLI tests for generated artifact output, missing identity, and
  unsupported profile.

## Result

Complete. RUNE now emits a neutral descriptor-output artifact with profile
metadata and preserved descriptor content, while external formats and product
adapters remain out of scope.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\valid_descriptor.json
git diff --check
```

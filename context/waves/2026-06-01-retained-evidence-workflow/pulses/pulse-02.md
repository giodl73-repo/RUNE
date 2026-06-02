# Pulse 02: Evidence CLI and fixtures

## Goal

Expose read-only retained evidence bundle generation through the CLI.

## Engineering decision supported

`rune evidence-collection` is the standard opt-in evidence refresh surface for
collection and discovery inputs.

## Evidence produced

- `rune evidence-collection --profile rune.neutral_descriptor_json --fixture <path>`
- `rune evidence-collection --profile rune.neutral_descriptor_json --manifest <path>`
- `crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.evidence_bundle.json`
- `crates\rune-cli\tests\fixtures\adopter_discovered_collection.evidence_bundle.json`
- CLI pass/fail tests

## Result

Complete. The command emits evidence bundles to stdout and fails closed for
unknown profiles and malformed collection fixtures.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-cli --test evidence_cli
cargo test --workspace
git diff --check
```

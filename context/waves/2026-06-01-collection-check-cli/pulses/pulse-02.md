# Pulse 02: Collection check evidence

## Goal

Retain pass/fail evidence for collection-level profile compatibility checks.

## Engineering decision supported

The approved neutral profile can validate a known descriptor collection without
emitting generated artifacts.

## Evidence produced

- `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.check.json`
- `crates/rune-cli/tests/fixtures/other_kind_descriptor_collection.json`
- `crates/rune-cli/tests/check_cli.rs`

## Result

Complete. The CLI emits retained neutral collection compatibility evidence and
fails closed for malformed collections and profile-unsupported descriptor kinds.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

# Pulse 02: Collection generation evidence

## Goal

Retain pass/fail evidence for neutral collection generation.

## Engineering decision supported

The approved neutral profile can emit a collection artifact without external
profiles or product adapters.

## Evidence produced

- `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.neutral_descriptor_artifact.json`
- `crates/rune-cli/tests/generate_cli.rs`
- `crates/rune-core/src/lib.rs`

## Result

Complete. The CLI emits retained neutral collection artifact evidence and fails
closed for malformed collections, unknown profiles, and profile-unsupported
descriptor kinds.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

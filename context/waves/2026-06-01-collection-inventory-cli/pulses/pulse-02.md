# Pulse 02: Collection inventory evidence

## Goal

Retain pass/fail evidence for neutral collection inventory.

## Engineering decision supported

The known descriptor collection can be summarized into deterministic descriptor
counts and kind counts.

## Evidence produced

- `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.inventory.json`
- `crates/rune-cli/tests/inspect_cli.rs`
- `crates/rune-core/src/lib.rs`

## Result

Complete. The CLI emits retained neutral inventory evidence and fails closed for
malformed collections.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

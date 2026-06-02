# Pulse 02: Collection inspection CLI tests

## Goal

Add retained pass/fail evidence for collection inspection.

## Engineering decision supported

Collection inspection can be verified deterministically and fail closed on
malformed collection evidence.

## Evidence produced

- `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.inspection.json`
- `crates/rune-cli/tests/fixtures/missing_collection_id_descriptor_collection.json`
- `crates/rune-cli/tests/fixtures/missing_collection_version_descriptor_collection.json`
- `crates/rune-cli/tests/fixtures/duplicate_id_descriptor_collection.json`

## Result

Complete. The CLI emits retained neutral collection JSON and fails closed with
`RUNE-COLL-INSP-001`, `RUNE-COLL-INSP-002`, and `RUNE-COLL-INSP-003`.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

# Pulse 01: Collection draft validation

## Goal

Add a validation boundary for descriptor collection documents.

## Engineering decision supported

Collection evidence can be treated as durable only if malformed collection
metadata and duplicate descriptor ids fail closed.

## Evidence produced

- `rune_core::DescriptorCollectionDraft`
- `RUNE-COLL-001` missing collection identity diagnostic
- `RUNE-COLL-002` missing collection version diagnostic
- `RUNE-COLL-003` duplicate descriptor id diagnostic

## Result

Complete with limits. Validation is core-only and does not add CLI collection or
discovery.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

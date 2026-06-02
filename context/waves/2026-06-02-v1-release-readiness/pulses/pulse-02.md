# Pulse 02: Compatibility policy

## Goal

Record v1 compatibility expectations.

## Engineering decision supported

Descriptor, collection, profile, adapter, retained fixture, and diagnostic
compatibility expectations are explicit before release.

## Evidence produced

- `docs\release-readiness.md`

## Result

Complete. Breaking neutral descriptor changes require version changes and
retained evidence updates; profiles and adapters remain separate reviewed
surfaces.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
git diff --check
```

# Pulse 02: Collection document readiness

## Goal

Decide whether the known-contract collection evidence is durable enough for the
next interface-control gate.

## Decision

Pass for durable known-contract collection evidence. The retained collection now
preserves:

- `collection_id`,
- `collection_version`,
- ordered descriptor documents.

## Limits

This does not approve crate discovery, source inference, external profiles, CLI
collection, product adapters, or the valid-but-profile-unsupported `other`
escape hatch.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

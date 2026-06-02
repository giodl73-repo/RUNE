# Pulse 02: Collection validation readiness

## Goal

Decide whether collection document validation is ready as a prerequisite for
future collection input surfaces.

## Decision

Pass for core validation readiness. Descriptor collection drafts now validate
into durable collection documents and reject:

- missing collection identity,
- missing collection version,
- duplicate descriptor ids.

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

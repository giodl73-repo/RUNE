# Pulse 01: V1 derive ergonomics spec

## Goal

Document the v1 derive authoring surface and deferred ergonomics.

## Engineering decision supported

RUNE can make durable identity/version explicit while deferring richer authoring
features until their semantics and evidence are reviewed.

## Evidence produced

- `docs/architecture/derive-v1-ergonomics.md`
- `docs/architecture/interface-control.md`

## Result

Complete. Field metadata, enum variant descriptors, invariants, doc capture, and
adapter hints are explicitly deferred.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

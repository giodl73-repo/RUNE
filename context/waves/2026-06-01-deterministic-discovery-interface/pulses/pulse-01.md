# Pulse 01: Discovery model decision

## Goal

Choose the first deterministic discovery model.

## Engineering decision supported

Manifest-based discovery over retained descriptor collection fixtures is the
safest first discovery boundary.

## Evidence produced

- `docs/architecture/deterministic-discovery-interface.md`

## Result

Complete. The first implementation candidate is fixture-backed discovery
manifest parsing and deterministic collection merge.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

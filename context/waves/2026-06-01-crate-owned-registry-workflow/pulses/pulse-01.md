# Pulse 01: Crate-owned registry spec

## Goal

Document the adopter-owned explicit registry workflow.

## Engineering decision supported

Adopter crates can own deterministic registry slices before discovery exists.

## Evidence produced

- `docs/architecture/crate-owned-registry-workflow.md`

## Result

Complete. The workflow specifies annotation, explicit registration, stable
collection identity/version, retained evidence, and duplicate-id failure.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

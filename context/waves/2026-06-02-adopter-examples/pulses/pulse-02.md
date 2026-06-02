# Pulse 02: Adopter guide

## Goal

Document the current v1 adopter workflow.

## Engineering decision supported

Adopters need one documented path through derive, registry, retained evidence,
profiles, and adapters before broader repo bakeoffs.

## Evidence produced

- `docs\adopter-guide.md`
- `README.md`

## Result

Complete. The guide lists the example crate steps, commands, retained fixtures,
and boundaries.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

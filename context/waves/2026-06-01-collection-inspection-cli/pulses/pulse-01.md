# Pulse 01: Collection inspection interface

## Goal

Define the read-only CLI collection inspection surface.

## Engineering decision supported

Collection inspection is safe to expose only as fixture-backed validation and
neutral JSON output.

## Evidence produced

- `docs/architecture/inspection-surface.md`
- `rune inspect-collection --fixture <path>`

## Result

Complete with limits. The interface inspects collection fixtures only and does
not discover Rust crates.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

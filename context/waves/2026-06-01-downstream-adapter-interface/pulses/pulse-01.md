# Pulse 01: Adapter boundary

## Goal

Define the separation between RUNE core/profile output and downstream adapters.

## Engineering decision supported

Adapters consume validated RUNE evidence or external profile outputs, not raw
source or unreviewed crate metadata.

## Evidence produced

- `docs\architecture\downstream-adapter-interface.md`

## Result

Complete. Adapter vocabulary is barred from `rune-core`, derive attributes, and
neutral fixtures.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

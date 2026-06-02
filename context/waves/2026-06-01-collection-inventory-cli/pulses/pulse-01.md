# Pulse 01: Collection inventory interface

## Goal

Define the fixture-backed CLI collection inventory surface.

## Engineering decision supported

Collection inventory is safe to expose as read-only summarization over validated
collection documents.

## Evidence produced

- `docs/architecture/inspection-surface.md`
- `rune inventory-collection --fixture <path>`

## Result

Complete with limits. The command summarizes collection fixtures only and does
not discover Rust crates.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

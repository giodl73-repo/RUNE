# Pulse 01: Collection check interface

## Goal

Define the read-only CLI collection compatibility-check surface.

## Engineering decision supported

Collection compatibility checks are safe to expose as fixture-backed validation
against the approved profile catalog.

## Evidence produced

- `docs/architecture/generator-profile-interface.md`
- `rune check-collection --profile rune.neutral_descriptor_json --fixture <path>`

## Result

Complete with limits. The command checks collection fixtures only and does not
discover Rust crates.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

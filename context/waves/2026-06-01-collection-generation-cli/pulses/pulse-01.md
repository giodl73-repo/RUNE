# Pulse 01: Collection generation interface

## Goal

Define the fixture-backed CLI collection generation surface.

## Engineering decision supported

Collection generation is safe to expose after collection validation and
collection/profile compatibility checking are verified.

## Evidence produced

- `docs/architecture/generator-profile-interface.md`
- `rune generate-collection --profile rune.neutral_descriptor_json --fixture <path>`

## Result

Complete with limits. The command generates from collection fixtures only and
does not discover Rust crates.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

# Pulse 01: Profile boundary

## Goal

Define the boundary between neutral core descriptors and external profile output.

## Engineering decision supported

External profiles consume validated neutral documents and may use profile-specific
vocabulary only inside profile artifacts.

## Evidence produced

- `docs\architecture\external-profile-interface.md`

## Result

Complete. Product-specific terms are kept out of `rune-core`, derive attributes,
and neutral collection records.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

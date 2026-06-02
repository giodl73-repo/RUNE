# Pulse 02: Metadata and diagnostics

## Goal

Define the required metadata and diagnostics for future external profiles.

## Engineering decision supported

Every external profile must declare stable identity, version, supported inputs,
supported concepts, output artifact kind, and fail-closed diagnostics.

## Evidence produced

- Required profile metadata list
- Required diagnostic families
- First implementation candidate selection

## Result

Complete. The preferred first profile is a documentation packet profile because
it is reviewable, useful for AI consumers, and avoids adapter-specific terms.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

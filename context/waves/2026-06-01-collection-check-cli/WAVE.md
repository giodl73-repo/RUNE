# Wave: Collection compatibility check CLI

## Goal

Expose read-only fixture-backed profile compatibility checks for descriptor
collection documents.

## Engineering decision supported

RUNE can validate a multi-contract collection against an approved profile and
retain a compact compatibility report before adding discovery-backed collection.

## Scope

This wave may add `check-collection --profile <profile> --fixture <path>`,
retained output fixtures, and fail-closed diagnostics. It must not add arbitrary
crate discovery, source inference, generated profile output, external profiles,
or product-specific adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Collection check interface | complete |
| 02 | Collection check evidence | complete |
| 03 | Collection check readiness | complete |

## Decision

Wave 16 passes for fixture-backed collection compatibility checks.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

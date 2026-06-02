# Wave: Collection neutral generation CLI

## Goal

Emit retained neutral generated artifacts for validated descriptor collection
documents.

## Engineering decision supported

RUNE can generate a neutral collection artifact after collection inspection and
collection/profile compatibility checks are verified.

## Scope

This wave may add
`generate-collection --profile rune.neutral_descriptor_json --fixture <path>`,
retained output fixtures, and fail-closed diagnostics. It must not add arbitrary
crate discovery, source inference, external profiles, or product-specific
adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Collection generation interface | complete |
| 02 | Collection generation evidence | complete |
| 03 | Collection generation readiness | complete |

## Decision

Wave 17 passes for fixture-backed neutral collection generation.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

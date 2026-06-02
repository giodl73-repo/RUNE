# Wave: Collection inspection CLI

## Goal

Expose read-only fixture-backed inspection for validated descriptor collection
documents.

## Engineering decision supported

RUNE can inspect retained multi-contract evidence through the CLI after the
collection document and validation boundary are in place, without adding
discovery or generation.

## Scope

This wave may add a read-only `inspect-collection --fixture <path>` command,
retained output fixtures, and fail-closed CLI diagnostics. It must not add
arbitrary crate discovery, source inference, generated profile output, or
product-specific adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Collection inspection interface | complete |
| 02 | Collection inspection CLI tests | complete |
| 03 | Collection inspection readiness | complete |

## Decision

Wave 15 passes for fixture-backed collection inspection. The CLI can validate
and emit neutral descriptor collection documents.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

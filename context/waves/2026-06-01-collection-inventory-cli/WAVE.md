# Wave: Collection inventory CLI

## Goal

Emit retained neutral inventory summaries for validated descriptor collection
documents.

## Engineering decision supported

RUNE can summarize collection coverage by descriptor kind without requiring
crate discovery, external profiles, or downstream adapters.

## Scope

This wave may add `inventory-collection --fixture <path>`, retained output
fixtures, and fail-closed diagnostics. It must not add arbitrary crate discovery,
source inference, external profiles, or product-specific adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Collection inventory interface | complete |
| 02 | Collection inventory evidence | complete |
| 03 | Collection inventory readiness | complete |

## Decision

Wave 18 passes for fixture-backed neutral collection inventory.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

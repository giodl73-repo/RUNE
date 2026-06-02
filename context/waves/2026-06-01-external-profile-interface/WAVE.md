# Wave: External profile interface

## Goal

Define how external profiles are added without changing the neutral core model.

## Engineering decision supported

External profiles are reviewed mappings over validated RUNE descriptor or
collection documents. Profile-specific vocabulary belongs below the generated
artifact boundary, not in `rune-core` or derive attributes.

## Scope

This wave may add an external profile interface-control document and VTRACE
records. It must not implement an external profile, adapter, source scraper,
Cargo metadata traversal, or executable hook.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Profile boundary | complete |
| 02 | Metadata and diagnostics | complete |
| 03 | Interface readiness | complete |

## Decision

Wave 25 passes for external profile interface-control. The first external
profile implementation remains gated to a later wave.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

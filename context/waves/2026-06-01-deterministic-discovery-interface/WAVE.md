# Wave: Deterministic discovery interface

## Goal

Specify deterministic discovery before implementation.

## Engineering decision supported

RUNE discovery should start with explicit manifest-based discovery over retained
descriptor collection fixtures, not source scraping or Cargo metadata traversal.

## Scope

This wave may add a discovery interface-control document and VTRACE records. It
must not implement discovery, execute registry hooks, scan source, traverse Cargo
metadata, add external profiles, or add adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Discovery model decision | complete |
| 02 | Discovery diagnostics and ordering | complete |
| 03 | Discovery interface readiness | complete |

## Decision

Wave 22 passes for deterministic discovery interface-control. Implementation is
gated to a later wave.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

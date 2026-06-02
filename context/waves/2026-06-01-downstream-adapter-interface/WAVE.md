# Wave: Downstream adapter interface

## Goal

Define downstream adapter boundaries before implementation.

## Engineering decision supported

Adapters are deterministic mappings from validated RUNE evidence or external
profile outputs into consumer artifacts. Adapter vocabulary stays outside the
neutral core.

## Scope

This wave may add an adapter interface-control document and VTRACE records. It
must not implement adapters, source scraping, Cargo metadata traversal,
executable hooks, or product-specific core fields.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Adapter boundary | complete |
| 02 | Adapter metadata and diagnostics | complete |
| 03 | Adapter readiness | complete |

## Decision

Wave 27 passes for adapter interface-control. First adapter implementation
remains gated to a later wave.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

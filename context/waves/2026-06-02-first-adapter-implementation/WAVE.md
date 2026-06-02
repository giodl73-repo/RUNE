# Wave: First adapter implementation

## Goal

Implement the first downstream adapter as a separate adapter surface.

## Engineering decision supported

RUNE can convert validated descriptor collection evidence into deterministic
review packets without adding adapter vocabulary to the neutral core.

## Scope

This wave may add a `rune-adapters` crate, adapter catalog, review packet
adapter, CLI adapter commands, retained fixtures, tests, and VTRACE records. It
must not add product-specific adapter vocabulary to `rune-core`, source scraping,
Cargo metadata traversal, executable hooks, or automatic publication.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Adapter crate and model | complete |
| 02 | CLI adapter evidence | complete |
| 03 | Adapter VTRACE closure | complete |

## Decision

Wave 28 passes for first generic adapter implementation. CLI hardening remains
the next gated v1 area.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

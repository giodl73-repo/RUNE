# Wave: V1 scope and documentation reconciliation

## Goal

Record the expanded RUNE v1 completion boundary and reconcile product-facing docs
with the implemented CLI surface.

## Engineering decision supported

RUNE v1 includes deterministic discovery, external profiles, and downstream
adapters, but keeps `rune-core` and the base descriptor model product-neutral.

## Scope

This wave may update README, product plan, VTRACE records, and wave records. It
must not implement discovery, external profiles, or adapters yet.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | V1 boundary decision | complete |
| 02 | Documentation reconciliation | complete |
| 03 | V1 scope readiness | complete |

## Decision

Wave 19 passes for scope and documentation reconciliation. Implementation of
discovery, external profiles, and adapters remains gated by later interface
waves.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

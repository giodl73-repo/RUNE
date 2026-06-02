# Wave: Crate-owned registry workflow

## Goal

Demonstrate how an adopter crate owns an explicit RUNE contract registry and
retained collection evidence.

## Engineering decision supported

RUNE can move beyond RUNE-internal fixtures by letting adopter crates expose
deterministic registries without adding discovery or adapters.

## Scope

This wave may add an example adopter crate, retained collection fixture, tests,
and registry workflow documentation. It must not add crate scanning, Cargo
metadata discovery, external profiles, or downstream adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Crate-owned registry spec | complete |
| 02 | Adopter registry example | complete |
| 03 | Registry workflow readiness | complete |

## Decision

Wave 21 passes for crate-owned explicit registry workflow evidence.

## Validation

```powershell
cargo test -p rune-adopter
cargo fmt --check
cargo test --workspace
git diff --check
```

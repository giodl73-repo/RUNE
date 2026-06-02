# Wave: Deterministic discovery implementation

## Goal

Implement the approved deterministic discovery boundary.

## Engineering decision supported

RUNE can merge retained descriptor collection fixtures through an explicit
manifest while preserving deterministic order and fail-closed diagnostics.

## Scope

This wave may add discovery manifest models, deterministic collection merge
logic, `rune discover --manifest <path>`, retained discovery fixtures, CLI tests,
and VTRACE records. It must not add Rust source scraping, Cargo metadata
traversal, executable registry hooks, external profiles, adapters, or downstream
product vocabulary.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Manifest model and merge | complete |
| 02 | Discovery CLI and evidence | complete |
| 03 | Diagnostics and VTRACE closure | complete |

## Decision

Wave 23 passes for deterministic manifest discovery over retained descriptor
collection fixtures. Arbitrary crate discovery and downstream adapters remain
deferred.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

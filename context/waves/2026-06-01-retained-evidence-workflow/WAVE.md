# Wave: Retained evidence workflow

## Goal

Provide a standard read-only workflow for regenerating retained collection
evidence.

## Engineering decision supported

RUNE can produce a complete neutral collection evidence bundle from retained
collection fixtures or approved discovery manifests without mutating files during
normal validation.

## Scope

This wave may add a collection evidence bundle model, an
`evidence-collection` CLI command, retained evidence bundle fixtures, tests,
documentation, and VTRACE records. It must not add external profiles, adapters,
source scraping, Cargo metadata traversal, or executable registry hooks.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Evidence bundle model | complete |
| 02 | Evidence CLI and fixtures | complete |
| 03 | Workflow docs and VTRACE | complete |

## Decision

Wave 24 passes for read-only retained evidence bundle generation. External
profile interface-control remains the next gated v1 area.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

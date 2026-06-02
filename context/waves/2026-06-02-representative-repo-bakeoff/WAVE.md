# Wave: Representative repo bakeoff

## Goal

Validate RUNE usefulness against a non-RUNE Rust repo scenario.

## Engineering decision supported

RUNE evidence improves AI/reviewer inspection compared with source-only review
for selected contracts in a representative Rust workspace.

## Scope

This wave may inspect a non-RUNE Rust repo as a scenario, create retained RUNE
bakeoff fixtures inside RUNE, and update VTRACE records. It must not modify the
scenario repo, add it as a dependency, or add product-specific vocabulary to
RUNE core.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Scenario selection | complete |
| 02 | RUNE evidence comparison | complete |
| 03 | Bakeoff VTRACE closure | complete |

## Decision

Wave 31 passes for representative repo bakeoff evidence. V1 release readiness
remains the final gated area.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

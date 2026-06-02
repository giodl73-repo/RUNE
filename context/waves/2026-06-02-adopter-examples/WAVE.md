# Wave: Adopter examples

## Goal

Provide adopter-facing v1 examples and guide documentation.

## Engineering decision supported

A real adopter can follow an explicit, deterministic path from derive metadata
through retained collection evidence, external profile output, and adapter
output without source scraping or product-specific core vocabulary.

## Scope

This wave may extend `examples\rune-adopter`, add adopter retained fixtures,
create adopter guide docs, and update VTRACE records. It must not add arbitrary
crate discovery, source scraping, Cargo traversal, executable hooks, or
product-specific core fields.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Adopter v1 fixtures | complete |
| 02 | Adopter guide | complete |
| 03 | Example VTRACE closure | complete |

## Decision

Wave 30 passes for adopter examples and guide documentation. Representative repo
bakeoff remains the next gated v1 area.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

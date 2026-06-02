# Wave: Source contract coverage

## Goal

Add retained known-contract evidence for a neutral source descriptor through the
explicit registry boundary.

## Engineering decision supported

RUNE should prove source-descriptor evidence with the same derive, check,
generate, and collection path used by earlier neutral descriptor-kind scenarios.

## Scope

This wave may add an explicitly registered annotated source contract, retained
descriptor evidence, compatibility evidence, neutral generated artifact evidence,
and collection fixture updates. It must not add arbitrary crate discovery, CLI
collection, external profiles, or product-specific adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Source contract scenario | complete |
| 02 | Source coverage readiness | complete |

## Decision

Wave 11 passes for bounded source descriptor evidence. RUNE now has retained
scenario evidence for entity, command, event, state, artifact, and source
descriptor kinds.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

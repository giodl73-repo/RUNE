# Wave: State contract coverage

## Goal

Add retained known-contract evidence for a neutral state descriptor through the
explicit registry boundary.

## Engineering decision supported

RUNE should prove state-descriptor evidence with the same derive, check,
generate, and collection path used by entity, command, and event scenarios before
expanding collection surfaces.

## Scope

This wave may add an explicitly registered annotated state contract, retained
descriptor evidence, compatibility evidence, neutral generated artifact evidence,
and collection fixture updates. It must not add arbitrary crate discovery, CLI
collection, external profiles, or product-specific adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | State contract scenario | complete |
| 02 | State coverage readiness | complete |

## Decision

Wave 9 passes for bounded state descriptor evidence. RUNE now has retained
scenario evidence for entity, command, event, and state descriptor kinds.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

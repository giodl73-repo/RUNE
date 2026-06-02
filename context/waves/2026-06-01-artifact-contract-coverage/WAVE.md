# Wave: Artifact contract coverage

## Goal

Add retained known-contract evidence for a neutral artifact descriptor through
the explicit registry boundary.

## Engineering decision supported

RUNE should prove artifact-descriptor evidence with the same derive, check,
generate, and collection path used by earlier neutral descriptor-kind scenarios.

## Scope

This wave may add an explicitly registered annotated artifact contract, retained
descriptor evidence, compatibility evidence, neutral generated artifact evidence,
and collection fixture updates. It must not add arbitrary crate discovery, CLI
collection, external profiles, or product-specific adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Artifact contract scenario | complete |
| 02 | Artifact coverage readiness | complete |

## Decision

Wave 10 passes for bounded artifact descriptor evidence. RUNE now has retained
scenario evidence for entity, command, event, state, and artifact descriptor
kinds.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

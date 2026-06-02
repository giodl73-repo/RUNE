# Wave: Evidence contract coverage

## Goal

Add retained known-contract evidence for a neutral evidence descriptor through
the explicit registry boundary.

## Engineering decision supported

RUNE should prove evidence-descriptor coverage with the same derive, check,
generate, and collection path used by all other neutral profile-supported
descriptor kinds.

## Scope

This wave may add an explicitly registered annotated evidence contract, retained
descriptor evidence, compatibility evidence, neutral generated artifact evidence,
and collection fixture updates. It must not add arbitrary crate discovery, CLI
collection, external profiles, or product-specific adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Evidence contract scenario | complete |
| 02 | Evidence coverage readiness | complete |

## Decision

Wave 12 passes for bounded evidence descriptor evidence. RUNE now has retained
scenario evidence for all neutral descriptor kinds currently supported by the
approved neutral profile: entity, command, event, state, artifact, source, and
evidence.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

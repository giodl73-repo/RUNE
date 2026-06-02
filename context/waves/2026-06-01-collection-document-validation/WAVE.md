# Wave: Collection document validation

## Goal

Add fail-closed validation for descriptor collection documents before any future
CLI collection or discovery input surface exists.

## Engineering decision supported

RUNE collection evidence must reject missing collection identity, missing
collection version, and duplicate descriptor ids before broader tooling consumes
collection documents.

## Scope

This wave may add a collection draft model, validation diagnostics, and core unit
tests. It must not add arbitrary crate discovery, source inference, CLI
collection, external profiles, or product-specific adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Collection draft validation | complete |
| 02 | Collection validation readiness | complete |

## Decision

Wave 14 passes for fail-closed collection document validation. Collection drafts
now reject missing collection identity, missing collection version, and duplicate
descriptor ids.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

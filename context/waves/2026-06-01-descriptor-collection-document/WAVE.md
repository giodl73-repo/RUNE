# Wave: Descriptor collection document

## Goal

Give retained multi-contract evidence a durable collection identity and version
without adding discovery or CLI collection.

## Engineering decision supported

Known-contract collection evidence should be independently identifiable before
it becomes an input to broader tooling, review gates, or future collection
commands.

## Scope

This wave may add a neutral collection document model, update the explicit
registry collection helper path, and retain collection evidence with collection
metadata. It must not add arbitrary crate discovery, source inference, CLI
collection, external profiles, or product adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Collection document envelope | complete |
| 02 | Collection document readiness | complete |

## Decision

Wave 13 passes for durable known-contract collection evidence. The retained
collection now has `collection_id`, `collection_version`, and ordered descriptor
documents.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

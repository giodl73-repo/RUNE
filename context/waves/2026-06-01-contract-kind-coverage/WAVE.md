# Wave: Contract kind coverage

## Goal

Expand known-contract scenario evidence from entity plus command to entity,
command, and event contracts through the explicit registry boundary.

## Engineering decision supported

RUNE descriptor evidence should cover multiple neutral contract kinds before any
repo-scale collection or downstream adapter work is approved.

## Scope

This wave may add explicitly registered annotated contracts, retained descriptor
fixtures, compatibility reports, neutral generated artifacts, and collection
fixture updates. It must not add arbitrary crate discovery, CLI collection,
external profiles, or product-specific adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Event contract scenario | complete |
| 02 | Contract-kind coverage readiness | complete |

## Decision

Wave 8 passes for scenario coverage across entity, command, and event descriptor
kinds. Broader discovery and downstream adapter work remain blocked.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

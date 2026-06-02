# Wave: Explicit registry interface

## Goal

Turn the Wave 6 test-owned known-contract collection into a reviewed neutral
explicit registry interface without approving crate discovery or product
adapters.

## Engineering decision supported

RUNE can move from ad hoc known-contract arrays toward controlled collection only
after the registry boundary, ordering rules, and diagnostics are owned by
`rune-core`.

## Scope

This wave may define registry interface docs, add a core explicit registration
record, add deterministic collection helpers, and update the known-contract
tests to use the core boundary. It must not add arbitrary crate scanning, Cargo
workspace discovery, CLI collection commands, external profiles, or downstream
product adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Registry interface control | complete |
| 02 | Core registry collection helper | complete |
| 03 | Registry readiness decision | complete |

## Decision

Wave 7 passes for an explicit known-contract registry interface. RUNE now owns a
neutral core registration record and ordered collection helper that can be used
by controlled evidence paths.

The wave does not approve crate discovery, CLI collection commands, external
profiles, or product adapters. Any future discovery surface must enter through a
new interface-control wave.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

# Wave: Derive V1 ergonomics

## Goal

Harden the derive macro for durable v1 evidence and record which authoring
ergonomics are supported or deferred.

## Engineering decision supported

RUNE v1 derive output must require explicit descriptor identity and version while
keeping richer field, enum, invariant, docs, and adapter hints behind later
reviewed surfaces.

## Scope

This wave may update derive diagnostics, compile-fail tests, and ergonomics
documentation. It must not add product-specific macro vocabulary, source
inference, adapter hints, or unreviewed field metadata.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | V1 derive ergonomics spec | complete |
| 02 | Required identity/version diagnostics | complete |
| 03 | Derive ergonomics readiness | complete |

## Decision

Wave 20 passes for v1 derive ergonomics hardening. Missing `id` or `version` is a
compile-time error.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

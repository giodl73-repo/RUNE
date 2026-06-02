# Wave: CLI hardening

## Goal

Add targeted CLI hardening coverage before v1 release readiness.

## Engineering decision supported

The CLI should fail clearly for invalid command shapes and report current v1
surfaces accurately.

## Scope

This wave may update status text, usage text, malformed JSON fixtures, CLI
hardening tests, docs, and VTRACE records. It must not add new discovery,
profile, adapter, source scraping, Cargo traversal, or executable hook behavior.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Status and usage text | complete |
| 02 | Hardening regression tests | complete |
| 03 | CLI hardening VTRACE closure | complete |

## Decision

Wave 29 passes for CLI hardening coverage. Adopter examples and docs remain the
next gated v1 area.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

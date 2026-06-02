# Wave: V1 release readiness

## Goal

Close RUNE v1 release readiness.

## Engineering decision supported

RUNE v1 has CI-ready validation, documented crate surfaces, compatibility
policy, retained evidence, and explicit non-goals.

## Scope

This wave may add release readiness docs, update README/product/VTRACE records,
and run final validation. It must not add new feature surfaces after the release
gate.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Release readiness docs | complete |
| 02 | Compatibility policy | complete |
| 03 | Final validation closure | complete |

## Decision

Wave 32 closes RUNE v1 release readiness.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
git diff --check
```

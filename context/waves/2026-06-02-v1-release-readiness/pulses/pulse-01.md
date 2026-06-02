# Pulse 01: Release readiness docs

## Goal

Document the v1 release-ready crate surfaces and validation gate.

## Engineering decision supported

RUNE has a release-ready surface that can be validated with CI-ready commands.

## Evidence produced

- `docs\release-readiness.md`
- `README.md`

## Result

Complete. Release readiness docs list crate surfaces, validation commands,
retained evidence, and v1 non-goals.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
git diff --check
```

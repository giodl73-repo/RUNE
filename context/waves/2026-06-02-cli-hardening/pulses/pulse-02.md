# Pulse 02: Hardening regression tests

## Goal

Retain regression tests for common CLI failure modes.

## Engineering decision supported

CLI hardening should be executable evidence, not just documentation.

## Evidence produced

- `crates\rune-cli\tests\cli_hardening.rs`
- `crates\rune-cli\tests\fixtures\malformed_descriptor.json`

## Result

Complete. Tests cover current status text, unknown command dispatch, missing
fixture flag usage, malformed JSON diagnostics, invalid adapter argument order,
and adapter subcommand usage.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-cli --test cli_hardening
cargo test --workspace
git diff --check
```

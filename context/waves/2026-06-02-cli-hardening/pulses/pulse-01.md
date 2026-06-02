# Pulse 01: Status and usage text

## Goal

Align CLI status and usage text with the current v1 surface.

## Engineering decision supported

Operators should see current crates, profiles, adapters, commands, and deferred
surfaces from the CLI.

## Evidence produced

- `crates\rune-cli\src\main.rs`

## Result

Complete. Status text now names `rune-adapters`, approved profiles, approved
adapters, and current deferred surfaces.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-cli --test cli_hardening
cargo test --workspace
git diff --check
```

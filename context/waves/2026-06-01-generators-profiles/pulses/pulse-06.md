# Pulse 06: Read-only profile catalog

## Goal

Make approved RUNE generator profiles discoverable without adding external
format generators or product adapters.

## Engineering decision supported

RUNE should expose a read-only profile catalog before expanding profile count, so
automation and reviewers can distinguish approved profile support from deferred
or unsupported targets.

## Trace links expected

- `docs/architecture/generator-profile-interface.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `crates/rune-cli/src/main.rs`
- `crates/rune-cli/tests/profile_cli.rs`
- `crates/rune-cli/tests/fixtures/profile_list.json`

## Evidence produced

- `rune profile list` CLI command.
- Retained expected profile catalog JSON.
- CLI tests for catalog output and unknown profile subcommands.

## Result

Complete with limits. The catalog lists only
`rune.neutral_descriptor_json@v0`. External formats remain unimplemented and
unlisted.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- profile list
git diff --check
```

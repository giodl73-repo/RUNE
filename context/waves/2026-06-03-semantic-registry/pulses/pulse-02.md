# Pulse 02: Read-only registry CLI checks

## Goal

Expose the first semantic registry CLI validation command without adding runtime
host behavior, mutation, Cargo traversal, source scraping, or plugin discovery.

## Changes

- Add `rune check-registry --fixture <path>`.
- Emit a compact registry check report over validated retained registry metadata.
- Cross-check declared profile and adapter references against approved catalogs.
- Fail closed on duplicate collection refs, runtime capability declarations,
  unknown profile refs, and invalid usage.
- Update README, product plan, DCR, requirements, trace, validation,
  verification, review, and Wave 42 records.

## Outcome

RUNE can now validate retained semantic registry fixtures from the CLI while
keeping collection source-ref loading, runtime host behavior, and mutation
blocked.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -q -p rune-cli -- status`
- `cargo run -q -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json`
- `git diff --check`

## Status

Complete.


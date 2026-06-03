# Pulse 04: Registry inspection reports

## Goal

Expose richer read-only semantic registry inspection after the same fail-closed
validation used by registry checks.

## Changes

- Add `rune inspect-registry --fixture <path>`.
- Reuse registry identity, version, scope, runtime capability, approved
  profile/adapter catalog, and retained collection source-ref validation.
- Emit validated registry metadata plus retained collection source refs, owners,
  descriptor counts, and kind inventories.
- Add retained inspection fixture and CLI tests.
- Update README, product plan, DCR, requirements, trace, validation,
  verification, review, and Wave 42 records.

## Outcome

RUNE can now inspect retained semantic registries as analyzable evidence while
keeping runtime host behavior, mutation, source scraping, Cargo traversal, and
plugin discovery blocked.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -q -p rune-cli -- status`
- `cargo run -q -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json`
- `cargo run -q -p rune-cli -- inspect-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json`
- `git diff --check`

## Status

Complete.


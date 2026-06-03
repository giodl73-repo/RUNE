# Pulse 01: Retained registry model

## Goal

Add the first semantic registry implementation slice without adding runtime host
or agent protocol behavior.

## Changes

- Add `SemanticRegistryDraft`, `SemanticRegistryDocument`, collection/profile/
  adapter reference records, and capability flags to `rune-core`.
- Add fail-closed validation for missing identity, missing version, duplicate
  collection refs, unsupported scopes, and blocked runtime capability.
- Add retained semantic registry fixtures under `crates\rune-cli\tests\fixtures`.
- Update DCR, requirements, trace, verification, validation, review, architecture,
  and product plan records.

## Outcome

RUNE can now retain and validate semantic registry metadata for crates and
workspaces while keeping runtime host, live state inspection, mutation, source
scraping, and Cargo traversal blocked.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -q -p rune-cli -- status`
- `git diff --check`

## Status

Complete.


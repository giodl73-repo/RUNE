# Pulse 03: Registry source-ref checks

## Goal

Validate retained descriptor collection source refs named by semantic registry
documents without adding Cargo traversal, source scraping, plugin discovery,
runtime host behavior, or mutation.

## Changes

- Extend `rune check-registry --fixture <path>` to load collection source refs
  relative to the retained registry fixture.
- Validate loaded descriptor collection fixtures using existing collection
  validation.
- Fail closed with `RUNE-REGISTRY-005` when a collection source ref is malformed,
  unreadable, or has a mismatched collection id/version.
- Add retained source-ref fixtures and CLI tests.
- Update README, product plan, DCR, requirements, trace, validation,
  verification, review, and Wave 42 records.

## Outcome

Semantic registries now prove that declared collection refs point to retained
descriptor collection evidence, while runtime behavior and source discovery stay
blocked.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -q -p rune-cli -- status`
- `cargo run -q -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json`
- `git diff --check`

## Status

Complete.


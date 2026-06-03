# Pulse 01: Retained state graph validation

## Intent

Add the first retained state graph evidence slice for Mission 2.0.

## Changes

- Added `StateGraphDraft`, `StateGraphDocument`, registry refs, nodes,
  transitions, ownership, evidence refs, capabilities, and validation codes.
- Added retained pass/failure state graph fixtures.
- Added `rune check-state-graph --fixture <path> --registry <path>`.
- Hardened validation after role/panel review so retained evidence refs,
  ownership refs, and duplicate graph ids fail closed.
- Updated DCR, requirements, trace, verification, validation, review, product,
  architecture, how-to, and runbook docs.

## Boundary

This pulse is read-only and retained-evidence-first. It does not add runtime host
behavior, live state inspection, pointer or heap walking, replay, mutation, Cargo
traversal, Rust source scraping, plugin discovery, automatic migration, or policy
enforcement.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
cargo run -q -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- inspect-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-state-graph --fixture crates\rune-cli\tests\fixtures\state_graph_workspace.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
git diff --check
```

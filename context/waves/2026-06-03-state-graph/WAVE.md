# Wave 43: Retained state graph evidence

## Goal

Implement the first retained Mission 2.0 state graph slice over semantic registry
evidence without adding live runtime behavior.

## Thesis

RUNE can make native Rust systems more analyzable when authored state nodes and
transitions are validated against explicit descriptor ids and semantic registry
refs before any live inspection or runtime host exists.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Retained state graph validation | complete | Added state graph model, fail-closed diagnostics, retained fixtures, `check-state-graph`, docs, VTRACE records, and role-review hardening for retained evidence refs, ownership refs, and duplicate graph ids. |

## Success criteria

- State graph fixtures have durable graph identity and semantic registry refs.
- Nodes reference known descriptor ids from retained registry collection refs.
- Transitions reference known source/target nodes and command/event descriptors.
- Retained evidence refs are required and must match semantic registry collection
  source refs.
- Ownership refs point to known graph node and transition ids.
- Duplicate node and transition ids fail closed.
- Live-state requests fail closed.
- CLI validation is read-only and fixture-backed.
- Runtime host behavior, live state inspection, mutation, replay, Cargo
  traversal, source scraping, plugin discovery, automatic migration, and policy
  enforcement remain blocked.

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

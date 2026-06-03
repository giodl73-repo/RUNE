# State graph validation runbook

Use this runbook to validate retained state graph evidence before treating a
Mission 2.0 state graph package as review-ready.

## Command sequence

```powershell
cargo run -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -p rune-cli -- check-state-graph --fixture crates\rune-cli\tests\fixtures\state_graph_workspace.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

## Pass criteria

| Check | Expected result |
|---|---|
| Registry validation | Registry metadata validates, profile/adapter refs are known, and collection source refs match retained descriptor collections. |
| State graph validation | Graph identity, registry ref, descriptor-backed nodes, command/event transitions, ownership, evidence refs, and capabilities validate. |
| Retained boundary | All inputs are retained fixtures; no live runtime state or source discovery is required. |

## Failure drills

Retained failure fixtures cover:

| Fixture | Expected diagnostic |
|---|---|
| `state_graph_unknown_descriptor.json` | `RUNE-STATE-003` |
| `state_graph_unknown_node.json` | `RUNE-STATE-004` |
| `state_graph_unsupported_transition.json` | `RUNE-STATE-005` |
| `state_graph_live_blocked.json` | `RUNE-STATE-006` |

## Blocked behavior

This runbook does not approve runtime host behavior, live state inspection,
pointer or heap graph walking, replay, mutation, Cargo traversal, Rust source
scraping, plugin discovery, automatic migration, or policy enforcement.

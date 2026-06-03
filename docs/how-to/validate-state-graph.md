# Validate a state graph

Use this workflow when retained state graph evidence needs to prove that graph
nodes and transitions line up with a semantic registry and descriptor collection
evidence.

## Inputs

| Input | Example |
|---|---|
| State graph fixture | `crates\rune-cli\tests\fixtures\state_graph_workspace.json` |
| Semantic registry fixture | `crates\rune-cli\tests\fixtures\semantic_registry_workspace.json` |
| Referenced collection fixtures | `known_contract_descriptor_collection.json`, `semantic_registry_adapter_contracts_collection.json` |

Registry collection source refs are resolved relative to the semantic registry
fixture.

## Check the graph

```powershell
cargo run -p rune-cli -- check-state-graph --fixture crates\rune-cli\tests\fixtures\state_graph_workspace.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

The check report confirms:

- graph identity and version,
- semantic registry identity and version,
- node count,
- transition count,
- retained evidence ref count,
- retained/live-state capability flags.

## Fail-closed diagnostics

| Diagnostic | Meaning |
|---|---|
| `RUNE-STATE-001` | Missing state graph identity or version. |
| `RUNE-STATE-002` | Missing or mismatched registry reference. |
| `RUNE-STATE-003` | Node references an unknown descriptor id. |
| `RUNE-STATE-004` | Transition references an unknown source or target node. |
| `RUNE-STATE-005` | Transition references an unknown or unsupported command/event descriptor. |
| `RUNE-STATE-006` | Live state was requested without an approved runtime host boundary. |

## Boundaries

State graph validation is read-only. It does not inspect live process state, walk
pointers or heap graphs, replay transitions, mutate state, inspect Cargo
metadata, scrape Rust source, discover plugins, or enable a runtime host.

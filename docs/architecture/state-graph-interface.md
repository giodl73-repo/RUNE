# State graph interface

## Purpose

The state graph lane describes retained or optional live state in terms of RUNE
descriptor ids, field metadata, state nodes, transitions, ownership, and
evidence references.

The first implementation must be retained-evidence first. Live state inspection
is explicitly deferred until a runtime host DCR approves it.

## Approved inputs

- semantic registry documents,
- descriptor collection documents,
- field metadata,
- state/event/command descriptors,
- retained evidence packet references.

## State graph document shape

First planning target:

| Field | Purpose |
|---|---|
| `state_graph_id` | Durable graph identity. |
| `state_graph_version` | Version of the state graph document shape. |
| `registry_ref` | Semantic registry id/version used to interpret node contracts. |
| `nodes` | Ordered state nodes keyed by descriptor id and stable local node id. |
| `transitions` | Ordered transitions keyed by command/event/condition descriptor refs. |
| `ownership` | Component, crate, or process owner for nodes and transitions. |
| `evidence_refs` | Links to retained evidence packets or descriptor collections. |
| `diagnostics` | Fail-closed state graph diagnostics. |

## Diagnostics

Reserve diagnostic families:

| Code | Meaning |
|---|---|
| `RUNE-STATE-001` | Missing state graph identity. |
| `RUNE-STATE-002` | Missing registry reference. |
| `RUNE-STATE-003` | Node references unknown descriptor id. |
| `RUNE-STATE-004` | Transition references unknown source or target node. |
| `RUNE-STATE-005` | Transition references unsupported command/event descriptor. |
| `RUNE-STATE-006` | Live state requested without runtime host approval. |

## Retained fixtures

Implementation must add:

- retained state graph over an existing descriptor collection,
- transition graph with command/event links,
- unknown descriptor failure fixture,
- live-state request rejection fixture.

## Implementation status

Wave 43 implements the retained evidence slice in `rune-core` and `rune-cli`:

- `StateGraphDraft` / `StateGraphDocument`,
- descriptor-backed nodes and command/event transitions,
- ownership and evidence refs,
- `rune check-state-graph --fixture <path> --registry <path>`,
- fail-closed diagnostics `RUNE-STATE-001` through `RUNE-STATE-006`.

The implementation remains read-only and fixture-backed.

## Non-goals

- No pointer/borrow inspection.
- No heap graph walking.
- No live process introspection.
- No mutation or replay.
- No product-specific state vocabulary in `rune-core`.

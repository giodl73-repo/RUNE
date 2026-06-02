# Data contracts

RUNE data contracts are profile-owned views over neutral descriptors. They make
Rust data, command, event, state, packet, and simulation surfaces explicit for AI
and review tools without moving domain vocabulary into `rune-core`.

## What a data contract preserves

| Surface | Source in neutral descriptor |
|---|---|
| Stable identity | `id` |
| Compatibility boundary | `version` |
| Contract family | `kind` |
| Rust symbol | `rust_type` |
| Shape | `fields` |
| Rules | `invariants` |
| Traceability | `trace_links` |
| Domain hints | namespaced `extensions` |

The `rune.data_contract_json` profile preserves these fields as data-contract
output. It does not validate runtime values; invariants remain declared contract
metadata until a future validator profile or adapter is reviewed.

## Why this matters for games and simulations

The games repo review showed that most useful RUNE adoption points are not only
traits. Stable structs and enums describe snapshots, commands, actions, panels,
metrics, event logs, validation packets, and runtime reports. These are ideal
RUNE data contracts because they answer AI and review questions such as:

| Question | RUNE answer |
|---|---|
| What snapshot shape should a renderer consume? | Descriptor fields and `kind = "state"` or `kind = "entity"`. |
| What command/event semantics are durable? | `kind = "command"` / `kind = "event"` with invariants. |
| What simulator outputs are comparable across runs? | `kind = "evidence"` or `kind = "artifact"` with trace links. |
| What game-specific rules exist? | Namespaced extensions owned by the adopter, not core. |

## Boundary

Use RUNE core fields for general Rust contract facts. Put game-specific formulas,
adapter rules, beat names, engine hints, or scenario language in namespaced
extensions, documentation profiles, or downstream adapters.

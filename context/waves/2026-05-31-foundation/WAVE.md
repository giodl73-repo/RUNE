# Wave: Foundation

## Goal

Establish RUNE as a neutral Rust contract derivation project with VTRACE-style
mission, requirements, traceability, verification, and review discipline from
the first commit.

## Engineering decision supported

Create RUNE as its own infrastructure repo rather than embedding AI contract
generation inside any downstream agent, context, workflow, or product system.

## Trace links expected

- Mission need: Rust code needs AI-readable contract metadata.
- Requirement: core specs stay neutral and extensible.
- Design: descriptors and derives emit stable metadata from Rust types.
- Implementation: `rune-core`, `rune-derive`, and `rune-cli`.
- Verification: formatting, workspace tests, and CLI status command.
- Evidence: VTRACE trace package and validation command output.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Foundation scaffold | complete |
| 02 | Ordered VTRACE stage gates | complete |
| 03 | Neutral descriptor vocabulary review | complete |
| 04 | Interface and control spec outline | complete |
| 05 | Scaffold alignment | complete |
| 06 | Verification gap closure | complete |
| 07 | First repo bakeoff scenario | planned/deferred |
| 08 | Foundation readiness review | complete |

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
```

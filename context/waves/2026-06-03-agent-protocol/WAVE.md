# Wave 45: Retained agent protocol

## Goal

Implement retained, read-first agent protocol request validation for AI tools
querying RUNE semantic registries, descriptor collections, evidence refs,
profiles, and adapters.

## Scope

- Add DCR-RUNE-006 for retained read-first agent protocol implementation.
- Implement protocol request draft/document models in `rune-core`.
- Add approved read operation validation and fail-closed protocol diagnostics.
- Add retained protocol pass/failure fixtures.
- Add read-only `rune check-agent-protocol --fixture <path> --registry <path>`.
- Update docs, VTRACE, readiness, communication, and wave records.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Retained agent protocol validation | complete | Added core protocol validation, retained fixtures/tests, CLI check, docs, and VTRACE records. |

## Boundaries

Agent protocol requests are retained artifacts. This wave does not implement a
server, SDK, live endpoint, runtime host, tool executor, prompt-only authority,
mutation/replay, private data exposure, Cargo traversal, source scraping, plugin
discovery, automatic migration, or policy enforcement.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
cargo run -q -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-state-graph --fixture crates\rune-cli\tests\fixtures\state_graph_workspace.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-evidence-packet --fixture crates\rune-cli\tests\fixtures\evidence_packet_diagnostic.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-agent-protocol --fixture crates\rune-cli\tests\fixtures\agent_protocol_registry_describe.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
git diff --check
```

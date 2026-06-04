# Wave 46: Retained compatibility negotiation

## Goal

Implement retained compatibility report validation for RUNE Mission 2.0 without
adding automatic migration, best-effort conversion, runtime host behavior, live
inspection, mutation, or policy enforcement.

## Scope

- Add DCR-RUNE-007 for retained compatibility negotiation implementation.
- Implement compatibility report draft/document models in `rune-core`.
- Add source/target artifact refs, degraded concept records, and fail-closed
  diagnostics.
- Add retained pass/failure compatibility fixtures.
- Add read-only `rune check-compatibility --fixture <path> --registry <path>`.
- Update docs, VTRACE, readiness, communication, and wave records.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Retained compatibility validation | complete | Added core compatibility report validation, retained fixtures/tests, CLI check, docs, and VTRACE records. |

## Boundaries

Compatibility reports are retained artifacts. This wave does not implement
automatic migration, best-effort conversion, runtime host negotiation, live
endpoint behavior, live state inspection, mutation/replay, private data exposure,
Cargo traversal, source scraping, plugin discovery, or policy enforcement.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
cargo run -q -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-state-graph --fixture crates\rune-cli\tests\fixtures\state_graph_workspace.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-evidence-packet --fixture crates\rune-cli\tests\fixtures\evidence_packet_diagnostic.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-agent-protocol --fixture crates\rune-cli\tests\fixtures\agent_protocol_registry_describe.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-compatibility --fixture crates\rune-cli\tests\fixtures\compatibility_collection_profile.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
git diff --check
```

# Agent protocol validation runbook

Run these commands before treating retained agent protocol requests as ready.

```powershell
cargo run -q -p rune-cli -- check-agent-protocol --fixture crates\rune-cli\tests\fixtures\agent_protocol_registry_describe.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-agent-protocol --fixture crates\rune-cli\tests\fixtures\agent_protocol_descriptor_get.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-agent-protocol --fixture crates\rune-cli\tests\fixtures\agent_protocol_compatibility_check.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

Failure fixtures must continue to fail closed with:

| Code | Expected failure |
|---|---|
| `RUNE-AGENT-001` | unknown operation |
| `RUNE-AGENT-002` | missing or mismatched capability |
| `RUNE-AGENT-003` | mutating operation |
| `RUNE-AGENT-004` | unknown registry, collection, descriptor, evidence, profile, or adapter ref |
| `RUNE-AGENT-005` | restricted data exposure |

The runbook is retained-evidence only. It does not approve live endpoints,
runtime host behavior, mutation, replay, Cargo traversal, source scraping,
plugin discovery, automatic migration, or policy enforcement.

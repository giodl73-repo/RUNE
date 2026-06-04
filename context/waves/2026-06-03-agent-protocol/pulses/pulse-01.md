# Pulse 01: Retained agent protocol validation

## Goal

Deliver the retained read-first agent protocol implementation approved by
DCR-RUNE-006.

## Implementation

- Added `AgentProtocolRequestDraft` and `AgentProtocolRequestDocument`.
- Added retained protocol input refs for registries, collections, descriptors,
  evidence, profiles, and adapters.
- Added validation for approved read operations, required capabilities,
  mutating-operation blocking, retained input refs, and restricted-data blocking.
- Added retained registry describe, descriptor get, and compatibility check
  fixtures plus fail-closed failure fixtures.
- Added `check-agent-protocol --fixture <path> --registry <path>`.

## Boundaries

No server, SDK, live endpoint, runtime host, tool executor, prompt-only
authority, mutation/replay, private data exposure, Cargo traversal, source
scraping, plugin discovery, automatic migration, or policy enforcement was
added.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
cargo run -q -p rune-cli -- check-agent-protocol --fixture crates\rune-cli\tests\fixtures\agent_protocol_registry_describe.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
git diff --check
```

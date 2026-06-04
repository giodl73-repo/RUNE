# Validate an agent protocol request

Use `check-agent-protocol` when you have a retained, read-first agent protocol
request and need to prove it references known registry, descriptor, evidence,
profile, or adapter evidence.

## Command

```powershell
cargo run -q -p rune-cli -- check-agent-protocol --fixture crates\rune-cli\tests\fixtures\agent_protocol_registry_describe.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

## Pass criteria

- The request has `protocol_id`, `protocol_version`, and an approved read
  operation.
- The request declares the capability required by the operation.
- The supplied semantic registry declares that capability.
- Descriptor, collection, evidence, profile, and adapter refs are known to the
  supplied registry and retained collection fixtures.
- Restricted-data requests are blocked.
- Mutating operations are blocked.

## Non-goals

This command does not start a live endpoint, run tools, authorize mutation,
inspect live state, traverse Cargo metadata, scrape source, discover plugins,
perform automatic migration, enforce policy, or implement a runtime host.

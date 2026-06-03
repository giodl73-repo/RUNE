# Evidence packet validation runbook

Run these commands before treating retained evidence runtime packets as ready.

```powershell
cargo run -q -p rune-cli -- check-evidence-packet --fixture crates\rune-cli\tests\fixtures\evidence_packet_diagnostic.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-evidence-packet --fixture crates\rune-cli\tests\fixtures\evidence_packet_validation.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-evidence-packet --fixture crates\rune-cli\tests\fixtures\evidence_packet_trace.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-evidence-packet --fixture crates\rune-cli\tests\fixtures\evidence_packet_health.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -q -p rune-cli -- check-evidence-packet --fixture crates\rune-cli\tests\fixtures\evidence_packet_audit.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

Failure fixtures must continue to fail closed with `RUNE-EVIDENCE-001` through
`RUNE-EVIDENCE-007` for missing identity, unsupported packet kind, unknown
descriptor refs, unsupported severity/status, missing audit capability decision,
mismatched registry refs, and unknown retained evidence refs.

The runbook is retained-evidence only. It does not approve runtime host behavior,
live state inspection, mutation, replay, Cargo traversal, source scraping, plugin
discovery, automatic migration, or policy enforcement.

# Validate an evidence runtime packet

Use `check-evidence-packet` when you have a retained diagnostic, validation,
trace, health, or audit packet and need to prove it references known descriptor
and registry evidence.

## Command

```powershell
cargo run -q -p rune-cli -- check-evidence-packet --fixture crates\rune-cli\tests\fixtures\evidence_packet_diagnostic.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

## Pass criteria

- The packet has `packet_id`, `packet_version`, and one approved `packet_kind`.
- `severity` is `info`, `warning`, `error`, or `blocked`.
- `status` is `pass`, `fail`, `blocked`, `degraded`, or `observed`.
- Descriptor refs match descriptor ids and versions in the supplied semantic
  registry's retained collection fixtures.
- Evidence refs are retained `descriptor_collection_fixture` refs declared by the
  supplied semantic registry.
- Audit packets include `capability_decision`.

## Non-goals

This command does not replace logging, metrics, traces, or observability
backends. It does not add a runtime host, live state inspection, private payload
capture, mutation authorization, Cargo traversal, source scraping, plugin
discovery, automatic migration, or policy enforcement.

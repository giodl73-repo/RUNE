# RUNE v1 release readiness

## Status

RUNE v1 is ready as publishable contract infrastructure with reviewed discovery,
external profile, adapter, adopter example, and bakeoff evidence. Mission 2.0
now has retained semantic registry evidence, retained state graph evidence, and
retained evidence runtime packets without live state inspection or runtime host
behavior.

## Crate surfaces

| Crate | V1 role |
|---|---|
| `rune-core` | Neutral descriptor, collection, discovery, profile, evidence, inventory, semantic registry, retained state graph, and evidence runtime packet models. |
| `rune-derive` | Compile-time derive macro for annotated Rust contract descriptors. |
| `rune-cli` | Fixture-backed inspection, check, generation, discovery, evidence, adapter, semantic registry, state graph, and evidence packet commands. |
| `rune-adapters` | Downstream adapter models outside the neutral core. |
| `examples\rune-adopter` | Non-published adopter workflow example. |

## CI-ready validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
cargo run -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -p rune-cli -- inspect-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -p rune-cli -- check-state-graph --fixture crates\rune-cli\tests\fixtures\state_graph_workspace.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -p rune-cli -- check-evidence-packet --fixture crates\rune-cli\tests\fixtures\evidence_packet_diagnostic.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
git diff --check
```

## Compatibility policy

- Descriptor and collection documents are versioned with explicit `version` and
  `collection_version` fields.
- Durable generated outputs must preserve profile or adapter identity and
  version.
- Breaking neutral descriptor changes require a new descriptor version and
  retained fixture updates.
- New external profiles must be added through a reviewed profile boundary and
  must not rewrite `rune-core` vocabulary.
- New adapters must live outside `rune-core` and must retain input/output
  fixtures plus fail-closed diagnostics.
- Diagnostic families are treated as review-facing contracts; changes require
  VTRACE updates.
- Semantic registry evidence is retained and versioned; registry source refs
  must resolve to retained descriptor collection fixtures.
- State graph evidence is retained and versioned; graph nodes must reference
  known descriptors, transitions must reference known nodes and command/event
  descriptors, evidence refs must match semantic registry collection source
  refs, ownership refs must resolve to known graph ids, duplicate graph ids fail
  closed, and live-state requests fail closed.
- Evidence runtime packets are retained and versioned; packet families, severity,
  status, descriptor refs, registry refs, evidence refs, and audit capability
  decisions fail closed before packets are treated as usable evidence.

## Release evidence

- VTRACE requirements and trace are complete through v1 release readiness.
- Retained fixtures cover descriptor, collection, discovery, evidence bundle,
  external profile, adapter, adopter workflow, semantic registry, and QUIVER
  bakeoff scenarios plus retained state graph and evidence packet pass/failure
  scenarios.
- Full workspace validation passes with the CI-ready commands above.

## Non-goals for v1

- No arbitrary Rust source scraping.
- No Cargo metadata traversal as discovery.
- No executable registry hooks.
- No product-specific vocabulary in `rune-core`.
- No automatic publishing to downstream systems.
- No runtime host, live state inspection, mutating agent operations, plugin
  discovery, automatic migration, or policy enforcement.

# RUNE v1 release readiness

## Status

RUNE v1 is ready as publishable contract infrastructure with reviewed discovery,
external profile, adapter, adopter example, and bakeoff evidence.

## Crate surfaces

| Crate | V1 role |
|---|---|
| `rune-core` | Neutral descriptor, collection, discovery, profile, evidence, and inventory models. |
| `rune-derive` | Compile-time derive macro for annotated Rust contract descriptors. |
| `rune-cli` | Fixture-backed inspection, check, generation, discovery, evidence, and adapter commands. |
| `rune-adapters` | Downstream adapter models outside the neutral core. |
| `examples\rune-adopter` | Non-published adopter workflow example. |

## CI-ready validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
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

## Release evidence

- VTRACE requirements and trace are complete through v1 release readiness.
- Retained fixtures cover descriptor, collection, discovery, evidence bundle,
  external profile, adapter, adopter workflow, and QUIVER bakeoff scenarios.
- Full workspace validation passes with the CI-ready commands above.

## Non-goals for v1

- No arbitrary Rust source scraping.
- No Cargo metadata traversal as discovery.
- No executable registry hooks.
- No product-specific vocabulary in `rune-core`.
- No automatic publishing to downstream systems.

# RUNE PITFALL Doctrine Integration

## Goal

Connect RUNE's reusable PITFALL memory to normal contract, VTRACE, role-review,
adopter, and compatibility work without changing product source or touching the
FERRIS-specific consumer proof.

## Findings

- `RUNE-PF-01` is mitigated: downstream product, agent, workflow, schema, and
  context vocabulary stays outside `rune-core` and is routed through reviewed
  profiles or adapters.
- `RUNE-PF-02` is mitigated: derive convenience remains inspectable through
  explicit metadata, retained fixtures, compile-fail tests, and opt-in evidence
  regeneration.
- `RUNE-PF-03` is mitigated: retained descriptor evidence, registries, and
  manifests remain the accepted path instead of broad source scraping or live
  runtime inspection.
- `RUNE-PF-04` is mitigated: unsupported versions, unsupported concepts,
  adapter limitations, and degradation fail closed or carry explicit warnings.
- `RUNE-PF-05` is mitigated: read-first protocol, state graph, evidence packet,
  and compatibility fixtures do not authorize mutation, live endpoints,
  restricted data, policy enforcement, runtime host behavior, or automatic
  migration.

## Role Coverage

- Contract Model Steward protects neutral descriptors from downstream vocabulary
  capture.
- Macro Safety Steward keeps generated behavior deterministic and inspectable.
- Generator Interop Steward and Platform Adapter Author preserve profile and
  adapter boundaries without widening the neutral core.
- VTRACE Traceability Auditor keeps mission-to-evidence proof from becoming
  macro enthusiasm.
- AI Contract Consumer and Future Agent keep agent-readable output tied to
  retained evidence instead of prompt convention or source scraping.
- Rust Maintainer keeps adoption incremental and idiomatic for real Rust crates.

## Tracker Integration

RUNE remains a high-value PITFALL adopter for cross-repo scoring because many
other repos want its descriptors, retained evidence, compatibility checks, and
agent-readable surfaces. PITFALL makes the improvement measurable: a repo can
claim RUNE adoption only when it identifies the descriptor boundary, retained
evidence source, compatibility command, adapter/profile owner, and runtime
authority limit that apply to that repo.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p rune-cli -- status`
- `git diff --check`

## Status

Complete.

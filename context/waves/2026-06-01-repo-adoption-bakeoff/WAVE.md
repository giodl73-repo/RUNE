# Wave: Repo adoption bakeoff

## Goal

Validate whether RUNE's neutral descriptor, check, profile catalog, and neutral
generated artifact surfaces improve review and AI handoff for representative
Rust code without adding product-specific adapters or broad crate discovery.

## Engineering decision supported

RUNE may proceed toward repository adoption only when constrained scenarios show
that generated descriptor evidence is more useful than source/prose inspection
alone, while preserving neutral core vocabulary and fail-closed diagnostics.

## Scope

This wave is validation-first. It may use retained fixtures, annotated test
types, and scenario reports. It must not add JSON Schema, CSDL, OpenAPI,
AgentMap, BAKER, LATTICE, or other product adapters.

## Trace links expected

- Mission need: AI tooling and reviewers need stable, inspectable descriptors.
- Requirement: representative bakeoffs before broad adoption.
- Design: descriptor document, profile catalog, compatibility check, neutral
  descriptor-output profile.
- Implementation: existing `rune-core`, `rune-derive`, and `rune-cli` surfaces.
- Verification: workspace tests, check/profile/generate commands.
- Validation: scenario findings in `docs/vtrace/BAKEOFF.md` and
  `docs/vtrace/VALIDATION.md`.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Adoption scenario gate | complete |
| 02 | Source-only baseline comparison | complete |
| 03 | RUNE evidence comparison | complete |
| 04 | Readiness decision | complete |

## Readiness decision

Wave 4 passes for controlled scenario-level usefulness and remains blocked for
broad adoption.

RUNE evidence improves the annotated `Customer` scenario by making durable
identity, version, kind, trace link, compatibility status, profile metadata, and
artifact shape explicit. Broad adoption remains blocked until RUNE can produce or
collect descriptor evidence from a build/test path without relying on manually
maintained CLI fixtures.

## Validation

```powershell
git diff --check
cargo test --workspace
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
```

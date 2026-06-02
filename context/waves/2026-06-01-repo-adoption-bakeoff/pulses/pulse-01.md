# Pulse 01: Adoption scenario gate

## Goal

Define the first repo-adoption validation gate before running scenario
comparisons or adding any downstream profile/adopter code.

## Engineering decision supported

RUNE can begin a constrained adoption bakeoff because Wave 3 now provides
fixture-backed descriptor inspection, compatibility checking, profile discovery,
and neutral generated artifacts. Adoption remains scenario-only until usefulness
is demonstrated.

## Scenario target

Use the existing annotated `Customer` contract as the first controlled target:

- annotated source: `crates/rune-derive/tests/derive_contract.rs`
- descriptor evidence:
  `crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json`
- check evidence: `crates/rune-cli/tests/fixtures/annotated_customer.check.json`
- generated artifact:
  `crates/rune-cli/tests/fixtures/annotated_customer.neutral_descriptor_artifact.json`

## Baseline to compare

The baseline is source/prose-only review of the annotated Rust type and related
tests, without relying on RUNE descriptor, check, or generated artifact output.

## RUNE evidence path

```powershell
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
```

## Pass criteria

| Criterion | Required evidence |
|---|---|
| Inspectability | Contract identity, version, kind, Rust type, fields, and trace links are visible without manual macro expansion. |
| Compatibility | `rune check` reports profile compatibility without emitting a generated artifact. |
| Artifact usefulness | Neutral generated artifact preserves descriptor and profile metadata. |
| Neutrality | No downstream product vocabulary or adapter dependency appears in core output. |
| Limits | Scenario explicitly remains fixture-backed and does not claim crate discovery. |

## Result

Complete. The first Wave 4 scenario gate is defined and constrained to existing
neutral RUNE evidence surfaces.

## Validation

```powershell
git diff --check
cargo test --workspace
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
```

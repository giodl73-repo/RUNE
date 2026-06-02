# Pulse 04: Annotated Rust type bakeoff

## Goal

Run a constrained bakeoff that compares an annotated Rust type with retained
neutral descriptor and generated artifact evidence.

## Engineering decision supported

RUNE's derive and neutral generator surfaces can preserve the contract shape of
a small annotated Rust type, while arbitrary crate analysis and external profile
generation remain out of scope.

## Trace links expected

- `crates/rune-derive/tests/derive_contract.rs`
- `crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json`
- `crates/rune-cli/tests/fixtures/annotated_customer.neutral_descriptor_artifact.json`
- `docs/vtrace/BAKEOFF.md`
- `docs/vtrace/VALIDATION.md`

## Evidence produced

- Annotated Rust `Customer` type with `#[derive(RuneContract)]`.
- Retained neutral descriptor fixture matching the annotated type.
- Retained neutral generated artifact for that descriptor.
- CLI test comparing generated output to expected artifact.

## Result

Complete with limits. The bakeoff validates the annotated type contract shape
through retained descriptor and generated artifact evidence. It does not yet
validate automatic crate-wide discovery or external profile generation.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
git diff --check
```

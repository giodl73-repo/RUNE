# Wave 36: Field metadata

## Goal

Implement the first DCR-RUNE-001 field-level metadata slice so data contracts can
carry required status, units, bounds, sensitivity, examples, stability, and
aliases without inferring from Rust source or product prose.

## Scope

- Extend RUNE core field descriptors and documents with explicit field metadata.
- Add `#[rune_field(...)]` derive authoring and fail-closed diagnostics.
- Preserve metadata through `rune.data_contract_json` and documentation packets.
- Upgrade the shape calculator retained fixtures.
- Record games spike guidance for RALLY, COURT, MUDDLE, and RACKET.

## Non-goals

- Do not add runtime validation semantics.
- Do not infer metadata from doc comments or type names.
- Do not add trait/function contracts in this wave.
- Do not edit games repos in this wave.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
git diff --check
```

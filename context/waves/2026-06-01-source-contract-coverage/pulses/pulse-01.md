# Pulse 01: Source contract scenario

## Goal

Add a source-contract scenario as the sixth explicitly registered known contract.

## Engineering decision supported

RUNE evidence collection can cover source descriptors in addition to entity,
command, event, state, and artifact descriptors without changing the neutral core
vocabulary.

## Scenario

```rust
#[derive(RuneContract)]
#[rune(
    id = "example.contract_source_reference",
    version = "v0",
    kind = "source",
    requirement = "RUNE-REQ-050"
)]
struct ContractSourceReference {
    descriptor_id: String,
    source_path: String,
    source_symbol: String,
}
```

## Evidence produced

- `crates/rune-cli/tests/fixtures/annotated_contract_source_reference_descriptor.json`
- `crates/rune-cli/tests/fixtures/annotated_contract_source_reference.check.json`
- `crates/rune-cli/tests/fixtures/annotated_contract_source_reference.neutral_descriptor_artifact.json`
- updated `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.json`

## Result

Complete with limits. The source scenario is explicitly registered and covered by
derive, check, generate, and collection evidence. It does not add discovery.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

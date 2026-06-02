# Pulse 01: State contract scenario

## Goal

Add a state-contract scenario as the fourth explicitly registered known contract.

## Engineering decision supported

RUNE evidence collection can cover state descriptors in addition to entity,
command, and event descriptors without changing the neutral core vocabulary.

## Scenario

```rust
#[derive(RuneContract)]
#[rune(
    id = "example.customer_lifecycle",
    version = "v0",
    kind = "state",
    requirement = "RUNE-REQ-048"
)]
struct CustomerLifecycleState {
    customer_id: String,
    status: String,
    updated_at: String,
}
```

## Evidence produced

- `crates/rune-cli/tests/fixtures/annotated_customer_lifecycle_state_descriptor.json`
- `crates/rune-cli/tests/fixtures/annotated_customer_lifecycle_state.check.json`
- `crates/rune-cli/tests/fixtures/annotated_customer_lifecycle_state.neutral_descriptor_artifact.json`
- updated `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.json`

## Result

Complete with limits. The state scenario is explicitly registered and covered by
derive, check, generate, and collection evidence. It does not add discovery.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

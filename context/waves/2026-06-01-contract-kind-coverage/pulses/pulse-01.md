# Pulse 01: Event contract scenario

## Goal

Add an event-contract scenario as the third explicitly registered known
contract.

## Engineering decision supported

RUNE evidence collection can cover event descriptors in addition to entity and
command descriptors without changing the neutral core vocabulary.

## Scenario

```rust
#[derive(RuneContract)]
#[rune(
    id = "example.customer_created",
    version = "v0",
    kind = "event",
    requirement = "RUNE-REQ-047"
)]
struct CustomerCreated {
    customer_id: String,
    occurred_at: String,
}
```

## Evidence produced

- `crates/rune-cli/tests/fixtures/annotated_customer_created_event_descriptor.json`
- `crates/rune-cli/tests/fixtures/annotated_customer_created_event.check.json`
- `crates/rune-cli/tests/fixtures/annotated_customer_created_event.neutral_descriptor_artifact.json`
- updated `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.json`

## Result

Complete with limits. The event scenario is explicitly registered and covered by
derive, check, generate, and collection evidence. It does not add discovery.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

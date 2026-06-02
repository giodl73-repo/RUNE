# Pulse 02: Second annotated contract fixture

## Goal

Add a command-contract scenario as the second explicitly registered known
contract.

## Engineering decision supported

RUNE evidence collection works for more than one annotated contract kind when a
second known contract can derive, check, and generate retained neutral evidence.

## Scenario

```rust
#[derive(RuneContract)]
#[rune(
    id = "example.create_customer",
    version = "v0",
    kind = "command",
    requirement = "RUNE-REQ-045"
)]
struct CreateCustomer {
    customer_id: String,
    email: String,
}
```

## Evidence produced

- `crates/rune-cli/tests/fixtures/annotated_create_customer_command_descriptor.json`
- `crates/rune-cli/tests/fixtures/annotated_create_customer_command.check.json`
- `crates/rune-cli/tests/fixtures/annotated_create_customer_command.neutral_descriptor_artifact.json`

## Result

Complete with limits. RUNE now has known-contract evidence for an entity and a
command. This still does not scan arbitrary crates.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

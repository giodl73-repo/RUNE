# Pulse 01: Evidence contract scenario

## Goal

Add an evidence-contract scenario as the seventh explicitly registered known
contract.

## Engineering decision supported

RUNE evidence collection can cover the final neutral profile-supported
descriptor kind without changing core vocabulary or adding discovery.

## Scenario

```rust
#[derive(RuneContract)]
#[rune(
    id = "example.contract_verification_evidence",
    version = "v0",
    kind = "evidence",
    requirement = "RUNE-REQ-051"
)]
struct ContractVerificationEvidence {
    descriptor_id: String,
    verification_id: String,
    evidence_uri: String,
}
```

## Evidence produced

- `crates/rune-cli/tests/fixtures/annotated_contract_verification_evidence_descriptor.json`
- `crates/rune-cli/tests/fixtures/annotated_contract_verification_evidence.check.json`
- `crates/rune-cli/tests/fixtures/annotated_contract_verification_evidence.neutral_descriptor_artifact.json`
- updated `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.json`

## Result

Complete with limits. The evidence scenario is explicitly registered and covered
by derive, check, generate, and collection evidence. It does not add discovery.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

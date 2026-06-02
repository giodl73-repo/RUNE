# Pulse 01: Artifact contract scenario

## Goal

Add an artifact-contract scenario as the fifth explicitly registered known
contract.

## Engineering decision supported

RUNE evidence collection can cover artifact descriptors in addition to entity,
command, event, and state descriptors without changing the neutral core
vocabulary.

## Scenario

```rust
#[derive(RuneContract)]
#[rune(
    id = "example.contract_evidence",
    version = "v0",
    kind = "artifact",
    requirement = "RUNE-REQ-049"
)]
struct ContractEvidenceArtifact {
    descriptor_id: String,
    artifact_uri: String,
    artifact_kind: String,
}
```

## Evidence produced

- `crates/rune-cli/tests/fixtures/annotated_contract_evidence_artifact_descriptor.json`
- `crates/rune-cli/tests/fixtures/annotated_contract_evidence_artifact.check.json`
- `crates/rune-cli/tests/fixtures/annotated_contract_evidence_artifact.neutral_descriptor_artifact.json`
- updated `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.json`

## Result

Complete with limits. The artifact scenario is explicitly registered and covered
by derive, check, generate, and collection evidence. It does not add discovery.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

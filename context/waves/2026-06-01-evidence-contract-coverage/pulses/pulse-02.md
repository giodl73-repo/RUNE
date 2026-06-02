# Pulse 02: Evidence coverage readiness

## Goal

Decide whether evidence descriptor coverage completes the current
profile-supported known-contract scenario set.

## Decision

Pass for bounded evidence coverage. RUNE now has retained known-contract evidence
for:

- `Customer`, an entity descriptor,
- `CreateCustomer`, a command descriptor,
- `CustomerCreated`, an event descriptor,
- `CustomerLifecycleState`, a state descriptor,
- `ContractEvidenceArtifact`, an artifact descriptor,
- `ContractSourceReference`, a source descriptor,
- `ContractVerificationEvidence`, an evidence descriptor.

## Limits

This does not approve crate discovery, source inference, external profiles, CLI
collection, product adapters, or the valid-but-profile-unsupported `other`
escape hatch.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

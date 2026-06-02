# Pulse 04: Collection readiness decision

## Goal

Decide whether Wave 6 evidence is sufficient to move beyond single-contract
derive evidence.

## Decision

Pass for controlled known-contract collection. RUNE now has deterministic
retained evidence for:

- `Customer`, an entity descriptor,
- `CreateCustomer`, a command descriptor,
- a stable collection containing both descriptors.

## Limits

This decision does not approve arbitrary crate scanning, source inference,
external profile generation, downstream product adapters, or repo-wide adoption.
The only approved collection path is the explicit test-owned known-contract set.

## Next gate

Before broadening collection, RUNE must define and review either an explicit
registry interface or a discovery interface with diagnostics, ordering, and
evidence-retention rules.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

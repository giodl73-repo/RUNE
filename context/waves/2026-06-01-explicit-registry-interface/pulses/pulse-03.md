# Pulse 03: Registry readiness decision

## Goal

Decide whether the explicit registry interface is sufficient to replace ad hoc
known-contract arrays in controlled evidence paths.

## Decision

Pass for controlled explicit registry collection. `rune-core` now owns:

- `ContractRegistration`,
- `collect_known_contract_documents`,
- duplicate descriptor id diagnostic `RUNE-REG-001`.

## Limits

This decision does not approve arbitrary crate scanning, Cargo workspace
discovery, proc-macro inventory, linker-section registries, CLI collection
commands, external profile output, or product-specific adapters.

## Next gate

Before repo-scale collection, RUNE must define a separate discovery or CLI
collection interface with deterministic ordering, explicit diagnostics, retained
evidence rules, and fail-closed behavior.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

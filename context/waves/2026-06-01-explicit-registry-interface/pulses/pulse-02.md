# Pulse 02: Core registry collection helper

## Goal

Move the known-contract collection boundary into `rune-core`.

## Engineering decision supported

Explicit known-contract collection can be shared by tests and future reviewed
surfaces without relying on ad hoc local arrays.

## Evidence produced

- `rune_core::ContractRegistration`
- `rune_core::collect_known_contract_documents`
- derive integration test collection routed through the core helper

## Result

Complete with limits. The helper preserves caller order and rejects duplicate
descriptor ids with `RUNE-REG-001`. It does not discover contracts.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

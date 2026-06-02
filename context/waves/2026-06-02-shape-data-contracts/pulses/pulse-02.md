# Pulse 02: Data-contract profile

## Intent

Add `rune.data_contract_json` so metadata-rich descriptors can produce a
profile-owned data-contract artifact without changing neutral descriptor
vocabulary.

## Validation

```powershell
cargo test -p rune-core
cargo test -p rune-cli
```

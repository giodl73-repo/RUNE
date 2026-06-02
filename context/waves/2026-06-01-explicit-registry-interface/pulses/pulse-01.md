# Pulse 01: Registry interface control

## Goal

Define the explicit registry boundary before broadening known-contract
collection.

## Engineering decision supported

RUNE may collect explicitly registered known contracts through a neutral core
interface, but broad discovery remains blocked.

## Evidence produced

- `docs/architecture/explicit-registry-interface.md`

## Result

Complete. The approved interface is an ordered slice of
`ContractRegistration` records collected into `DescriptorDocument` evidence.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

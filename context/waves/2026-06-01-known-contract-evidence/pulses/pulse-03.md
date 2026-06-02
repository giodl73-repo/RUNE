# Pulse 03: Multi-contract evidence check

## Goal

Prove that multiple explicitly registered known contracts can be serialized into
one stable retained descriptor collection.

## Engineering decision supported

RUNE can collect known-contract evidence without adding arbitrary crate scanning,
external profiles, or product-specific adapters.

## Evidence produced

- `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.json`

## Result

Complete with limits. The retained collection is generated from the two known
annotated contracts in the derive integration test: `Customer` and
`CreateCustomer`. This proves deterministic collection for an explicit test
registry only; it does not discover contracts across a crate.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

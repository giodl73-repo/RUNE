# Pulse 01: Collection document envelope

## Goal

Wrap retained known-contract descriptor evidence in a durable collection
document.

## Engineering decision supported

Multi-contract evidence needs its own identity/version boundary before it can be
used as durable evidence by future reviewed surfaces.

## Evidence produced

- `rune_core::DescriptorCollectionDocument`
- updated `crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.json`

## Result

Complete with limits. The collection is still produced from the explicit
known-contract registry and does not add discovery.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

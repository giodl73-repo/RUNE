# Pulse 02: Adopter registry example

## Goal

Add a workspace example crate that owns contracts, registry, and retained
collection evidence.

## Engineering decision supported

The registry workflow works outside RUNE's derive integration tests.

## Evidence produced

- `examples/rune-adopter/src/lib.rs`
- `examples/rune-adopter/tests/registry_workflow.rs`
- `examples/rune-adopter/tests/fixtures/adopter_contract_collection.json`

## Result

Complete. The example crate exposes `RUNE_CONTRACTS`, builds a
`DescriptorCollectionDocument`, compares retained fixture evidence, and fails
closed on duplicate descriptor ids.

## Validation

```powershell
cargo test -p rune-adopter
cargo fmt --check
cargo test --workspace
git diff --check
```

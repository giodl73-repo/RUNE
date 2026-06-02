# Pulse 02: Deterministic descriptor evidence path

## Goal

Add a deterministic test-only path for producing retained descriptor evidence
from the annotated `Customer` contract.

## Engineering decision supported

RUNE can regenerate descriptor evidence for a known annotated contract through
`rune-core::DescriptorDocument`, without adding arbitrary crate discovery or a
separate descriptor JSON model.

## Implementation

`crates/rune-derive/tests/derive_contract.rs` now exposes the evidence path for
the annotated `Customer` type:

- `customer_descriptor_json()` serializes `DescriptorDocument::from_contract::<Customer>()`.
- `derive_document_serialization_is_deterministic` verifies repeated
  serialization is stable.
- `update_retained_customer_descriptor_when_requested` writes the retained
  descriptor fixture only when `RUNE_UPDATE_EVIDENCE` is set.

## Regeneration command

```powershell
$env:RUNE_UPDATE_EVIDENCE = '1'
cargo test -p rune-derive --test derive_contract update_retained_customer_descriptor_when_requested
Remove-Item Env:RUNE_UPDATE_EVIDENCE
```

## Result

Complete with limits. Evidence regeneration is deterministic and test-only for a
known annotated contract. It does not scan arbitrary crates or generate external
profile outputs.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

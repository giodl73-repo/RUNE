# Pulse 03: Retained evidence regeneration check

## Goal

Prove the opt-in evidence writer can regenerate the retained annotated customer
descriptor without changing its content.

## Engineering decision supported

RUNE can treat the retained annotated customer descriptor as generated evidence
for the controlled scenario because the fixture can be rewritten from the
annotated Rust type and remains byte-stable.

## Evidence path

Source contract:

```text
crates/rune-derive/tests/derive_contract.rs
```

Retained evidence:

```text
crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json
```

Regeneration command:

```powershell
$env:RUNE_UPDATE_EVIDENCE = '1'
cargo test -p rune-derive --test derive_contract update_retained_customer_descriptor_when_requested
Remove-Item Env:RUNE_UPDATE_EVIDENCE
```

## Result

Complete. The retained descriptor can be regenerated from
`DescriptorDocument::from_contract::<Customer>()` through the opt-in test writer.
Normal tests continue to compare the retained fixture without mutating it.

## Limits

- This covers one known annotated contract.
- This does not discover descriptors across arbitrary crates.
- This does not emit external profile formats.
- This does not replace future repo-scale evidence collection.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-derive --test derive_contract
$env:RUNE_UPDATE_EVIDENCE = '1'
cargo test -p rune-derive --test derive_contract update_retained_customer_descriptor_when_requested
Remove-Item Env:RUNE_UPDATE_EVIDENCE
cargo test --workspace
git diff --check
```

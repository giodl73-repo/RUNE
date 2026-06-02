# Generate retained evidence

Use retained collection evidence when a reviewer or downstream tool needs a
portable bundle.

## Inspect the collection

```powershell
cargo run -p rune-cli -- inspect-collection --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

## Check profile compatibility

```powershell
cargo run -p rune-cli -- check-collection --profile rune.neutral_descriptor_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

## Emit an evidence bundle

```powershell
cargo run -p rune-cli -- evidence-collection --profile rune.neutral_descriptor_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

The command is read-only. Redirect output only when you intentionally want to
retain a regenerated fixture:

```powershell
cargo run -p rune-cli -- evidence-collection --profile rune.neutral_descriptor_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json > evidence-bundle.json
```

## Review checklist

- The collection identity and version are present.
- Every descriptor has an id, version, kind, and Rust type.
- Compatibility status is explicit.
- Generated artifacts preserve profile identity and version.

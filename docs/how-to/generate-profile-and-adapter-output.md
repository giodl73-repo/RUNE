# Generate profile and adapter output

Use profile output when you want a RUNE-owned generated artifact. Use adapter
output when a downstream consumer needs a separate artifact shape.

## Documentation packet profile

```powershell
cargo run -p rune-cli -- generate-collection --profile rune.documentation_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

This emits a deterministic documentation packet without changing neutral
descriptor vocabulary.

## Review packet adapter

```powershell
cargo run -p rune-cli -- adapt-collection --adapter rune.review_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

This emits a deterministic review packet from `rune-adapters`.

## Boundary rule

If the output vocabulary is consumer-specific, keep it in an adapter. If it is a
general RUNE profile, keep the profile mapping explicit and retain fixtures.

# RUNE adopter guide

## Purpose

This guide shows the current v1 path for a Rust crate that wants machine-readable
contract evidence without adopting a product-specific framework.

## Example crate

The workspace example `examples\rune-adopter` demonstrates the adopter-owned
workflow:

1. Annotate Rust contract types with `#[derive(RuneContract)]`.
2. Expose an explicit `RUNE_CONTRACTS` registry.
3. Build a `DescriptorCollectionDocument` from the registry.
4. Retain collection evidence under `tests\fixtures`.
5. Generate external profile evidence with `rune.documentation_packet_json`.
6. Generate downstream adapter evidence with `rune.review_packet_json`.

## Evidence commands

```powershell
cargo run -p rune-cli -- inspect-collection --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
cargo run -p rune-cli -- check-collection --profile rune.neutral_descriptor_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
cargo run -p rune-cli -- generate-collection --profile rune.documentation_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
cargo run -p rune-cli -- adapt-collection --adapter rune.review_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

## Retained fixtures

`examples\rune-adopter\tests\fixtures` retains:

- `adopter_contract_collection.json`
- `adopter_documentation_packet.json`
- `adopter_review_packet.json`

## Boundaries

The adopter workflow is explicit and deterministic. It does not scan Rust source,
traverse Cargo metadata, execute hooks, or add downstream adapter vocabulary to
`rune-core`.

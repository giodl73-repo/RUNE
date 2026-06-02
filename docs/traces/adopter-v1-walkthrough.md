# Adopter v1 walkthrough

## Decision supported

A Rust crate can adopt RUNE v1 without adding product-specific vocabulary or
depending on arbitrary source scanning.

## Inputs

| Input | Location |
|---|---|
| Example crate | `examples\rune-adopter` |
| Collection fixture | `examples\rune-adopter\tests\fixtures\adopter_contract_collection.json` |
| Documentation packet fixture | `examples\rune-adopter\tests\fixtures\adopter_documentation_packet.json` |
| Review packet fixture | `examples\rune-adopter\tests\fixtures\adopter_review_packet.json` |

## Walkthrough

1. The adopter crate annotates stable contract types.
2. The crate exposes an explicit `RUNE_CONTRACTS` registry.
3. Tests compare the generated descriptor collection to retained evidence.
4. CLI commands inspect and check the retained collection.
5. The documentation packet profile emits AI-readable documentation evidence.
6. The review packet adapter emits deterministic review evidence.

## Validation commands

For the full ordered command sequence, use
`docs\runbooks\adopter-evidence-validation.md`.

```powershell
cargo test -p rune-adopter
cargo run -p rune-cli -- inspect-collection --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
cargo run -p rune-cli -- generate-collection --profile rune.documentation_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
cargo run -p rune-cli -- adapt-collection --adapter rune.review_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

## Boundary

The walkthrough does not require source scraping, Cargo traversal, executable
hooks, or downstream product terms in `rune-core`.

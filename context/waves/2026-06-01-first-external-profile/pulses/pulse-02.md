# Pulse 02: Profile generation fixtures

## Goal

Generate retained artifacts for the documentation packet profile.

## Engineering decision supported

The existing `generate` and `generate-collection` surfaces can emit external
profile artifacts after profile compatibility validation.

## Evidence produced

- `crates\rune-cli\tests\fixtures\profile_list.json`
- `crates\rune-cli\tests\fixtures\annotated_customer.documentation_packet.json`
- `crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.documentation_packet.json`
- CLI generation tests

## Result

Complete. Descriptor and collection documentation packet artifacts are retained
and tested.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-cli --test profile_cli
cargo test -p rune-cli --test generate_cli
cargo test --workspace
git diff --check
```

# Pulse 01: Adopter v1 fixtures

## Goal

Extend the adopter example with profile and adapter evidence.

## Engineering decision supported

The adopter-owned registry can feed retained external profile and adapter
artifacts.

## Evidence produced

- `examples\rune-adopter\tests\v1_workflow.rs`
- `examples\rune-adopter\tests\fixtures\adopter_documentation_packet.json`
- `examples\rune-adopter\tests\fixtures\adopter_review_packet.json`

## Result

Complete. The example crate compares generated documentation packet and review
packet output against retained fixtures.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-adopter
cargo test --workspace
git diff --check
```

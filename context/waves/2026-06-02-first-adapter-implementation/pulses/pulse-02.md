# Pulse 02: CLI adapter evidence

## Goal

Expose the first adapter through the CLI with retained evidence.

## Engineering decision supported

`rune adapt-collection --adapter rune.review_packet_json --fixture <path>` is a
bounded first adapter command over validated descriptor collections.

## Evidence produced

- `rune adapter list`
- `rune adapt-collection --adapter rune.review_packet_json --fixture <path>`
- `crates\rune-cli\tests\adapter_cli.rs`
- `crates\rune-cli\tests\fixtures\adapter_list.json`
- `crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.review_packet.json`

## Result

Complete. The CLI emits retained adapter outputs and fails closed for unknown
adapters, malformed collections, and unsupported descriptor kinds.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-cli --test adapter_cli
cargo test --workspace
git diff --check
```

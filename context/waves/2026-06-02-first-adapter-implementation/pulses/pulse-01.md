# Pulse 01: Adapter crate and model

## Goal

Add a separate adapter crate and first adapter model.

## Engineering decision supported

Adapter vocabulary belongs in `rune-adapters`, not in `rune-core`.

## Evidence produced

- `crates\rune-adapters\Cargo.toml`
- `crates\rune-adapters\src\lib.rs`
- `AdapterCatalog`
- `AdapterDescriptor`
- `ReviewPacketDocument`

## Result

Complete. `rune.review_packet_json` is available through the adapter catalog and
has compatibility checks for supported descriptor kinds and concepts.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-adapters
cargo test --workspace
git diff --check
```

# Pulse 02: Discovery CLI and evidence

## Goal

Expose the approved discovery boundary through a CLI command with retained
evidence.

## Engineering decision supported

`rune discover --manifest <path>` is sufficient for the first deterministic
discovery slice because it consumes retained descriptor collection fixtures and
does not inspect crate source.

## Evidence produced

- `rune discover --manifest <path>`
- `crates\rune-cli\tests\fixtures\adopter_discovery_manifest.json`
- `crates\rune-cli\tests\fixtures\adopter_discovered_collection.json`
- CLI discovery pass/fail tests

## Result

Complete. The CLI emits a deterministic discovered collection for the adopter
example and fails closed for unsupported source kinds, malformed sources, missing
manifest identity, and duplicate descriptor ids.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-cli --test inspect_cli
cargo test --workspace
git diff --check
```

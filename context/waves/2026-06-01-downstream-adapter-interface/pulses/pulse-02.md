# Pulse 02: Adapter metadata and diagnostics

## Goal

Define required adapter metadata and fail-closed diagnostics.

## Engineering decision supported

Every adapter needs stable identity, version, accepted input artifact kinds,
supported concepts, output artifact kind, and retained fixtures.

## Evidence produced

- Required adapter metadata
- Required adapter diagnostics
- First adapter candidate families

## Result

Complete. Review packet, context map, and transition input adapters are viable
first implementation candidates.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

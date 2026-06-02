# Pulse 02: Discovery diagnostics and ordering

## Goal

Specify deterministic ordering and fail-closed diagnostics for discovery.

## Engineering decision supported

Discovery can be safe only if manifest order, source order, and duplicate
descriptor behavior are explicit.

## Evidence produced

- `docs/architecture/deterministic-discovery-interface.md`

## Result

Complete. Discovery entries process in manifest order, descriptors preserve
source order, and malformed manifests or duplicate descriptor ids fail closed.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

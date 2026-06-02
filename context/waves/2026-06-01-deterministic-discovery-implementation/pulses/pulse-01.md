# Pulse 01: Manifest model and merge

## Goal

Add core discovery manifest records and deterministic collection merge behavior.

## Engineering decision supported

The core discovery boundary is a product-neutral manifest plus descriptor
collection merge, not source inference or downstream adapter generation.

## Evidence produced

- `DiscoveryManifestDraft`
- `DiscoveryManifestDocument`
- `DiscoveryManifestEntry`
- `DescriptorCollectionDocument::from_discovered_collections`
- Core discovery manifest and merge tests

## Result

Complete. Manifest identity/version and output collection identity/version are
validated, and discovered output preserves source order while rejecting duplicate
descriptor ids.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-core
cargo test --workspace
git diff --check
```

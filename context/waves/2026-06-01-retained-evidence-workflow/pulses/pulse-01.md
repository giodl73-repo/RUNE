# Pulse 01: Evidence bundle model

## Goal

Define a product-neutral collection evidence bundle.

## Engineering decision supported

A single bundle can carry collection inspection, compatibility, inventory, and
neutral generated artifact evidence without introducing downstream vocabulary.

## Evidence produced

- `CollectionEvidenceBundleDocument`
- `CollectionEvidenceBundleDocument::from_collection`
- Core unit test for bundle surfaces

## Result

Complete. The bundle preserves the validated collection, check report,
inventory, generated artifact, selected profile metadata, and portable source
label.

## Validation

```powershell
cargo fmt --check
cargo test -p rune-core
cargo test --workspace
git diff --check
```

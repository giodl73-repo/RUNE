# Pulse 08: Shared profile catalog boundary

## Goal

Move approved profile metadata out of `rune-cli` and into the shared neutral core
profile catalog.

## Engineering decision supported

RUNE profile facts should have one reviewed owner before more generators or
profiles are added. CLI listing and generation should consume the same approved
catalog so profile ids, versions, supported kinds, and output artifact kinds do
not drift.

## Trace links expected

- `crates/rune-core/src/lib.rs`
- `crates/rune-cli/src/main.rs`
- `docs/architecture/generator-profile-interface.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VERIFICATION.md`

## Evidence produced

- `rune-core::ProfileCatalog`
- `rune-core::ProfileDescriptor`
- `rune-core::GeneratedArtifactDocument`
- `ProfileCatalog::approved()` and catalog lookup for the neutral descriptor
  profile.
- CLI `profile list` and `generate` now use the shared catalog.

## Result

Complete with limits. The only approved profile remains
`rune.neutral_descriptor_json@v0`. No external formats, product adapters, or
additional profile mappings were added.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- profile list
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
git diff --check
```

# Pulse 07: Shared descriptor-document boundary

## Goal

Remove the duplicate descriptor JSON model from `rune-cli` and route
fixture-backed inspection/generation through the shared `rune-core` descriptor
document boundary.

## Engineering decision supported

RUNE descriptor validation should live in the neutral core model, not in each CLI
or generator consumer. This keeps inspection, generation, tests, and future
profiles aligned on one reviewed descriptor document shape.

## Trace links expected

- `crates/rune-core/src/lib.rs`
- `crates/rune-cli/src/main.rs`
- `docs/architecture/generator-profile-interface.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VERIFICATION.md`

## Evidence produced

- `rune-core::DescriptorDraft` for fixture/input parsing with optional identity,
  version, and kind fields.
- `DescriptorDraft::validate_with_codes(...)` to produce
  `rune-core::DescriptorDocument` or a caller-specific diagnostic code.
- `rune-cli inspect` and `rune-cli generate` now consume the shared core model.
- Core unit tests for draft validation and missing identity diagnostics.

## Result

Complete with limits. Descriptor document parsing and validation are shared
between CLI inspection and generation. This does not add crate discovery,
external profiles, or product adapters.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- profile list
git diff --check
```

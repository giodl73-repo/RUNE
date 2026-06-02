# Pulse 09: Profile compatibility enforcement

## Goal

Enforce selected-profile compatibility for descriptor version and descriptor
kind before emitting generated artifacts.

## Engineering decision supported

RUNE generation should not treat a descriptor as generateable merely because the
descriptor is syntactically valid. The selected profile must explicitly support
the descriptor version and kind.

## Trace links expected

- `crates/rune-core/src/lib.rs`
- `crates/rune-cli/src/main.rs`
- `crates/rune-cli/tests/generate_cli.rs`
- `crates/rune-cli/tests/fixtures/other_kind_descriptor.json`
- `crates/rune-cli/tests/fixtures/unsupported_version_descriptor.json`
- `docs/architecture/generator-profile-interface.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VERIFICATION.md`

## Evidence produced

- `ProfileDescriptor::validate_descriptor_with_codes(...)`.
- Core tests for unsupported profile kind and unsupported descriptor version.
- CLI fail-closed tests for valid-but-profile-unsupported `other` kind and
  unsupported descriptor version `v9`.

## Result

Complete with limits. Profile compatibility is enforced for version and kind.
Invariant, trace-link, and extension compatibility diagnostics remain future
profile-hardening work.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- profile list
git diff --check
```

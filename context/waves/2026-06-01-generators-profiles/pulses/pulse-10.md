# Pulse 10: Read-only compatibility check

## Goal

Add a check command that validates descriptor/profile compatibility without
emitting generated artifacts.

## Engineering decision supported

RUNE should support CI/review workflows that need compatibility evidence without
producing a durable generated output.

## Trace links expected

- `crates/rune-core/src/lib.rs`
- `crates/rune-cli/src/main.rs`
- `crates/rune-cli/tests/check_cli.rs`
- `crates/rune-cli/tests/fixtures/annotated_customer.check.json`
- `docs/architecture/generator-profile-interface.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VERIFICATION.md`

## Evidence produced

- `rune-core::CheckReportDocument`.
- `rune check --profile rune.neutral_descriptor_json --fixture <path>`.
- Retained compatibility report fixture.
- CLI fail-closed tests for unsupported version and unsupported kind.

## Result

Complete with limits. Compatibility can now be checked without generating an
artifact. The command is still fixture-backed and does not perform crate
discovery.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- profile list
git diff --check
```

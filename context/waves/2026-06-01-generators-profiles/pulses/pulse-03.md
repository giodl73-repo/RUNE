# Pulse 03: Generated artifact verification

## Goal

Verify the neutral descriptor-output profile with retained expected output and
fail-closed diagnostics for missing identity, unsupported profile, and
unsupported descriptor kind.

## Engineering decision supported

Generated artifacts can become validation evidence only when output is
deterministic, identity/version data is preserved, and unsupported concepts are
diagnosed rather than silently dropped.

## Trace links expected

- `docs/architecture/generator-profile-interface.md`
- `docs/vtrace/VERIFICATION.md`
- `docs/vtrace/VALIDATION.md`
- `crates/rune-cli/tests/fixtures/`

## Evidence produced

- Retained neutral generated artifact fixture.
- CLI test comparing exact generated artifact output.
- CLI fail-closed checks for `RUNE-GEN-001` and `RUNE-GEN-003`.
- Unsupported descriptor-kind fixture.

## Result

Complete. Generated artifact verification covers the neutral profile output and
the first required generator diagnostics without adding external formats or
product-specific adapters.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\valid_descriptor.json
git diff --check
```

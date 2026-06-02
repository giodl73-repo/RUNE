# Pulse 11: Profile concept compatibility

## Goal

Make profile compatibility explicit for descriptor concept families, not only
descriptor version and kind.

## Engineering decision supported

Profiles must declare whether they can preserve fields, invariants, trace links,
and extensions. Future lossy profiles must fail closed before generation or
compatibility success.

## Trace links expected

- `crates/rune-core/src/lib.rs`
- `crates/rune-cli/src/main.rs`
- `crates/rune-cli/tests/fixtures/profile_list.json`
- `docs/architecture/generator-profile-interface.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VERIFICATION.md`

## Evidence produced

- `ProfileDescriptor.supported_concepts`.
- Concept-aware `ProfileCompatibilityCodes`.
- Core tests for unsupported invariants, trace links, and extensions.
- Profile catalog output now declares `fields`, `invariants`, `trace_links`, and
  `extensions` support for `rune.neutral_descriptor_json`.

## Result

Complete with limits. Concept compatibility is enforced in the shared profile
validation path. The approved neutral profile supports all first-slice concept
families; no external profile or lossy target was added.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- profile list
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
git diff --check
```

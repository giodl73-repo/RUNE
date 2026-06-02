# Pulse 04: Foundation bakeoff execution

## Goal

Execute the first fixture-backed validation bakeoff using the approved read-only
inspection surface.

## Engineering decision supported

RUNE inspection can make a neutral descriptor easier to inspect and review than
source/prose-only interpretation, while broad adoption and profile generation
remain unvalidated.

## Trace links expected

- `docs/vtrace/BAKEOFF.md`
- `docs/vtrace/VALIDATION.md`
- `docs/architecture/inspection-surface.md`
- `crates/rune-cli/tests/fixtures/valid_descriptor.json`
- `crates/rune-cli/tests/fixtures/valid_descriptor.inspection.json`

## Evidence produced

- Inspection command run over retained fixture.
- Baseline comparison recorded.
- Success criteria marked pass, partial pass, or provisional pass.
- Limitations recorded.

## Result

Complete. The fixture-backed bakeoff validates inspectability, neutrality,
identity/version preservation, and review usefulness for the read-only inspection
surface. It does not validate generators, adapters, arbitrary crate analysis, or
broad adoption.

## Validation

```powershell
cargo run -p rune-cli -- inspect --fixture crates\rune-cli\tests\fixtures\valid_descriptor.json
cargo test --workspace
git diff --check
```

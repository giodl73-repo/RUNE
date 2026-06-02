# Pulse 03: Inspection verification

## Goal

Make `rune inspect --fixture <path>` verification deterministic and reviewable by
retaining fixture input, expected inspection output, and fail-closed diagnostics.

## Engineering decision supported

Inspection output can be retained as evidence for the first bakeoff only if the
CLI behavior is deterministic and covered by pass/fail tests.

## Trace links expected

- `docs/architecture/inspection-surface.md`
- `docs/vtrace/VERIFICATION.md`
- `docs/vtrace/VALIDATION.md`
- `crates/rune-cli/tests/fixtures/`

## Evidence produced

- Retained valid descriptor fixture.
- Retained expected inspection output.
- Retained invalid descriptor fixture.
- CLI tests compare exact inspection output and fail closed on missing identity.

## Result

Complete. Inspection verification now uses retained fixtures and expected output
instead of temp-only test data.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
git diff --check
```

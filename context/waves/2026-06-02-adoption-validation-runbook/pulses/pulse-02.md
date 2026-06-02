# Pulse 02: Runbook indexing and VTRACE closure

## Objective

Link the runbook from the docs map, corpus, how-to index, trace walkthrough, and
VTRACE records.

## Evidence

- `docs\README.md`
- `docs\CORPUS.md`
- `docs\how-to\README.md`
- `docs\traces\adopter-v1-walkthrough.md`
- `PRODUCT_PLAN.md`
- `context\waves\PHASES.md`
- `docs\vtrace\REQUIREMENTS.md`
- `docs\vtrace\TRACE.md`
- `docs\vtrace\VERIFICATION.md`
- `docs\vtrace\VALIDATION.md`
- `docs\vtrace\REVIEW.md`

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
git diff --check
```

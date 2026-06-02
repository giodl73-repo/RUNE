# Pulse 02: Ordered VTRACE stage gates

## Goal

Define every RUNE VTRACE stage in order, review current project status against
each stage, and stop feature expansion until the earlier-stage issues are
resolved.

## Engineering decision supported

RUNE will not treat scaffolded crates, derives, CLI commands, generators, or
downstream scenarios as contract-stable until the relevant VTRACE stage exits
with review evidence.

## Trace links expected

- `docs/vtrace/STAGES.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/REVIEW.md`

## Evidence produced

- Ordered stage map with entry criteria, exit criteria, evidence, and current
  status.
- Foundation review updated with stage results and ordered issues.
- Wave plan updated so the next work follows stage order.

## Validation

```powershell
git diff --check
```

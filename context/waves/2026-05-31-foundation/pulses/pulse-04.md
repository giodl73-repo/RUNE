# Pulse 04: Interface and control spec outline

## Goal

Define the Stage 4 interface-control boundary before treating any scaffolded code
as contract-stable.

## Engineering decision supported

Only the surfaces named in `docs/architecture/interface-control.md` are approved
for the Stage 5 implementation slice.

## Trace links expected

- `docs/architecture/interface-control.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/REVIEW.md`

## Evidence produced

- Controlled descriptor, trait, derive macro, CLI, generator boundary, and
  diagnostics concepts.
- Explicit deferral of inspect/generate/check/profile commands.
- Stage 4 review recorded.

## Result

Complete. Stage 4 closed with generators and adapters deferred.

## Validation

```powershell
git diff --check
```

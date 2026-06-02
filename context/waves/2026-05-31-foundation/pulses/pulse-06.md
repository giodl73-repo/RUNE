# Pulse 06: Verification gap closure

## Goal

Close the foundation verification gap by adding proc-macro compile pass/fail
checks and recording which verification surfaces are covered for the approved
Stage 5 slice.

## Engineering decision supported

RUNE can treat the foundation descriptor and derive slice as verified for
scaffold purposes, while keeping generated artifact checks not applicable until
a generator surface is approved.

## Trace links expected

- `docs/vtrace/VERIFICATION.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/REVIEW.md`

## Evidence produced

- `trybuild` compile-pass fixture for approved `id`, `version`, and `kind`
  attributes.
- `trybuild` compile-fail fixture for unsupported legacy `schema` attribute.
- Verification matrix updated with covered and not-applicable surfaces.

## Result

Complete. Stage 6 verifies the approved foundation slice and keeps generator
artifact checks deferred until generator interfaces exist.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
git diff --check
```

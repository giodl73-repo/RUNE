# Pulse 01: Automation interface gate

## Goal

Define the allowed automation boundary before adding descriptor evidence
generation or regeneration.

## Engineering decision supported

RUNE should close the manual fixture dependency with a deterministic
build/test-oriented path, while still blocking arbitrary crate discovery,
external profiles, and product adapters.

## Trace links expected

- `docs/architecture/derive-evidence-automation.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VALIDATION.md`
- `docs/vtrace/REVIEW.md`

## Decision

The first automation implementation may serialize known annotated contracts to
`rune-core::DescriptorDocument` evidence through tests or a controlled
build/test-oriented example. It must not scan arbitrary crates or emit external
profile formats.

## Result

Complete. The automation interface is gated and ready for a smallest
implementation slice.

## Validation

```powershell
git diff --check
cargo test --workspace
```

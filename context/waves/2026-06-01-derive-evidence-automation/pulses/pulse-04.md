# Pulse 04: Automation readiness decision

## Goal

Decide whether deterministic descriptor evidence regeneration is sufficient for
broad adoption or whether RUNE needs another constrained automation wave first.

## Engineering decision supported

RUNE should move from one known annotated contract to multiple explicitly
registered known contracts before attempting arbitrary crate discovery or
external profile outputs.

## Evidence reviewed

- `docs/architecture/derive-evidence-automation.md`
- `crates/rune-derive/tests/derive_contract.rs`
- `crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json`
- `context/waves/2026-06-01-derive-evidence-automation/pulses/pulse-02.md`
- `context/waves/2026-06-01-derive-evidence-automation/pulses/pulse-03.md`
- `docs/vtrace/VERIFICATION.md`
- `docs/vtrace/VALIDATION.md`

## Decision

| Area | Decision | Reason |
|---|---|---|
| Single-contract evidence regeneration | pass | The retained customer descriptor can be regenerated from the annotated type. |
| Determinism | pass | Repeated descriptor serialization is stable. |
| Manual fixture dependency | reduced | The retained fixture is no longer a parallel hand-maintained descriptor model for the controlled target. |
| Multi-contract evidence collection | blocked | Only one known annotated contract is covered. |
| Arbitrary crate discovery | blocked | No discovery interface or validation exists. |
| External profiles/adapters | blocked | Neutral evidence automation must mature first. |
| Next wave | approved | Add constrained known-contract evidence collection. |

## Result

Complete with limits. Wave 5 proves deterministic regeneration for one known
annotated contract. It does not prove repo-scale evidence collection.

## Validation

```powershell
cargo test --workspace
git diff --check
```

# Pulse 04: Readiness decision

## Goal

Decide whether the controlled repo-adoption bakeoff is enough to broaden RUNE
adoption or whether the next work must close remaining validation gaps.

## Engineering decision supported

RUNE should not move directly to external profiles, product adapters, or broad
repo adoption after a single fixture-backed scenario. The next safe step is
descriptor evidence automation.

## Evidence reviewed

- `context/waves/2026-06-01-repo-adoption-bakeoff/pulses/pulse-02.md`
- `context/waves/2026-06-01-repo-adoption-bakeoff/pulses/pulse-03.md`
- `crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json`
- `crates/rune-cli/tests/fixtures/annotated_customer.check.json`
- `crates/rune-cli/tests/fixtures/annotated_customer.neutral_descriptor_artifact.json`
- `docs/vtrace/BAKEOFF.md`
- `docs/vtrace/VALIDATION.md`

## Decision

| Area | Decision | Reason |
|---|---|---|
| Controlled scenario usefulness | pass | RUNE evidence reduces source-only inference burden for the annotated `Customer` contract. |
| Neutrality | pass | Evidence uses neutral RUNE vocabulary and no product adapter names. |
| Profile mechanics | pass | Check and generated artifact evidence preserve profile identity/version and descriptor compatibility. |
| Broad adoption | blocked | Evidence is still fixture-backed and limited to one annotated type. |
| External profiles | blocked | No external profile should be added before descriptor evidence automation is available. |
| Next wave | approved | Open derive/build-output automation to remove manual fixture dependency. |

## Result

Complete with limits. Wave 4 validates scenario-level usefulness, not broad
adoption readiness.

## Validation

```powershell
cargo test --workspace
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
git diff --check
```

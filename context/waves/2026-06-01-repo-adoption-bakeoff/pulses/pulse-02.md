# Pulse 02: Source-only baseline comparison

## Goal

Capture what a reviewer or AI agent can determine from the annotated Rust source
and tests without using RUNE descriptor, check, or generated artifact evidence.

## Engineering decision supported

The adoption bakeoff needs a fair baseline before claiming RUNE improves review
or AI handoff.

## Source reviewed

- `crates/rune-derive/tests/derive_contract.rs`

## Baseline findings

| Question | Source-only answer | Review burden |
|---|---|---|
| What contract is represented? | `Customer` is visible as a Rust struct name. | Low. |
| What fields exist? | `id: String` and `email: String` are visible. | Low. |
| What stable contract identity should tools use? | `example.customer` is present in an attribute but requires knowing RUNE macro semantics. | Medium. |
| What version is durable? | `v0` is present in an attribute but not validated without macro/test knowledge. | Medium. |
| What contract kind is intended? | `entity` is present in an attribute; meaning depends on RUNE vocabulary. | Medium. |
| What requirement is linked? | `RUNE-REQ-034` is present in an attribute; relation semantics are implicit. | Medium. |
| Are generated descriptor fields complete? | The test assertions reveal expected descriptor shape. | Medium. |
| Is the descriptor compatible with a profile? | Not answerable from source-only review. | High. |
| What artifact would downstream tooling consume? | Not answerable from source-only review. | High. |
| Are unsupported concepts fail-closed? | Not answerable from this source file alone. | High. |

## Result

Complete. Source-only review can identify the Rust type and fields quickly, but
stable identity/version semantics, requirement trace semantics, profile
compatibility, generated artifact shape, and fail-closed behavior require RUNE
macro/test knowledge or generated evidence.

## Validation

```powershell
git diff --check
```

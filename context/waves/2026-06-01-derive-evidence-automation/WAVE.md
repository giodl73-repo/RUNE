# Wave: Derive evidence automation

## Goal

Close the gap between annotated Rust contracts and retained descriptor evidence
by producing or collecting descriptor JSON through a build/test-oriented path
instead of relying on manually maintained CLI fixtures.

## Engineering decision supported

RUNE can move toward broader repo adoption only if descriptor evidence can be
generated deterministically from annotated Rust code. External profiles and
product adapters remain out of scope until this automation exists.

## Scope

This wave may add test/build evidence helpers, fixture generation tests, or a
controlled CLI path for descriptor documents. It must not add arbitrary crate
discovery, JSON Schema, CSDL, OpenAPI, AgentMap, BAKER, LATTICE, or product
adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Automation interface gate | complete |
| 02 | Deterministic descriptor evidence path | complete |
| 03 | Retained evidence regeneration check | complete |
| 04 | Automation readiness decision | complete |

## Readiness decision

Wave 5 passes for deterministic evidence regeneration of one known annotated
contract. It removes the manual parallel-model concern for the annotated
`Customer` descriptor fixture.

RUNE remains blocked for broad adoption and arbitrary crate discovery. The next
safe step is a constrained known-contract evidence collection wave that can
handle multiple explicitly registered annotated contracts without scanning
arbitrary crates or adding external profiles.

## Validation

```powershell
git diff --check
cargo test --workspace
```

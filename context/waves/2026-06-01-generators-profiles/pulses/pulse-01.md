# Pulse 01: Generator/profile interface boundary

## Goal

Define the generator/profile interface before any generation code is added.

## Engineering decision supported

RUNE generation starts with a neutral profile boundary, not with JSON Schema,
CSDL, OpenAPI, AgentMap, or any product-specific adapter.

## Trace links expected

- `docs/architecture/generator-profile-interface.md`
- `docs/architecture/descriptor-model.md`
- `docs/architecture/inspection-surface.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`

## Evidence produced

- Generator input/output contract.
- Profile compatibility rules.
- Diagnostics requirements for unsupported descriptor concepts.
- Explicit non-goals for downstream formats and adapters.

## Result

Complete. `docs/architecture/generator-profile-interface.md` defines the
generator/profile boundary and names a neutral descriptor-output profile as the
first implementation candidate.

## Validation

```powershell
git diff --check
```

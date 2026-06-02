# Pulse 01: Read-only inspection surface

## Goal

Define `rune inspect` as a read-only, neutral descriptor inspection surface that
can later produce validation evidence without adding profile generators.

## Engineering decision supported

Inspection should come before generation. It gives humans and agents a durable
way to view RUNE descriptors while avoiding premature commitment to JSON Schema,
CSDL, OpenAPI, AgentMap, or any other downstream format.

## Trace links expected

- `docs/architecture/inspection-surface.md`
- `docs/architecture/interface-control.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VALIDATION.md`

## Evidence produced

- Inspection command contract.
- Diagnostics and non-goals for inspection.
- Updated requirements and trace rows.

## Result

Complete. `docs/architecture/inspection-surface.md` defines read-only fixture
inspection as the next approved surface before generators or adapters.

## Validation

```powershell
git diff --check
```

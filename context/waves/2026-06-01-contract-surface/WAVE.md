# Wave: Contract surface

## Goal

Define the first durable contract surfaces beyond the foundation scaffold,
starting with a read-only inspection command that can produce evidence for the
first validation bakeoff without introducing generators, adapters, or downstream
product vocabulary.

## Engineering decision supported

RUNE should unlock validation through a minimal neutral inspection surface before
adding generated profiles. Inspection is the safest next step because it exposes
descriptor metadata for review without deciding any target output format.

## Trace links expected

- Mission need: Rust code needs machine-readable contract metadata.
- Requirement: inspected descriptors preserve identity, version, kind, fields,
  invariants, trace links, and extensions.
- Design: `docs/architecture/inspection-surface.md`.
- Implementation: future `rune-cli inspect` slice.
- Verification: CLI command tests and fixture output checks.
- Validation: first bakeoff evidence from retained inspection output.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Read-only inspection surface | complete |
| 02 | Inspection implementation slice | complete |
| 03 | Inspection verification | complete |
| 04 | Foundation bakeoff execution | complete |

## Validation

```powershell
git diff --check
```

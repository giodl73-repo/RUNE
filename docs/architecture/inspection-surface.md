# RUNE Inspection Surface

## Engineering decision supported

Approve `rune inspect` as the next read-only contract surface after the
foundation scaffold. Inspection is not generation: it exposes neutral descriptor
metadata for review, testing, and validation evidence without mapping into any
downstream profile.

## Command contract

The descriptor inspection command shape is:

```powershell
cargo run -p rune-cli -- inspect --fixture <path>
```

The collection inspection command shape is:

```powershell
cargo run -p rune-cli -- inspect-collection --fixture <path>
```

The collection inventory command shape is:

```powershell
cargo run -p rune-cli -- inventory-collection --fixture <path>
```

Foundation implementation may start with a fixture input before crate-wide Rust
analysis exists. The fixture must represent RUNE `ContractDescriptor` data, not a
target schema profile.

## Output contract

Descriptor inspection output must be neutral and preserve:

| Field | Requirement |
|---|---|
| `id` | Stable descriptor identity. |
| `version` | Descriptor compatibility boundary. |
| `kind` | Foundation contract kind. |
| `rust_type` | Rust item represented by the descriptor. |
| `fields` | Named field descriptors when available. |
| `invariants` | Invariant descriptors when supplied. |
| `trace_links` | Need, requirement, design, implementation, verification, validation, or evidence links when supplied. |
| `extensions` | Namespaced extension data without core vocabulary expansion. |

Collection inspection output must be neutral and preserve:

| Field | Requirement |
|---|---|
| `collection_id` | Stable collection identity. |
| `collection_version` | Collection compatibility boundary. |
| `descriptors` | Ordered descriptor documents. |

Collection inventory output must be neutral and preserve:

| Field | Requirement |
|---|---|
| `collection_id` | Stable collection identity. |
| `collection_version` | Collection compatibility boundary. |
| `descriptor_count` | Total validated descriptor count. |
| `kinds` | Deterministic descriptor counts grouped by kind. |

## Diagnostics

`rune inspect` should fail closed when required descriptor data is missing or
unsupported. Initial diagnostic expectations:

| Code | Severity | Meaning |
|---|---|---|
| `RUNE-INSP-001` | error | Descriptor identity is missing. |
| `RUNE-INSP-002` | error | Descriptor version is missing. |
| `RUNE-INSP-003` | error | Descriptor kind is unsupported. |
| `RUNE-INSP-004` | warning/error | Extension namespace is unsupported by the selected inspection mode. |
| `RUNE-COLL-INSP-001` | error | Descriptor collection identity is missing. |
| `RUNE-COLL-INSP-002` | error | Descriptor collection version is missing. |
| `RUNE-COLL-INSP-003` | error | Descriptor collection contains duplicate descriptor ids. |
| `RUNE-COLL-INV-001` | error | Descriptor collection identity is missing. |
| `RUNE-COLL-INV-002` | error | Descriptor collection version is missing. |
| `RUNE-COLL-INV-003` | error | Descriptor collection contains duplicate descriptor ids. |

Stage 2 implementation may keep diagnostics simple, but it must not silently
emit success-shaped output for missing identity, missing version, or unsupported
kind.

## Non-goals

- Do not implement profile generators.
- Do not emit JSON Schema, CSDL, OpenAPI, AgentMap, or product-specific maps.
- Do not inspect arbitrary Rust crates through compiler analysis yet.
- Do not discover or build collection inputs from crates; collection inspection
  and inventory remain fixture-backed.
- Do not use inspection output as validation evidence until verification checks
  retain deterministic fixture output.

## Validation path

1. Specify fixture shape.
2. Implement read-only fixture inspection.
3. Add pass/fail CLI tests.
4. Retain deterministic inspection output as validation evidence.
5. Run the first bakeoff scenario from `docs/vtrace/VALIDATION.md`.

## Validation command

```powershell
git diff --check
```

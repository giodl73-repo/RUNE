# Capability and sensitivity policy

## Purpose

Capability and sensitivity policy defines the metadata required before RUNE can
expose private data, mutating operations, runtime host endpoints, or agent-facing
query surfaces.

The first implementation should be descriptive and fail-closed. Enforcement
requires a later runtime host implementation DCR.

## Policy concepts

| Concept | Purpose |
|---|---|
| Sensitivity | public, internal, private, restricted, secret, or domain-specific policy value. |
| Exportability | whether evidence may leave the current crate/process/workspace boundary. |
| Mutability | read-only, preview-only, mutating, destructive. |
| Authority | required actor/tool/host authority to perform an operation. |
| Stability | experimental, stable, deprecated, removed. |
| Retention | whether data may be retained as evidence. |

## Policy document shape

First planning target:

| Field | Purpose |
|---|---|
| `policy_id` | Durable policy identity. |
| `policy_version` | Policy document version. |
| `scope_ref` | Registry, collection, descriptor, field, operation, or packet reference. |
| `sensitivity` | Declared sensitivity. |
| `exportability` | Export and retention rule. |
| `mutability` | Operation mutability classification. |
| `authority` | Required authority declaration. |
| `stability` | Compatibility expectation. |
| `diagnostics` | Fail-closed policy diagnostics. |

## Diagnostics

Reserve diagnostic families:

| Code | Meaning |
|---|---|
| `RUNE-POLICY-001` | Missing policy identity. |
| `RUNE-POLICY-002` | Unknown policy scope reference. |
| `RUNE-POLICY-003` | Unsupported sensitivity or mutability value. |
| `RUNE-POLICY-004` | Export requested for non-exportable data. |
| `RUNE-POLICY-005` | Mutating operation requested without authority. |
| `RUNE-POLICY-006` | Runtime enforcement requested before runtime host approval. |

## Retained fixtures

Implementation must add:

- public read-only policy fixture,
- private non-exportable policy fixture,
- mutating operation blocked fixture,
- unknown scope failure fixture.

## Non-goals

- No runtime enforcement in the first policy document implementation.
- No secret scanning.
- No authority provider integration.
- No mutating agent operations.


# RUNE Deterministic Discovery Interface

## Engineering decision supported

Define a deterministic discovery boundary before implementing any repo-scale
collection workflow.

## Problem

RUNE now supports adopter-owned explicit registries, but v1 also requires a
discovery path. Discovery must not mean arbitrary source scraping, opportunistic
Cargo metadata traversal, linker-section inventory, or downstream adapter
loading. It must be reviewable, deterministic, and evidence-oriented.

## Approved discovery model

The first discovery model is **manifest-based discovery**.

An adopter supplies a small RUNE discovery manifest that names:

| Field | Meaning |
|---|---|
| `manifest_id` | Stable identity for the discovery manifest. |
| `manifest_version` | Manifest compatibility boundary. |
| `collection_id` | Stable output descriptor collection identity. |
| `collection_version` | Output descriptor collection compatibility boundary. |
| `entries` | Ordered discovery entries. |

Each entry names one deterministic source of descriptor evidence:

| Field | Meaning |
|---|---|
| `name` | Human-readable entry name. |
| `source_kind` | Discovery source family. |
| `source` | Source path, module path, or future registry hook identifier. |

The first implementation candidate should support retained descriptor collection
fixture entries before executable registry hooks:

```json
{
  "manifest_id": "example.discovery",
  "manifest_version": "v0",
  "collection_id": "example.discovered_contracts",
  "collection_version": "v0",
  "entries": [
    {
      "name": "adopter-contracts",
      "source_kind": "descriptor_collection_fixture",
      "source": "examples/rune-adopter/tests/fixtures/adopter_contract_collection.json"
    }
  ]
}
```

This keeps discovery deterministic while allowing later implementation to merge
known retained collection evidence. Future registry-hook support may be added
only after it has a separate execution and safety review.

## Ordering rules

1. Manifest entries are processed in file order.
2. Descriptors inside collection fixtures preserve their collection order.
3. Merged descriptor output preserves manifest order, then source order.
4. Duplicate descriptor ids fail closed.
5. Missing manifest identity or version fails closed.
6. Missing collection identity or version fails closed.
7. Unknown `source_kind` fails closed.

## Diagnostic families

| Code | Meaning |
|---|---|
| `RUNE-DISC-001` | Discovery manifest identity is missing. |
| `RUNE-DISC-002` | Discovery manifest version is missing. |
| `RUNE-DISC-003` | Discovery output collection identity is missing. |
| `RUNE-DISC-004` | Discovery output collection version is missing. |
| `RUNE-DISC-005` | Discovery entry source kind is unsupported. |
| `RUNE-DISC-006` | Discovery entry source cannot be read or parsed. |
| `RUNE-DISC-007` | Discovery output contains duplicate descriptor ids. |
| `RUNE-DISC-900` | Discovery input or output cannot be read, parsed, or serialized. |

## Non-goals

- No arbitrary Rust source scraping.
- No Cargo metadata traversal.
- No proc-macro inventory, linker-section inventory, or plugin loading.
- No execution of downstream adapters.
- No external profile generation.
- No product-specific vocabulary in discovery manifests.

## Implementation gate

The next implementation wave may add:

- discovery manifest data structures,
- fixture-backed manifest parsing,
- deterministic merge of descriptor collection fixtures,
- retained discovery output fixtures,
- fail-closed CLI tests.

It must not add executable registry hooks, source analysis, Cargo metadata
traversal, external profiles, or adapters.

## Validation command

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

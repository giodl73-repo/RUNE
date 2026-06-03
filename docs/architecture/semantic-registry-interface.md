# Semantic registry interface

## Purpose

The semantic registry is the first Mission 2.0 implementation target. It gives a
crate, workspace, process, or retained evidence bundle a deterministic index of
the RUNE contract surfaces it exposes.

The registry answers:

- which descriptor collections exist,
- which crate or component owns them,
- which profile and adapter surfaces are declared,
- which versions are compatible,
- which runtime or agent surfaces are intentionally absent.

## Approved inputs

- `DescriptorCollectionDocument`
- `CollectionEvidenceBundleDocument`
- profile catalog entries
- adapter catalog entries
- optional static registry metadata authored by the crate or process

The registry must not inspect Rust source, traverse Cargo metadata implicitly, or
execute crate hooks.

## Registry document shape

First planning target:

| Field | Purpose |
|---|---|
| `registry_id` | Durable id for the crate, workspace, process, or retained bundle registry. |
| `registry_version` | Version of the registry document shape. |
| `owner` | Human or crate/workspace owner string. |
| `scope` | `crate`, `workspace`, `process`, or `retained_bundle`. |
| `collections` | Ordered descriptor collection references with ids, versions, source refs, and ownership. |
| `profiles` | Declared supported profile ids and versions. |
| `adapters` | Declared supported adapter ids and versions. |
| `capabilities` | Declared read/query/generate/mutate/runtime capability flags. |
| `diagnostics` | Fail-closed registry diagnostics. |

## Diagnostics

Reserve diagnostic families:

| Code | Meaning |
|---|---|
| `RUNE-REGISTRY-001` | Missing registry identity. |
| `RUNE-REGISTRY-002` | Missing registry version. |
| `RUNE-REGISTRY-003` | Duplicate collection id/version entry. |
| `RUNE-REGISTRY-004` | Unsupported registry scope. |
| `RUNE-REGISTRY-005` | Referenced collection is malformed or incompatible. |
| `RUNE-REGISTRY-006` | Declared profile or adapter is unknown. |
| `RUNE-REGISTRY-007` | Runtime capability declared without approved runtime host boundary. |

## Retained fixtures

Implementation must add:

- one minimal crate registry fixture,
- one multi-collection workspace registry fixture,
- one fail-closed duplicate collection fixture,
- one fixture proving runtime capabilities default to absent.

## Non-goals

- No live runtime host.
- No Cargo graph scanning.
- No source scraping.
- No plugin discovery.
- No mutating agent protocol.

## First implementation status

The first implementation slice is complete for retained semantic registry
documents:

- `SemanticRegistryDraft`
- `SemanticRegistryDocument`
- collection, profile, adapter, and capability reference records
- fail-closed validation for missing identity, missing version, duplicate
  collection references, unsupported scope, and runtime capability declarations
- retained fixtures for crate, workspace, duplicate collection, and runtime
  blocked scenarios

This slice does not add runtime host behavior, Cargo traversal, or source
scraping.

## Read-only CLI status

`rune check-registry --fixture <path>` validates retained semantic registry JSON
and emits a compact check report. The command also cross-checks declared
profile and adapter references against approved catalogs.

The command remains read-only. It does not load collection source refs, traverse
Cargo metadata, scrape Rust source, discover plugins, mutate registry state, or
enable runtime host behavior.

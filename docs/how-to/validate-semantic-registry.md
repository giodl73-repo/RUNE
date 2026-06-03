# Validate a semantic registry

Use this workflow when a crate, workspace, process, or retained evidence bundle
has authored semantic registry metadata and you need to prove it is coherent.

## Inputs

| Input | Example |
|---|---|
| Semantic registry fixture | `crates\rune-cli\tests\fixtures\semantic_registry_workspace.json` |
| Referenced collection fixtures | `known_contract_descriptor_collection.json`, `semantic_registry_adapter_contracts_collection.json` |

Collection source refs are resolved relative to the semantic registry fixture.

## Check the registry

```powershell
cargo run -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

The check report confirms:

- registry identity and version,
- approved scope,
- collection/profile/adapter counts,
- capability flags,
- approved profile and adapter references,
- retained collection source refs.

## Inspect registry evidence

```powershell
cargo run -p rune-cli -- inspect-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

The inspection report preserves the validated registry metadata and summarizes
each retained collection source ref, owner, descriptor count, and descriptor-kind
inventory.

## Fail-closed diagnostics

| Diagnostic | Meaning |
|---|---|
| `RUNE-REGISTRY-001` | Missing registry identity. |
| `RUNE-REGISTRY-002` | Missing registry version. |
| `RUNE-REGISTRY-003` | Duplicate collection id/version ref. |
| `RUNE-REGISTRY-004` | Unsupported registry scope. |
| `RUNE-REGISTRY-005` | Collection source ref is unreadable, malformed, or mismatched. |
| `RUNE-REGISTRY-006` | Unknown or unsupported profile/adapter ref. |
| `RUNE-REGISTRY-007` | Runtime capability was declared without an approved runtime host boundary. |

## Boundaries

Registry validation is read-only. It does not inspect Cargo metadata, scrape Rust
source, discover plugins, mutate registry state, expose live state, or enable a
runtime host.


# Semantic registry validation runbook

## Purpose

Validate that retained semantic registry evidence is coherent before downstream
adopters or future Mission 2.0 lanes consume it.

Run this from the RUNE repository root.

## Inputs

| Input | Path |
|---|---|
| Workspace registry fixture | `crates\rune-cli\tests\fixtures\semantic_registry_workspace.json` |
| Registry check fixture | `crates\rune-cli\tests\fixtures\semantic_registry_workspace.check.json` |
| Registry inspection fixture | `crates\rune-cli\tests\fixtures\semantic_registry_workspace.inspect.json` |
| Duplicate collection failure fixture | `crates\rune-cli\tests\fixtures\semantic_registry_duplicate_collection.json` |
| Source-ref mismatch failure fixture | `crates\rune-cli\tests\fixtures\semantic_registry_mismatched_collection.json` |
| Runtime blocked failure fixture | `crates\rune-cli\tests\fixtures\semantic_registry_runtime_blocked.json` |

## 1. Check the registry

```powershell
cargo run -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

Compare the output to:

```text
crates\rune-cli\tests\fixtures\semantic_registry_workspace.check.json
```

## 2. Inspect registry summaries

```powershell
cargo run -p rune-cli -- inspect-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

Compare the output to:

```text
crates\rune-cli\tests\fixtures\semantic_registry_workspace.inspect.json
```

## 3. Verify fail-closed cases

```powershell
cargo test -p rune-cli --test registry_cli
```

The test suite covers duplicate collection refs, runtime capability rejection,
unknown profile refs, source-ref mismatches, and invalid usage.

## Pass criteria

- Registry check and inspection commands exit successfully for the workspace
  registry fixture.
- Collection source refs resolve to retained descriptor collection fixtures.
- Profile and adapter references are approved catalog entries.
- Runtime capability remains blocked.
- Failure fixtures fail with the expected `RUNE-REGISTRY-*` diagnostic families.

## Boundaries

This runbook validates retained evidence only. It does not approve Cargo
metadata traversal, source scraping, plugin discovery, registry mutation, live
state inspection, runtime hosts, or automatic migration.


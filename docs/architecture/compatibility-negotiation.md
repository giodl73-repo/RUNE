# Compatibility negotiation

## Purpose

Compatibility negotiation expands RUNE's v1 profile checks into a broader
evidence model across descriptor collections, profiles, adapters, semantic
registries, state graphs, agent protocol versions, and optional runtime hosts.

Compatibility is evidence. It must not silently migrate or degrade behavior.

## Negotiation scopes

| Scope | Question |
|---|---|
| Collection/Profile | Can this profile represent the descriptor concepts? |
| Collection/Adapter | Can this adapter consume the collection and emit its artifact? |
| Registry/Registry | Are two registries compatible or comparable? |
| Registry/State Graph | Can this state graph be interpreted by the registry? |
| Protocol/Registry | Can this agent protocol operation access the registry safely? |
| Runtime Host/Registry | Can a runtime host expose this registry and capabilities? |

## Report document shape

Implemented retained report target:

| Field | Purpose |
|---|---|
| `compatibility_report_id` | Durable report identity. |
| `compatibility_report_version` | Report document version. |
| `source_ref` | Source collection, registry, state graph, protocol, or host. |
| `target_ref` | Target profile, adapter, registry, protocol, or host. |
| `status` | compatible, compatible_with_warnings, incompatible, blocked. |
| `supported_concepts` | Concepts accepted by target. |
| `unsupported_concepts` | Concepts rejected or blocked. |
| `degraded_concepts` | Concepts requiring explicit degraded behavior. |
| `diagnostics` | Fail-closed compatibility diagnostics. |
| `automatic_migration_requested` | Must remain false; automatic migration is blocked. |

## Diagnostics

Reserve diagnostic families:

| Code | Meaning |
|---|---|
| `RUNE-COMPAT-001` | Unknown source artifact. |
| `RUNE-COMPAT-002` | Unknown target artifact. |
| `RUNE-COMPAT-003` | Unsupported source or target version. |
| `RUNE-COMPAT-004` | Required concept is unsupported. |
| `RUNE-COMPAT-005` | Degraded behavior requested without explicit approval. |
| `RUNE-COMPAT-006` | Runtime host compatibility requested before host DCR approval. |

## Retained fixtures

Implementation adds:

- compatible collection/profile report,
- incompatible collection/adapter report,
- registry/state graph report,
- degraded behavior blocked report,
- runtime host compatibility blocked report.

## CLI

```powershell
cargo run -p rune-cli -- check-compatibility --fixture crates\rune-cli\tests\fixtures\compatibility_collection_profile.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

The command validates retained report evidence only. It does not load runtime
hosts, execute migrations, inspect live state, mutate data, scrape source, or
enforce policy.

## Non-goals

- No automatic migration.
- No best-effort conversion.
- No runtime host implementation.
- No compatibility claims without retained report evidence.

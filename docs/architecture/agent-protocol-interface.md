# Agent protocol interface

## Purpose

The agent protocol gives AI tools a safe, explicit, read-first way to query RUNE
registries, descriptors, evidence, compatibility reports, and approved generated
artifacts.

Mutating operations are out of scope until capability policy and runtime host
boundaries are implemented and reviewed.

## Approved read operations

Implemented retained read-first operations:

| Operation | Purpose |
|---|---|
| `registry.describe` | Return semantic registry metadata and declared capabilities. |
| `collection.list` | List descriptor collections in registry order. |
| `descriptor.get` | Return a descriptor by id/version. |
| `evidence.list` | List retained evidence packets and bundles. |
| `compatibility.check` | Return compatibility evidence for selected profiles/adapters/hosts. |
| `profile.generate.preview` | Produce non-mutating generated output preview where an approved profile exists. |
| `adapter.generate.preview` | Produce non-mutating adapter output preview where an approved adapter exists. |

## Required protocol metadata

| Field | Purpose |
|---|---|
| `protocol_id` | Stable protocol id. |
| `protocol_version` | Protocol document version. |
| `operation` | Requested operation id. |
| `capability_ref` | Capability required by the operation. |
| `input_refs` | Registry, collection, descriptor, profile, adapter, or evidence ids. |
| `result` | Operation-specific read-only result. |
| `diagnostics` | Fail-closed protocol diagnostics. |
| `restricted_data_requested` | Retained test flag for blocked restricted-data requests. |

## Diagnostics

Reserve diagnostic families:

| Code | Meaning |
|---|---|
| `RUNE-AGENT-001` | Unknown operation. |
| `RUNE-AGENT-002` | Missing required capability. |
| `RUNE-AGENT-003` | Operation is mutating and not approved. |
| `RUNE-AGENT-004` | Requested descriptor, evidence, profile, or adapter is unknown. |
| `RUNE-AGENT-005` | Requested output would expose restricted data. |

## Retained fixtures

Implementation must add:

- `registry.describe` success fixture,
- `descriptor.get` success fixture,
- `compatibility.check` success fixture,
- unknown operation failure fixture,
- mutating operation blocked fixture.

Current implementation also retains missing capability, unknown reference, and
restricted-data blocked fixtures.

## Implemented command

```powershell
cargo run -q -p rune-cli -- check-agent-protocol --fixture crates\rune-cli\tests\fixtures\agent_protocol_registry_describe.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

The command validates retained protocol request JSON against a retained semantic
registry and the registry's descriptor collection source refs. It emits a small
check report; it does not start a server, expose live endpoints, run tools, or
authorize mutation.

## Non-goals

- No autonomous mutation.
- No prompt-only authority model.
- No hidden source scraping.
- No live runtime endpoint until runtime host design is approved.

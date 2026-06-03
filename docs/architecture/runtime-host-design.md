# Optional runtime host design

## Purpose

The runtime host is the final Mission 2.0 lane. It would be an optional native
Rust component that exposes approved RUNE semantic surfaces from a running
process.

It is not approved for implementation until semantic registry, state graph,
evidence runtime packets, agent protocol, compatibility negotiation, and
capability/sensitivity policy have approved boundaries and retained fixtures.

## Candidate host surfaces

| Surface | Dependency |
|---|---|
| Registry endpoint | Semantic registry interface. |
| State graph endpoint | State graph interface and policy metadata. |
| Evidence endpoint | Evidence runtime packet interface. |
| Agent query endpoint | Agent protocol and capability policy. |
| Compatibility endpoint | Compatibility negotiation reports. |
| Policy decision endpoint | Capability and sensitivity policy. |

## Host document shape

First planning target:

| Field | Purpose |
|---|---|
| `host_id` | Durable runtime host identity. |
| `host_version` | Host document/API version. |
| `registry_ref` | Exposed semantic registry. |
| `enabled_surfaces` | Explicitly enabled endpoint families. |
| `capability_policy_ref` | Policy document governing exposure. |
| `transport` | local-file, stdio, in-process, local-http, or future reviewed transport. |
| `diagnostics` | Fail-closed host diagnostics. |

## Diagnostics

Reserve diagnostic families:

| Code | Meaning |
|---|---|
| `RUNE-HOST-001` | Missing host identity. |
| `RUNE-HOST-002` | Surface enabled without approved dependency lane. |
| `RUNE-HOST-003` | Missing capability policy. |
| `RUNE-HOST-004` | Unsupported transport. |
| `RUNE-HOST-005` | Live state endpoint requested without state graph approval. |
| `RUNE-HOST-006` | Mutating operation requested without agent protocol and policy approval. |

## Retained fixtures

Design implementation must add:

- disabled host declaration fixture,
- registry-only host fixture,
- unsupported transport failure fixture,
- surface-without-policy failure fixture.

## Non-goals

- No VM, garbage collector, async runtime, plugin loader, or OS service.
- No network exposure by default.
- No live state inspection by default.
- No mutating operations by default.
- No product-specific host behavior in `rune-core`.


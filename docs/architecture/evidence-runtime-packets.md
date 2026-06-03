# Evidence runtime packets

## Purpose

Evidence runtime packets give RUNE a structured way to describe diagnostics,
validation results, traces, health checks, and audit events tied to descriptor
ids and semantic registries.

They are not a logging framework. They are retained, machine-readable evidence
artifacts that can be emitted by tests, tools, or future runtime hosts.

## Approved packet families

| Packet | Purpose |
|---|---|
| Diagnostic packet | Explain unsupported concepts, malformed evidence, or blocked operations. |
| Validation packet | Record pass/fail/blocked checks over descriptors, registries, state graphs, or adapters. |
| Trace packet | Link runtime or tool events to descriptor ids and trace links. |
| Health packet | Summarize readiness or degraded state of a semantic registry or runtime host. |
| Audit packet | Record agent/tool access attempts, capability checks, and policy decisions. |

## Packet document shape

First planning target:

| Field | Purpose |
|---|---|
| `packet_id` | Durable packet identity. |
| `packet_version` | Version of the packet document shape. |
| `packet_kind` | diagnostic, validation, trace, health, or audit. |
| `registry_ref` | Optional semantic registry id/version. |
| `descriptor_refs` | Descriptor ids and versions involved. |
| `severity` | info, warning, error, blocked. |
| `status` | pass, fail, blocked, degraded, observed. |
| `message` | Human-readable summary. |
| `evidence_refs` | Retained evidence links. |
| `diagnostics` | Packet construction diagnostics. |

## Diagnostics

Reserve diagnostic families:

| Code | Meaning |
|---|---|
| `RUNE-EVIDENCE-001` | Missing packet identity. |
| `RUNE-EVIDENCE-002` | Unsupported packet kind. |
| `RUNE-EVIDENCE-003` | Unknown descriptor reference. |
| `RUNE-EVIDENCE-004` | Unsupported severity or status. |
| `RUNE-EVIDENCE-005` | Audit packet missing capability decision. |

## Retained fixtures

Implementation must add one retained packet fixture per approved packet kind and
one failure fixture for an unknown descriptor reference.

## Non-goals

- No replacement for `tracing`, logs, metrics, or observability backends.
- No live runtime host requirement.
- No private payload capture by default.
- No mutation authorization.


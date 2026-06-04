# Mission 2.0 planning index

Mission 2.0 turns RUNE from v1 contract infrastructure into a managed native
semantic runtime direction. This index records the planning package and the
current DCR-gated implementation status.

## Lane order

| Wave | Lane | Planning artifact | Implementation readiness |
|---:|---|---|---|
| 42 | Semantic registry interface | `semantic-registry-interface.md` | implemented by DCR-RUNE-003 as read-only retained registry evidence |
| 43 | State graph interface | `state-graph-interface.md` | implemented and role-review hardened by DCR-RUNE-004 as retained validation; live state remains blocked |
| 44 | Evidence runtime packets | `evidence-runtime-packets.md` | implemented by DCR-RUNE-005 as retained packet validation; logging/runtime behavior remains blocked |
| 45 | Agent protocol interface | `agent-protocol-interface.md` | implemented by DCR-RUNE-006 as retained read-first protocol validation; live endpoints and mutation remain blocked |
| 46 | Compatibility negotiation | `compatibility-negotiation.md` | next planned lane; requires collection/profile/adapter/runtime-host report shapes |
| 47 | Capability and sensitivity policy | `capability-sensitivity-policy.md` | required before private data or mutation exposure |
| 48 | Optional runtime host design | `runtime-host-design.md` | blocked until prior lanes are approved |

## Planning completion rule

Planning is complete when every lane has:

1. a purpose,
2. approved input and output boundaries,
3. fail-closed diagnostics,
4. non-goals,
5. retained fixture expectations,
6. explicit implementation blockers.

Mission 2.0 planning does not approve runtime behavior. Runtime host, live state
inspection, mutating agent operations, automatic migration, and policy
enforcement remain blocked until implementation DCRs add code-specific validation.

## Implemented DCR packages

| DCR | Wave | Package | Status |
|---|---:|---|---|
| DCR-RUNE-003 | 42 | Semantic registry model, retained registry fixtures, `check-registry`, `inspect-registry`, catalog checks, retained collection source-ref validation, communications/readiness docs. | complete |
| DCR-RUNE-004 | 43 | Retained state graph model, `check-state-graph`, retained evidence ref validation, ownership ref validation, duplicate graph-id diagnostics, live-state blocking, how-to/runbook, VTRACE records. | complete and role-review hardened |
| DCR-RUNE-005 | 44 | Retained evidence runtime packet model, `check-evidence-packet`, diagnostic/validation/trace/health/audit fixtures, descriptor/evidence ref validation, audit capability decision checks, how-to/runbook, VTRACE records. | complete |
| DCR-RUNE-006 | 45 | Retained read-first agent protocol model, `check-agent-protocol`, registry/descriptor/compatibility fixtures, capability/ref validation, mutating-operation and restricted-data blocking, how-to/runbook, VTRACE records. | complete |

Next package: DCR for Wave 46 compatibility negotiation. It must stay retained
evidence-first and must not introduce automatic migration, policy enforcement,
live inspection, mutation, or runtime host behavior.

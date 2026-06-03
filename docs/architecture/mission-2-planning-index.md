# Mission 2.0 planning index

Mission 2.0 turns RUNE from v1 contract infrastructure into a managed native
semantic runtime direction. This index records the full planning package before
implementation begins.

## Lane order

| Wave | Lane | Planning artifact | Implementation readiness |
|---:|---|---|---|
| 42 | Semantic registry interface | `semantic-registry-interface.md` | first implementable planning target |
| 43 | State graph interface | `state-graph-interface.md` | requires semantic registry identity model |
| 44 | Evidence runtime packets | `evidence-runtime-packets.md` | requires descriptor ids and packet diagnostics |
| 45 | Agent protocol interface | `agent-protocol-interface.md` | requires registry, evidence packets, and policy declarations |
| 46 | Compatibility negotiation | `compatibility-negotiation.md` | requires collection/profile/adapter/runtime-host report shapes |
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


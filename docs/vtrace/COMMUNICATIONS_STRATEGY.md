# RUNE Communications Strategy

## Purpose

This artifact maps accepted RUNE VTRACE intent to user-facing docs surfaces.
The docs package explains descriptors, evidence/discovery, profiles/adapters,
adopter workflow, examples, and retained traces without coupling RUNE core to a
downstream product vocabulary.

## Surface Plan

| Surface ID | Source IDs | Audience | User Question | Generated Docs | Cadence | Owner | Status |
|---|---|---|---|---|---|---|---|
| COMMS-RUNE-README-001 | NEED-001 / RUNE-REQ-069 / RUNE-REQ-072 | Rust maintainer / adopter | Where do I start, and which docs exist? | `docs/README.md` | every docs wave | RUNE maintainer | accepted |
| COMMS-RUNE-CONCEPTS-001 | NEED-002 / RUNE-REQ-010 / RUNE-REQ-012 / RUNE-REQ-013 | AI tooling author / assurance reviewer | What are descriptors, trace links, and neutral contract evidence? | `docs/concepts/` | when descriptor or evidence semantics change | RUNE core owner | accepted |
| COMMS-RUNE-HOWTO-001 | RUNE-REQ-060 / RUNE-REQ-062 / RUNE-REQ-063 | Rust maintainer / adopter | How do I register contracts, use discovery, and generate retained evidence? | `docs/how-to/` | when adopter workflow commands change | RUNE adopter-docs owner | accepted |
| COMMS-RUNE-TUTORIALS-001 | RUNE-REQ-069 / RUNE-REQ-071 / RUNE-REQ-072 | new adopter | How do I learn the v1 adopter path end to end? | `docs/tutorials/adopter-path/` | when the adopter path changes | RUNE tutorial owner | accepted |
| COMMS-RUNE-EXAMPLES-001 | RUNE-REQ-055 / RUNE-REQ-056 / RUNE-REQ-057 / RUNE-REQ-065 / RUNE-REQ-067 | implementer / integrator | What can I copy and what output should I expect? | `docs/examples/`, retained fixtures in `docs/vtrace/fixtures/` | when profile, adapter, or fixture outputs change | RUNE examples owner | accepted |
| COMMS-RUNE-TRACES-001 | RUNE-REQ-070 / RUNE-REQ-071 / RUNE-REQ-072 | reviewer / future agent | How did RUNE prove usefulness and release readiness? | `docs/traces/`, `docs/vtrace/BAKEOFF.md`, `docs/release-readiness.md` | when bakeoff or release evidence changes | RUNE evidence owner | accepted |
| COMMS-RUNE-CORPUS-001 | RUNE-REQ-072 / REVIEW.md | docs owner / future agent | Who owns each docs surface and update obligation? | `docs/CORPUS.md` | every docs wave | RUNE docs owner | accepted |

## Review Checklist

| Item | Required | Decision | Evidence / Rationale |
|---|---|---|---|
| Docs claims trace to controlled source IDs. | yes | accepted | Rows cite stakeholder needs and accepted RUNE requirements. |
| Concepts/tutorials/examples do not overclaim unvalidated behavior. | yes | accepted | Rows keep generated docs, profiles, adapters, and bakeoff evidence tied to retained proof. |
| Public interfaces have expected usage or expected output docs. | if applicable | accepted | How-to, examples, tutorial, and trace rows map to CLI/profile/adapter workflows. |
| `docs/CORPUS.md` names ownership and update obligations. | if multiple surfaces exist | accepted | The existing corpus surface is part of the planned docs package. |

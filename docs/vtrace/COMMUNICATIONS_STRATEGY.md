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
| COMMS-RUNE-REGISTRY-001 | RUNE-REQ-085 / RUNE-REQ-086 / RUNE-REQ-087 / RUNE-REQ-088 | Rust maintainer / adopter / future agent | How do I validate and inspect semantic registry evidence? | `docs/how-to/validate-semantic-registry.md`, `docs/runbooks/semantic-registry-validation.md`, `docs/examples/semantic-registry.md`, `docs/traces/semantic-registry-walkthrough.md` | when registry commands or fixtures change | RUNE registry owner | accepted |
| COMMS-RUNE-STATE-001 | RUNE-REQ-079 / RUNE-REQ-090 | Rust maintainer / future agent | How do I validate retained state graph evidence? | `docs/how-to/validate-state-graph.md`, `docs/runbooks/state-graph-validation.md`, state graph fixtures | when state graph commands or fixtures change | RUNE state graph owner | accepted |
| COMMS-RUNE-EVIDENCE-001 | RUNE-REQ-080 / RUNE-REQ-091 | Rust maintainer / future agent | How do I validate retained evidence runtime packets? | `docs/how-to/validate-evidence-packet.md`, `docs/runbooks/evidence-packet-validation.md`, evidence packet fixtures | when evidence packet commands or fixtures change | RUNE evidence owner | accepted |
| COMMS-RUNE-AGENT-001 | RUNE-REQ-081 / RUNE-REQ-092 | Rust maintainer / future agent | How do I validate retained read-first agent protocol requests? | `docs/how-to/validate-agent-protocol.md`, `docs/runbooks/agent-protocol-validation.md`, agent protocol fixtures | when agent protocol commands or fixtures change | RUNE agent protocol owner | accepted |
| COMMS-RUNE-COMPAT-001 | RUNE-REQ-082 / RUNE-REQ-093 | Rust maintainer / future agent | How do I validate retained compatibility reports? | `docs/how-to/validate-compatibility.md`, `docs/runbooks/compatibility-validation.md`, compatibility fixtures | when compatibility commands or fixtures change | RUNE compatibility owner | accepted |
| COMMS-RUNE-BOUNDARY-001 | RUNE-REQ-084 / DCR-RUNE-002 / DCR-RUNE-003 / DCR-RUNE-004 / DCR-RUNE-005 / DCR-RUNE-006 / DCR-RUNE-007 | reviewer / platform owner | What is still blocked after Wave 46? | README, release readiness, architecture docs, VTRACE review | every Mission 2.0 lane | RUNE maintainer | accepted |

## Review Checklist

| Item | Required | Decision | Evidence / Rationale |
|---|---|---|---|
| Docs claims trace to controlled source IDs. | yes | accepted | Rows cite stakeholder needs and accepted RUNE requirements. |
| Concepts/tutorials/examples do not overclaim unvalidated behavior. | yes | accepted | Rows keep generated docs, profiles, adapters, and bakeoff evidence tied to retained proof. |
| Public interfaces have expected usage or expected output docs. | if applicable | accepted | How-to, examples, tutorial, and trace rows map to CLI/profile/adapter workflows. |
| `docs/CORPUS.md` names ownership and update obligations. | if multiple surfaces exist | accepted | The existing corpus surface is part of the planned docs package. |
| Mission 2.0 registry, state graph, evidence packet, agent protocol, and compatibility docs distinguish retained evidence from runtime behavior. | yes | accepted | Registry, state graph, evidence packet, agent protocol, and compatibility how-to/runbook docs plus release readiness repeat blocked runtime, mutation, migration, Cargo traversal, source scraping, plugin discovery, live state, and policy boundaries. |

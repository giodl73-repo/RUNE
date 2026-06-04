# RUNE Mission 2.0: Managed Native Semantic Runtime

## Mission upgrade

RUNE v1 proved that Rust crates can retain neutral, AI-readable contract
evidence without source scraping. Mission 2.0 extends that idea: make Rust feel
managed to tools, agents, reviewers, and operators while preserving native Rust
execution and ownership.

RUNE should become the semantic management layer for native Rust systems:
compile-time authored contracts, deterministic registries, retained evidence,
optional runtime inspection surfaces, and agent-safe protocols that explain what
a program is, what state it exposes, what evidence proves it, and what changes
are safe.

## Managed native thesis

Managed runtimes traditionally provide reflection, type registries, schema
negotiation, diagnostics, observability, and tooling protocols by owning the
runtime. RUNE should provide the same semantic benefits for Rust without
requiring garbage collection, bytecode execution, or a non-native VM.

```text
Idiomatic Rust
  + derive-authored contracts
  + explicit registries
  + retained evidence
  + optional runtime query surfaces
  -> native binaries with managed semantic affordances
```

## Stakeholder needs

| ID | Need |
|---|---|
| NEED-006 | Rust maintainers need a native semantic registry that can describe contract surfaces across crates without relying on source scraping. |
| NEED-007 | AI agents need safe, explicit query protocols for contracts, state, diagnostics, evidence, and allowed operations. |
| NEED-008 | Operators and reviewers need managed-runtime-style diagnostics and observability tied to contract identities instead of ad hoc logs. |
| NEED-009 | Platform teams need version negotiation and compatibility evidence across crate, profile, adapter, and runtime boundaries. |
| NEED-010 | Security and privacy reviewers need first-class capability and sensitivity metadata before runtime or agent surfaces expose contract data. |

## Mission 2.0 capability lanes

| Lane | Purpose | Boundary |
|---|---|---|
| Semantic registry | Discover descriptor collections, versions, profiles, adapters, and ownership boundaries at crate or process scope. | Registry is explicit and deterministic; no arbitrary source scraping. |
| State graph | Describe retained state nodes, transitions, ownership, retained evidence refs, and optional future live-state references. | State graph is opt-in and contract-backed; it does not pierce Rust privacy or borrow rules. |
| Evidence runtime | Emit validation, diagnostic, trace, health, and audit packets tied to descriptor ids. | Evidence packets are structured artifacts, not free-form logging replacements. |
| Agent protocol | Give tools/AI a read/query/generate/migrate surface with declared capabilities and diagnostics. | Mutating operations are separately authorized and fail closed by default. |
| Compatibility negotiation | Compare descriptor/profile/adapter/runtime versions and report safe, degraded, or blocked paths. | Compatibility is explicit evidence, not implicit best-effort conversion. |
| Capability and sensitivity policy | Mark data, operations, and evidence with exportability, sensitivity, stability, and authority metadata. | Policy metadata is descriptive first; enforcement requires a later runtime host DCR. |
| Runtime host | Optional embeddable native host that exposes selected registry, state, evidence, and protocol endpoints. | RUNE core stays neutral; host integrations remain optional crates or adapters. |

## DCR and implementation status

| DCR | Lane | Status | Implemented / blocked boundary |
|---|---|---|---|
| DCR-RUNE-002 | Mission 2.0 direction | approved | Managed native semantic runtime direction is approved; runtime host and agent mutation remain blocked. |
| DCR-RUNE-003 | Semantic registry | implemented read-only registry slices | Retained registry documents, `check-registry`, `inspect-registry`, catalog checks, and retained collection source-ref validation are implemented without Cargo traversal, source scraping, mutation, plugin discovery, or runtime host behavior. |
| DCR-RUNE-004 | State graph | implemented and role-review hardened | Retained `check-state-graph` validates semantic registry refs, descriptor-backed nodes/transitions, retained evidence refs, ownership refs, duplicate graph ids, and live-state blocking. No live state inspection, replay, mutation, pointer/heap walking, Cargo traversal, source scraping, or runtime host behavior is approved. |
| DCR-RUNE-005 | Evidence runtime packets | implemented retained evidence slice | Retained `check-evidence-packet` validates diagnostic, validation, trace, health, and audit packets against semantic registry and descriptor collection refs. No logging backend, runtime host, live inspection, mutation, private payload capture, Cargo traversal, source scraping, plugin discovery, migration, or policy enforcement is approved. |
| DCR-RUNE-006 | Agent protocol | implemented retained read-first slice | Retained `check-agent-protocol` validates registry, descriptor, compatibility, capability, profile/adapter, evidence, and blocked restricted-data request refs. No live endpoint, runtime host, mutation, prompt-only authority, source scraping, plugin discovery, migration, or policy enforcement is approved. |

## Non-goals

- Do not turn RUNE into a Rust VM, garbage collector, async runtime, plugin
  loader, or operating system.
- Do not add source scraping or runtime reflection that bypasses explicit Rust
  authoring.
- Do not move BAKER, LATTICE, AgentMaps, game, product, or portfolio vocabulary
  into `rune-core`.
- Do not expose private state or mutating actions to agents without explicit
  capability metadata and a reviewed protocol.
- Do not make Mission 2.0 block v1 adopters from using current descriptor,
  profile, adapter, and retained-evidence workflows.

## Success criteria

Mission 2.0 is credible when RUNE can:

1. Retain and validate multi-crate semantic registries with stable ownership and
   version boundaries.
2. Retain and validate state graphs that reference descriptor ids, semantic
   registry refs, ownership refs, and retained evidence.
3. Produce evidence/diagnostic packets that reference descriptor ids and field
   metadata.
4. Define an agent-safe protocol where tools can query contracts and evidence
   without source scraping.
5. Report compatibility between descriptor collections, profiles, adapters, and
   runtime hosts.
6. Preserve native Rust execution and product-neutral core vocabulary.

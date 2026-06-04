# RUNE Design Change Requests

Design change requests record planned changes to RUNE's controlled contract
surface before implementation. A DCR may approve a direction without claiming the
surface is implemented or stable.

## DCR-RUNE-001: Field-level data metadata

| Field | Value |
|---|---|
| Status | implemented read-only registry slices |
| Date | 2026-06-02 |
| Source requirement | RUNE-REQ-075 |
| Trigger | Shape data-contract scenario and games repo survey showed that data contracts need field-level metadata before broad game/simulation adoption. |
| Primary repo | RUNE |
| Coordinating repo | TRACKER |
| Scenario repos | RALLY, COURT, MUDDLE, RACKET, AMAZE, BANISH, QUEST, TIGRIS |

### Decision

RUNE should add a reviewed field-level metadata lane before trait/function
contracts. The next implementation wave should make field metadata explicit,
retained, and profile-visible while preserving the neutral core boundary.

First implementation slice completed under `RUNE-REQ-076`: `FieldDescriptor`
now carries explicit metadata, `#[rune_field(...)]` authors it, and
`rune.data_contract_json` preserves it in retained shape evidence.

### Proposed authoring surface

```rust
#[derive(RuneContract)]
#[rune(id = "example.shape.circle", version = "v0", kind = "entity")]
pub struct Circle {
    #[rune_field(
        required = true,
        unit = "px",
        min = "0",
        sensitivity = "public",
        example = "12.5",
        stability = "stable",
        alias = "radius"
    )]
    pub radius: f64,
}
```

Initial metadata keys:

| Key | Purpose | Initial representation |
|---|---|---|
| `required` | Whether a consumer may omit the field. | boolean |
| `unit` | Measurement unit or domain unit. | string |
| `min` / `max` | Declared boundary metadata. | string values to avoid premature numeric semantics |
| `sensitivity` | Review/privacy handling hint. | string |
| `example` | Copyable example value for docs and AI consumers. | string |
| `stability` | Compatibility expectation for the field. | string |
| `alias` | Alternate consumer-facing field name. | repeatable string |

### Scope

1. Extend RUNE descriptor documents with field metadata while preserving existing
   field `name` and `rust_type` behavior.
2. Add `#[rune_field(...)]` derive parsing with fail-closed diagnostics for
   unsupported keys.
3. Extend `rune.data_contract_json` to retain field metadata.
4. Upgrade `examples\rune-shape-calculator` to express units, bounds, examples,
   and stability for shape fields.
5. Add a games adoption spike document that maps COURT, MUDDLE, RALLY, and
   RACKET candidates to concrete RUNE annotations.
6. Defer trait/function contracts until the data-contract lane is verified.

### Non-goals

- Do not add game-specific vocabulary to `rune-core`.
- Do not make field metadata a runtime validator in this wave.
- Do not infer metadata from Rust doc comments or type names.
- Do not annotate games repos until the RUNE field metadata surface is reviewed,
  implemented, and pushed.
- Do not require TRACKER-relative Rust dependencies.

### Cross-repo ownership

| Repo | Owner action | Notes |
|---|---|---|
| RUNE | update | Implement field metadata, retained fixtures, docs, VTRACE, validation, and wave records. |
| TRACKER | update | Coordinate the portfolio wave and later record the RUNE submodule pointer after RUNE is pushed. |
| RALLY | defer | Preferred first adoption spike after RUNE field metadata lands because simulator/report structs are clean candidates. |
| COURT | defer | Candidate for snapshot/action/validation packet contracts after data metadata proves out. |
| MUDDLE | defer | Candidate for room/command/session/client snapshot contracts after RUNE metadata proves out. |
| RACKET | defer | Candidate for adapter runtime report/frame plan contracts after data metadata proves out. |
| AMAZE / BANISH / QUEST / TIGRIS | no-op for this DCR | Scenario-specific host/state contracts remain downstream candidates. |

### Proposed wave sequence

1. **Wave 36: Field metadata interface** - specify descriptor shape,
   `#[rune_field(...)]`, diagnostics, and compatibility rules.
2. **Wave 37: Field metadata implementation** - implement core, derive, CLI
   fixtures, and data-contract profile preservation.
3. **Wave 38: Shape metadata upgrade** - update the shape calculator evidence to
   include units, bounds, examples, stability, and sensitivity metadata.
4. **Wave 39: Games annotation spike plan** - produce a repo-by-repo annotation
   plan for RALLY, COURT, MUDDLE, and RACKET without editing games repos yet.
5. **Wave 40: First games adoption spike** - annotate one low-coupling game repo
   candidate, likely RALLY, only after RUNE field metadata is validated.

### Validation expectations

RUNE implementation waves must run:

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
git diff --check
```

TRACKER pointer waves must run:

```powershell
git diff --check
git diff --submodule=short -- repos\standards-protocols\rune
```

Games adoption spikes must use each game repo's documented validation commands
and must keep game-specific semantics in field metadata extensions, profile
outputs, or adapters rather than in RUNE core vocabulary.

### Open questions for Wave 36

| Question | Default for first implementation |
|---|---|
| Should `min` and `max` be typed numbers? | No; store as strings first to avoid cross-type coercion semantics. |
| Should aliases be repeatable? | Yes; preserve author order. |
| Should field metadata affect compatibility checks? | Not until profile support is explicit; unsupported concepts must fail closed. |
| Should docs comments be captured automatically? | No; require explicit metadata until doc capture has its own DCR. |
| Should trait/function contracts be designed now? | No; keep as the next lane after field metadata. |

## DCR-RUNE-002: Mission 2.0 managed native semantic runtime

| Field | Value |
|---|---|
| Status | approved mission direction |
| Date | 2026-06-03 |
| Source requirement | RUNE-REQ-077 |
| Trigger | Portfolio games adoption showed that RUNE descriptors are useful beyond schema generation: they can become the semantic management layer that makes native Rust systems inspectable, comparable, and agent-safe. |
| Primary repo | RUNE |
| Coordinating repo | TRACKER |
| Scenario repos | RALLY, COURT, MUDDLE, RACKET, BAKER, LATTICE |

### Decision

RUNE should upgrade its mission from "derive AI-readable contracts" to "provide
a managed native semantic runtime layer for Rust." The core remains neutral and
compile-time/retained-evidence first; runtime and agent surfaces are optional,
reviewed lanes that build on explicit descriptors, registries, profiles,
adapters, and field metadata.

### Approved capability lanes

| Lane | First DCR outcome |
|---|---|
| Semantic registry | Define a crate/process registry document that can list descriptor collections, profile support, adapter support, ownership, and version boundaries. |
| State graph | Define retained and optional live state graph documents keyed by descriptor ids, state nodes, transitions, and ownership. |
| Evidence runtime | Define structured diagnostic, validation, trace, health, and audit packets that reference descriptor ids and field metadata. |
| Agent protocol | Define a read-first protocol for AI/tool queries over registries, contracts, evidence, and compatibility reports. |
| Compatibility negotiation | Extend compatibility evidence from profile checks to collection/profile/adapter/runtime-host negotiation. |
| Capability and sensitivity policy | Promote sensitivity, stability, exportability, authority, and mutability metadata into a reviewed policy lane. |
| Runtime host | Defer implementation until registry, state, evidence, agent protocol, compatibility, and policy documents are specified. |

### Scope

1. Add `MISSION_2_0.md` to define the managed native thesis and non-goals.
2. Add requirements and trace rows for Mission 2.0 lanes.
3. Keep all runtime/agent implementation behind future DCRs.
4. Preserve current RUNE v1 adopter workflows.
5. Use RALLY, COURT, MUDDLE, and RACKET adoption evidence as scenario input, not
   core vocabulary.

### Non-goals

- Do not implement a runtime host in this DCR.
- Do not add a VM, garbage collector, plugin loader, source scraper, or product
  orchestrator.
- Do not expose mutating agent operations before capability metadata and protocol
  boundaries are reviewed.
- Do not make current descriptors require a runtime dependency.
- Do not encode BAKER, LATTICE, AgentMaps, games, or portfolio-specific terms in
  `rune-core`.

### Proposed wave sequence

1. **Wave 41: Mission 2.0 control package** - record mission, DCR, requirements,
   trace, verification, validation, and product docs.
2. **Wave 42: Semantic registry interface** - specify registry documents and
   deterministic process/crate ownership boundaries.
3. **Wave 43: State graph interface** - specify retained state graph and
   transition documents without live runtime requirements.
4. **Wave 44: Evidence runtime packets** - specify diagnostics, validation,
   health, audit, and trace packet documents.
5. **Wave 45: Agent protocol interface** - specify read-first query operations,
   diagnostics, and capability declarations.
6. **Wave 46: Compatibility negotiation** - specify collection/profile/adapter
   and runtime-host compatibility reports.
7. **Wave 47: Capability and sensitivity policy** - specify exportability,
   mutability, authority, stability, and sensitivity metadata.
8. **Wave 48: Optional runtime host design** - decide if and how an embeddable
   native runtime host should expose approved surfaces.

### Implementation readiness

Mission 2.0 implementation remains lane-gated. DCR-RUNE-003 through
DCR-RUNE-006 completed the first four retained-evidence slices; broad runtime
implementation is still blocked:

| Lane | Readiness |
|---|---|
| Semantic registry | implemented by DCR-RUNE-003 as retained registry documents, read-only CLI checks, inspection reports, catalog checks, and retained collection source-ref validation |
| State graph | implemented and role-review hardened by DCR-RUNE-004 as retained graph validation over registry refs, descriptor-backed nodes/transitions, retained evidence refs, ownership refs, duplicate graph-id diagnostics, and live-state blocking |
| Evidence runtime packets | implemented by DCR-RUNE-005 as retained packet documents, read-only CLI checks, packet family fixtures, descriptor/evidence ref validation, and audit capability decision checks |
| Agent protocol | implemented by DCR-RUNE-006 as retained read-first protocol request documents, read-only CLI checks, capability/ref validation, mutating-operation blocking, and restricted-data blocking |
| Compatibility negotiation | interface planning complete in `docs/architecture/compatibility-negotiation.md`; next implementation DCR must stay retained-evidence-first and must not approve automatic migration |
| Capability and sensitivity policy | interface planning complete in `docs/architecture/capability-sensitivity-policy.md`; implementation gated until enforcement boundaries are approved |
| Runtime host | design planning complete in `docs/architecture/runtime-host-design.md`; implementation blocked until all prior lanes have approved implementations |

### Planning closeout

Mission 2.0 planning is complete as a docs/spec package. The planning index is
`docs/architecture/mission-2-planning-index.md`.

Next allowed implementation-oriented work is a narrow DCR for **Wave 46:
Compatibility negotiation implementation**. It must stay retained-evidence-first
and must not add runtime host behavior, live process inspection, mutation/replay,
Cargo traversal, source scraping, plugin discovery, automatic migration, or
policy enforcement.

### Validation expectations

Mission 2.0 control-package waves are documentation-only and must run:

```powershell
git diff --check
```

Implementation waves must add code-specific validation before they are treated as
complete.

## DCR-RUNE-003: Semantic registry implementation

| Field | Value |
|---|---|
| Status | implemented read-only registry slices |
| Date | 2026-06-03 |
| Source requirement | RUNE-REQ-085 |
| Trigger | DCR-RUNE-002 and Mission 2.0 planning named semantic registry as the first implementation target because it builds directly on descriptor collections and explicit registries. |
| Primary repo | RUNE |
| Coordinating repo | TRACKER |

### Decision

RUNE should implement the first semantic registry slice as a retained evidence
document in `rune-core`. It should validate authored registry metadata without
adding live runtime discovery, Cargo traversal, source scraping, runtime host
behavior, or agent protocol operations.

### Implemented surface

| Surface | Status |
|---|---|
| `SemanticRegistryDraft` | implemented |
| `SemanticRegistryDocument` | implemented |
| `SemanticRegistryCollectionRef` | implemented |
| `SemanticRegistryProfileRef` | implemented |
| `SemanticRegistryAdapterRef` | implemented |
| `SemanticRegistryCapabilities` | implemented |
| Retained crate/workspace/failure fixtures | implemented |
| `rune check-registry --fixture <path>` | implemented |
| `rune inspect-registry --fixture <path>` | implemented |
| Retained collection source-ref loading | implemented for local fixture-relative refs |

### Validation behavior

| Diagnostic | Status |
|---|---|
| `RUNE-REGISTRY-001` missing registry identity | implemented |
| `RUNE-REGISTRY-002` missing registry version | implemented |
| `RUNE-REGISTRY-003` duplicate collection id/version ref | implemented |
| `RUNE-REGISTRY-004` unsupported registry scope | implemented |
| `RUNE-REGISTRY-005` malformed or mismatched collection source ref | implemented for retained local collection fixtures |
| `RUNE-REGISTRY-006` unknown or unsupported profile/adapter reference | implemented for approved catalog cross-checks |
| `RUNE-REGISTRY-007` runtime capability blocked without host boundary | implemented |

Registry source-ref validation is intentionally bounded to retained local
descriptor collection fixtures. It does not traverse Cargo metadata or discover
collections.

### Non-goals

- No runtime host.
- No live state inspection.
- No Cargo graph scanning.
- No source scraping.
- No mutating agent protocol.

### Next allowed work

The next semantic registry slice may add compatibility report documents or
consumer adoption fixtures, but must stay read-only unless a new DCR approves
mutation or runtime exposure.

## DCR-RUNE-004: Retained state graph implementation

| Field | Value |
|---|---|
| Status | implemented retained evidence slice |
| Date | 2026-06-03 |
| Source requirement | RUNE-REQ-090 |
| Trigger | DCR-RUNE-002 and `docs\architecture\state-graph-interface.md` named state graph evidence as the next Mission 2.0 lane after semantic registry identity and collection source refs existed. |
| Primary repo | RUNE |
| Coordinating repo | TRACKER |

### Decision

RUNE should implement the first state graph slice as retained evidence over an
explicit semantic registry and its retained descriptor collection refs. The
implementation should validate authored graph documents without adding live
state inspection, pointer/heap walking, mutation, replay, runtime host behavior,
Cargo traversal, or source scraping.

### Implemented surface

| Surface | Status |
|---|---|
| `StateGraphDraft` | implemented |
| `StateGraphDocument` | implemented |
| `StateGraphRegistryRef` | implemented |
| `StateGraphNode` | implemented |
| `StateGraphTransition` | implemented |
| `StateGraphOwnership` | implemented |
| `StateGraphEvidenceRef` | implemented |
| `StateGraphCapabilities` | implemented with live state blocked |
| Retained pass/failure fixtures | implemented |
| `rune check-state-graph --fixture <path> --registry <path>` | implemented read-only |

### Validation behavior

| Diagnostic | Status |
|---|---|
| `RUNE-STATE-001` missing state graph identity/version | implemented |
| `RUNE-STATE-002` missing or mismatched registry reference | implemented |
| `RUNE-STATE-003` node references unknown descriptor id | implemented |
| `RUNE-STATE-004` transition references unknown source or target node | implemented |
| `RUNE-STATE-005` transition references unsupported command/event descriptor | implemented |
| `RUNE-STATE-006` live state requested without runtime host approval | implemented |
| `RUNE-STATE-007` missing/unsupported/mismatched retained evidence ref | implemented |
| `RUNE-STATE-008` ownership references unknown node or transition id | implemented |
| `RUNE-STATE-009` duplicate node or transition id | implemented |

State graph validation is intentionally bounded to retained graph fixtures,
semantic registry fixtures, and the registry's retained descriptor collection
source refs.

### Non-goals

- No runtime host.
- No live state inspection.
- No pointer, borrow, or heap graph walking.
- No mutation or replay.
- No Cargo graph scanning.
- No Rust source scraping.
- No product-specific state vocabulary in `rune-core`.

## DCR-RUNE-005: Retained evidence runtime packet implementation

| Field | Value |
|---|---|
| Status | implemented retained evidence slice |
| Date | 2026-06-03 |
| Source requirement | RUNE-REQ-091 |
| Trigger | DCR-RUNE-002 and `docs\architecture\evidence-runtime-packets.md` named evidence runtime packets as the next Mission 2.0 lane after retained registry and state graph evidence existed. |
| Primary repo | RUNE |
| Coordinating repo | TRACKER |

### Decision

RUNE should implement evidence runtime packets as retained diagnostic,
validation, trace, health, and audit artifacts over explicit semantic registry
and descriptor collection refs. The implementation validates authored packet
documents without adding a logging framework, observability backend, runtime host,
private payload capture, mutation authorization, Cargo traversal, source
scraping, plugin discovery, automatic migration, or policy enforcement.

### Implemented surface

| Surface | Status |
|---|---|
| `EvidenceRuntimePacketDraft` | implemented |
| `EvidenceRuntimePacketDocument` | implemented |
| `EvidencePacketRegistryRef` | implemented |
| `EvidenceDescriptorRef` | implemented |
| `EvidencePacketDiagnostic` | implemented |
| `EvidenceCapabilityDecision` | implemented for audit packets |
| Retained diagnostic/validation/trace/health/audit fixtures | implemented |
| `rune check-evidence-packet --fixture <path> --registry <path>` | implemented read-only |

### Validation behavior

| Diagnostic | Status |
|---|---|
| `RUNE-EVIDENCE-001` missing packet identity/version | implemented |
| `RUNE-EVIDENCE-002` unsupported packet kind | implemented |
| `RUNE-EVIDENCE-003` unknown or mismatched descriptor reference | implemented |
| `RUNE-EVIDENCE-004` unsupported severity or status | implemented |
| `RUNE-EVIDENCE-005` audit packet missing capability decision | implemented |
| `RUNE-EVIDENCE-006` mismatched registry reference | implemented |
| `RUNE-EVIDENCE-007` unsupported or undeclared retained evidence ref | implemented |

Evidence packet validation is intentionally bounded to retained packet fixtures,
semantic registry fixtures, and the registry's retained descriptor collection
source refs.

### Non-goals

- No replacement for `tracing`, logs, metrics, or observability backends.
- No runtime host.
- No live state inspection.
- No private payload capture.
- No mutation authorization or replay.
- No Cargo graph scanning.
- No Rust source scraping.
- No plugin discovery, automatic migration, or policy enforcement.

## DCR-RUNE-006: Retained read-first agent protocol implementation

| Field | Value |
|---|---|
| Status | implemented retained evidence slice |
| Date | 2026-06-03 |
| Source requirement | RUNE-REQ-092 |
| Trigger | DCR-RUNE-002 and `docs\architecture\agent-protocol-interface.md` named a read-first agent protocol as the next Mission 2.0 lane after retained registry, state graph, and evidence packet surfaces existed. |
| Primary repo | RUNE |
| Coordinating repo | TRACKER |

### Decision

RUNE should implement the first agent protocol slice as retained request evidence
over explicit semantic registry and descriptor collection refs. The slice is
read-first and validates authored protocol request documents without adding live
endpoints, autonomous mutation, prompt-only authority, hidden source scraping,
private data exposure, runtime host behavior, Cargo traversal, plugin discovery,
automatic migration, or policy enforcement.

### Implemented surface

| Surface | Status |
|---|---|
| `AgentProtocolRequestDraft` | implemented |
| `AgentProtocolRequestDocument` | implemented |
| `AgentProtocolInputRefs` | implemented |
| `AgentProtocolCollectionRef` | implemented |
| `AgentProtocolResult` | implemented |
| Retained `registry.describe`, `descriptor.get`, and `compatibility.check` fixtures | implemented |
| Retained unknown operation, mutating operation, missing capability, unknown ref, and restricted-data fixtures | implemented |
| `rune check-agent-protocol --fixture <path> --registry <path>` | implemented read-only |

### Validation behavior

| Diagnostic | Status |
|---|---|
| `RUNE-AGENT-001` unknown operation | implemented |
| `RUNE-AGENT-002` missing or mismatched required capability | implemented |
| `RUNE-AGENT-003` mutating operation blocked | implemented |
| `RUNE-AGENT-004` unknown registry, collection, descriptor, evidence, profile, or adapter ref | implemented |
| `RUNE-AGENT-005` restricted data exposure blocked | implemented |

Agent protocol validation is intentionally bounded to retained request fixtures,
semantic registry fixtures, the registry's retained descriptor collection source
refs, and registry-declared profile/adapter refs.

### Non-goals

- No live endpoint, server, SDK, or runtime host.
- No autonomous mutation or replay.
- No prompt-only authority model.
- No private data exposure.
- No Cargo graph scanning.
- No Rust source scraping.
- No plugin discovery, automatic migration, or policy enforcement.

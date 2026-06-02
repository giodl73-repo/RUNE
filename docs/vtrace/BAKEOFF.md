# RUNE Bakeoff Findings

## Engineering decision supported

Record the first RUNE validation slice using the approved read-only inspection
surface. This bakeoff validates inspectability of neutral descriptor metadata; it
does not validate generators, adapters, arbitrary crate analysis, or broad
portfolio adoption.

## Scenario

| Field | Value |
|---|---|
| Scenario | Fixture-backed contract inspection |
| Target | `crates/rune-cli/tests/fixtures/valid_descriptor.json` |
| Command | `cargo run -p rune-cli -- inspect --fixture crates\rune-cli\tests\fixtures\valid_descriptor.json` |
| Retained expected output | `crates/rune-cli/tests/fixtures/valid_descriptor.inspection.json` |
| Negative fixture | `crates/rune-cli/tests/fixtures/missing_id_descriptor.json` |

## Baseline comparison

| Question | Source/prose-only baseline | RUNE inspection path |
|---|---|---|
| What is the contract identity? | Reader must infer or trust prose. | `id` is explicit: `example.customer`. |
| What version is being reviewed? | Reader must infer from surrounding docs or code history. | `version` is explicit: `v0`. |
| What kind of contract is it? | Reader must infer from type name and context. | `kind` is explicit: `entity`. |
| What Rust item is represented? | Reader must inspect source. | `rust_type` is explicit: `Customer`. |
| What fields exist? | Reader must inspect source. | `fields` lists `id: String`. |
| What invariant matters? | Reader must inspect comments, docs, or tests. | `invariants` includes `customer.id.present`. |
| What requirement is linked? | Reader must search docs manually. | `trace_links` points to `RUNE-REQ-033`. |
| What happens on missing identity? | Source/prose may omit it silently. | Inspection fails closed with `RUNE-INSP-001`. |

## Success criteria results

| Criterion | Result | Evidence |
|---|---|---|
| Inspectability | pass | Inspection output exposes descriptor fields without macro expansion. |
| Neutrality | pass | Output uses RUNE descriptor vocabulary only; no downstream product terms. |
| Identity/versioning | pass | Output preserves `id` and `version`. |
| Unsupported concepts | partial pass | Missing identity fails closed; broader unsupported concept coverage remains future work. |
| AI usefulness | provisional pass | The inspected contract answers basic contract-shape questions without source scraping. Broader usefulness requires more scenarios. |
| Review usefulness | pass | Output links to `RUNE-REQ-033` and verification fixtures are retained. |

## Limitations

- This is a fixture-backed bakeoff, not arbitrary Rust crate analysis.
- This is inspection, not generation.
- This does not validate JSON Schema, CSDL, OpenAPI, AgentMap, or any downstream
  profile.
- This does not prove broad adoption readiness.
- The next validation slice should use a small annotated Rust type as the source
  and retain the inspected descriptor output.

## Annotated type and neutral artifact bakeoff

| Field | Value |
|---|---|
| Scenario | Annotated Rust type to neutral generated artifact |
| Annotated type | `crates/rune-derive/tests/derive_contract.rs` |
| Descriptor fixture | `crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json` |
| Generated artifact | `crates/rune-cli/tests/fixtures/annotated_customer.neutral_descriptor_artifact.json` |
| Command | `cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json` |

### Result

| Criterion | Result | Evidence |
|---|---|---|
| Annotated source | pass | `Customer` derives `RuneContract` with `id`, `version`, and `kind`. |
| Descriptor preservation | pass | Retained descriptor fixture preserves `id`, `version`, `kind`, `rust_type`, and fields. |
| Generated artifact | pass | Neutral profile output wraps the descriptor with profile metadata. |
| External format neutrality | pass | No JSON Schema, CSDL, OpenAPI, AgentMap, or product adapter is emitted. |
| Automation | partial | The descriptor fixture is retained manually; automatic crate-wide discovery remains future work. |

## Derive-to-document bridge follow-up

The annotated `Customer` type now emits a runtime `ContractDescriptor` with a
neutral requirement trace link. `rune-core::DescriptorDocument` converts that
runtime descriptor into the same JSON shape used by the retained fixture, and
`crates/rune-derive/tests/derive_contract.rs` compares the derived document to
`crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json`.

This upgrades the bakeoff from manually aligned evidence to derived evidence for
the single annotated type. It remains short of arbitrary crate discovery or CLI
build integration.

## Repo adoption bakeoff gate

| Field | Value |
|---|---|
| Wave | `context/waves/2026-06-01-repo-adoption-bakeoff/` |
| Scenario target | Existing annotated `Customer` contract |
| Baseline | Source/prose-only review of the annotated Rust type and tests |
| RUNE path | `rune check` plus `rune generate --profile rune.neutral_descriptor_json` |
| Evidence | Retained descriptor, check report, and neutral generated artifact fixtures |

### Gate decision

The scenario may proceed because Wave 3 now provides:

| Capability | Status |
|---|---|
| Neutral descriptor document | available |
| Derived descriptor evidence | available for one annotated type |
| Read-only compatibility check | available |
| Neutral generated artifact | available |
| Profile compatibility diagnostics | available for version, kind, invariants, trace links, and extensions |

### Limits

- This is still fixture-backed.
- This does not discover descriptors across arbitrary Rust crates.
- This does not add external profiles or product adapters.
- This does not prove broad adoption readiness.

## Wave 4 source-only baseline

| Field | Value |
|---|---|
| Scenario | Source/prose-only review of annotated `Customer` contract |
| Source | `crates/rune-derive/tests/derive_contract.rs` |
| RUNE evidence excluded | Descriptor fixture, check report, generated artifact, profile catalog |

### Baseline findings

| Review question | Source-only result | Burden |
|---|---|---|
| What Rust type is represented? | `Customer` is visible. | low |
| What fields exist? | `id: String` and `email: String` are visible. | low |
| What stable contract id should tools use? | `example.customer` appears in `#[rune(...)]`, but a reviewer must know macro semantics. | medium |
| What version is durable? | `v0` appears in `#[rune(...)]`, but durability depends on RUNE behavior. | medium |
| What kind is intended? | `entity` appears in `#[rune(...)]`, but vocabulary meaning is external to plain Rust. | medium |
| What requirement is linked? | `RUNE-REQ-034` appears, but trace-link relation semantics are implicit. | medium |
| Is the descriptor profile-compatible? | Not answerable from source-only review. | high |
| What generated artifact would tools consume? | Not answerable from source-only review. | high |
| Are unsupported concepts fail-closed? | Not answerable from this source file alone. | high |

### Baseline conclusion

Source-only review is adequate for seeing the struct and fields, but it leaves
machine-facing identity, version semantics, trace semantics, profile
compatibility, artifact shape, and fail-closed behavior dependent on macro/test
knowledge or manual inference. The next comparison must evaluate whether RUNE
check and generated artifact evidence reduce that inference burden.

## Wave 4 RUNE evidence comparison

| Field | Value |
|---|---|
| Scenario | RUNE evidence review of annotated `Customer` contract |
| Descriptor evidence | `crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json` |
| Compatibility evidence | `crates/rune-cli/tests/fixtures/annotated_customer.check.json` |
| Generated artifact evidence | `crates/rune-cli/tests/fixtures/annotated_customer.neutral_descriptor_artifact.json` |

### Comparison findings

| Review question | Source-only burden | RUNE evidence result |
|---|---|---|
| Stable contract id | medium | Descriptor and artifact state `example.customer`. |
| Durable version | medium | Descriptor and check report state `v0`. |
| Contract kind | medium | Descriptor, check report, and artifact state `entity`. |
| Requirement trace | medium | Descriptor and artifact preserve `requirement -> RUNE-REQ-034`. |
| Profile compatibility | high | Check report states `status: ok` for `rune.neutral_descriptor_json@v0`. |
| Generated artifact shape | high | Artifact declares `output_artifact_kind: rune.descriptor.json`. |
| Profile metadata | high | Artifact declares `profile_id` and `profile_version`. |
| Neutrality | manual review | Evidence uses neutral RUNE vocabulary and no product adapter names. |

### Evidence conclusion

RUNE evidence reduces review ambiguity for the controlled annotated type. It
turns macro semantics and profile compatibility from inferred source knowledge
into explicit JSON evidence. This supports scenario-level usefulness but still
does not prove broad repo adoption, crate discovery, or external profile
readiness.

## Wave 4 readiness decision

| Area | Result | Finding |
|---|---|---|
| Controlled scenario usefulness | pass | RUNE evidence reduces source-only inference for the annotated `Customer` contract. |
| Neutrality | pass | Evidence uses neutral descriptor/profile vocabulary only. |
| Compatibility evidence | pass | `rune check` gives a compact compatibility report without artifact generation. |
| Generated artifact evidence | pass | Neutral artifact records profile metadata and descriptor content. |
| Broad adoption | blocked | Evidence is fixture-backed and scoped to one annotated type. |
| Next step | derive evidence automation | Remove manual fixture dependency before external profiles or adapters. |

## Wave 5 readiness decision

| Area | Result | Finding |
|---|---|---|
| Deterministic regeneration | pass | The annotated customer descriptor is regenerated from `DescriptorDocument::from_contract::<Customer>()`. |
| Retained evidence stability | pass | The regeneration check leaves the retained descriptor unchanged. |
| Manual fixture dependency | reduced | The controlled descriptor fixture is generated evidence, not a standalone parallel model. |
| Multi-contract collection | blocked | Only one explicitly known annotated contract is covered. |
| Broad adoption | blocked | No arbitrary crate discovery or repo-scale evidence collection exists. |
| Next step | known contract evidence collection | Add multiple explicitly registered contracts before discovery or external profiles. |

## Wave 6 multi-contract evidence check

| Area | Result | Finding |
|---|---|---|
| Explicit known-contract collection | pass | The derive integration test serializes `Customer` and `CreateCustomer` into one retained descriptor collection fixture. |
| Deterministic order | pass | The test-owned registry emits descriptors in a fixed order. |
| Neutrality | pass | The collection contains only neutral RUNE descriptor documents. |
| Broad adoption | blocked | Collection is test-owned and explicit; there is still no arbitrary crate discovery or external profile output. |

## Wave 6 readiness decision

| Area | Result | Finding |
|---|---|---|
| Multi-contract known evidence | pass | RUNE retains deterministic descriptor evidence for entity and command contracts plus a combined collection fixture. |
| Test-owned collection boundary | pass | Collection is explicit and ordered by the derive integration test. |
| Registry/discovery interface | blocked | No stable interface exists yet for repo-scale contract registration or discovery. |
| Broad adoption | blocked | External profiles, product adapters, and arbitrary crate scanning remain out of scope. |

## Wave 7 explicit registry readiness

| Area | Result | Finding |
|---|---|---|
| Core registry interface | pass | `ContractRegistration` and `collect_known_contract_documents` define the explicit known-contract boundary. |
| Deterministic collection | pass | Collection preserves caller-provided order. |
| Duplicate identity diagnostics | pass | Duplicate descriptor ids fail closed with `RUNE-REG-001`. |
| Discovery readiness | blocked | Repo-scale discovery and CLI collection require a separate interface-control wave. |

## Wave 8 contract-kind coverage

| Area | Result | Finding |
|---|---|---|
| Entity scenario | pass | `Customer` retains descriptor, check, generated artifact, and collection evidence. |
| Command scenario | pass | `CreateCustomer` retains descriptor, check, generated artifact, and collection evidence. |
| Event scenario | pass | `CustomerCreated` retains descriptor, check, generated artifact, and collection evidence. |
| Broad adoption | blocked | Scenario coverage does not replace discovery/interface validation. |

## Wave 9 state contract coverage

| Area | Result | Finding |
|---|---|---|
| State scenario | pass | `CustomerLifecycleState` retains descriptor, check, generated artifact, and collection evidence. |
| Registry coverage | pass | The state scenario is included in the explicit known-contract registry. |
| Neutrality | pass | Evidence uses neutral RUNE descriptor vocabulary only. |
| Broad adoption | blocked | Scenario coverage does not replace discovery/interface validation. |

## Wave 12 evidence contract coverage

| Area | Result | Finding |
|---|---|---|
| Evidence scenario | pass | `ContractVerificationEvidence` retains descriptor, check, generated artifact, and collection evidence. |
| Registry coverage | pass | The evidence scenario is included in the explicit known-contract registry. |
| Neutral profile kind coverage | pass | Retained scenarios now cover entity, command, event, state, artifact, source, and evidence. |
| Broad adoption | blocked | Scenario coverage does not replace discovery/interface validation. |

## Wave 13 descriptor collection document

| Area | Result | Finding |
|---|---|---|
| Collection identity | pass | Retained collection evidence now includes `collection_id`. |
| Collection versioning | pass | Retained collection evidence now includes `collection_version`. |
| Ordered descriptors | pass | Descriptor order remains controlled by the explicit registry. |
| Broad adoption | blocked | Durable collection evidence does not replace discovery/interface validation. |

## Wave 14 collection document validation

| Area | Result | Finding |
|---|---|---|
| Missing collection identity | pass | Collection drafts fail closed with `RUNE-COLL-001`. |
| Missing collection version | pass | Collection drafts fail closed with `RUNE-COLL-002`. |
| Duplicate descriptor ids | pass | Collection drafts fail closed with `RUNE-COLL-003`. |
| Broad adoption | blocked | Validation does not replace discovery/interface validation. |

## Wave 15 collection inspection CLI

| Area | Result | Finding |
|---|---|---|
| Retained collection inspection | pass | `inspect-collection` emits retained neutral collection JSON. |
| Missing collection identity | pass | CLI fails closed with `RUNE-COLL-INSP-001`. |
| Missing collection version | pass | CLI fails closed with `RUNE-COLL-INSP-002`. |
| Duplicate descriptor ids | pass | CLI fails closed with `RUNE-COLL-INSP-003`. |
| Broad adoption | blocked | Fixture-backed inspection does not replace discovery/interface validation. |

## Wave 16 collection compatibility check CLI

| Area | Result | Finding |
|---|---|---|
| Retained collection check | pass | `check-collection` emits retained collection compatibility JSON. |
| Missing collection identity | pass | CLI fails closed with `RUNE-COLL-CHECK-001`. |
| Duplicate descriptor ids | pass | CLI fails closed with `RUNE-COLL-CHECK-008`. |
| Profile-unsupported descriptor kind | pass | CLI fails closed with `RUNE-COLL-CHECK-003`. |
| Broad adoption | blocked | Fixture-backed checks do not replace discovery/interface validation. |

## Wave 17 collection neutral generation CLI

| Area | Result | Finding |
|---|---|---|
| Retained collection artifact | pass | `generate-collection` emits retained neutral collection artifact JSON. |
| Missing collection identity | pass | CLI fails closed with `RUNE-COLL-GEN-001`. |
| Duplicate descriptor ids | pass | CLI fails closed with `RUNE-COLL-GEN-008`. |
| Unknown profile | pass | CLI fails closed with `RUNE-COLL-GEN-003`. |
| Profile-unsupported descriptor kind | pass | CLI fails closed with `RUNE-COLL-GEN-003`. |
| Broad adoption | blocked | Fixture-backed generation does not replace discovery/interface validation. |

## Wave 18 collection inventory CLI

| Area | Result | Finding |
|---|---|---|
| Retained inventory | pass | `inventory-collection` emits retained descriptor and kind counts. |
| Missing collection identity | pass | CLI fails closed with `RUNE-COLL-INV-001`. |
| Duplicate descriptor ids | pass | CLI fails closed with `RUNE-COLL-INV-003`. |
| Broad adoption | blocked | Fixture-backed inventory does not replace discovery/interface validation. |

## Wave 19 v1 scope and documentation reconciliation

| Area | Result | Finding |
|---|---|---|
| V1 boundary | pass | V1 explicitly includes deterministic discovery, external profiles, and adapters. |
| Core neutrality | pass | Product-specific vocabulary remains barred from `rune-core`. |
| Documentation alignment | pass | README now lists the implemented CLI evidence surface. |
| Implementation approval | blocked | Discovery, external profiles, and adapters still require later interface waves. |

## Wave 20 derive v1 ergonomics

| Area | Result | Finding |
|---|---|---|
| Missing identity | pass | Derive compile-fail tests reject missing `id`. |
| Missing version | pass | Derive compile-fail tests reject missing `version`. |
| Deferred ergonomics | pass | Field metadata, enum variants, invariant macros, docs, source inference, and adapter hints are documented as deferred. |
| Core neutrality | pass | No product-specific macro vocabulary was added. |

## Wave 21 crate-owned registry workflow

| Area | Result | Finding |
|---|---|---|
| Adopter-owned registry | pass | `examples/rune-adopter` owns `RUNE_CONTRACTS`. |
| Deterministic order | pass | Retained collection evidence preserves explicit slice order. |
| Duplicate diagnostics | pass | Duplicate descriptor ids fail closed with `RUNE-REG-001`. |
| Discovery boundary | blocked | Crate scanning and Cargo metadata discovery remain deferred. |

## Wave 22 deterministic discovery interface

| Area | Result | Finding |
|---|---|---|
| Discovery model | pass | Manifest-based discovery over retained descriptor collection fixtures is specified. |
| Ordering | pass | Manifest order and source collection order are deterministic. |
| Diagnostics | pass | Missing manifest data, unsupported source kinds, unreadable sources, and duplicates have reserved diagnostics. |
| Implementation | blocked | Discovery implementation waits for the next wave. |

## Wave 23 deterministic discovery implementation

RUNE now implements the approved manifest discovery boundary. The retained
manifest `crates\rune-cli\tests\fixtures\adopter_discovery_manifest.json`
references the adopter registry collection and produces
`crates\rune-cli\tests\fixtures\adopter_discovered_collection.json`.

| Area | Result | Finding |
|---|---|---|
| Discovery evidence | pass | The discovered collection records output collection identity/version and merged descriptors. |
| Ordering | pass | Descriptor order follows the manifest source order and retained source collection order. |
| Diagnostics | pass | Unsupported source kinds and duplicate discovered descriptor ids fail closed. |
| Neutrality | pass | Output remains a neutral descriptor collection document. |
| Broad adoption | blocked | This does not add source scanning, Cargo metadata traversal, external profiles, or adapters. |

## Wave 24 retained evidence workflow

RUNE now has retained evidence bundle fixtures for:

- `crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.evidence_bundle.json`
- `crates\rune-cli\tests\fixtures\adopter_discovered_collection.evidence_bundle.json`

| Area | Result | Finding |
|---|---|---|
| Collection evidence bundle | pass | The known-contract collection fixture emits collection, check, inventory, and generated artifact evidence. |
| Discovery evidence bundle | pass | The adopter discovery manifest emits the same bundle shape after deterministic discovery. |
| Portable evidence | pass | The bundle source label avoids machine-specific absolute paths. |
| Mutation control | pass | The command is read-only and requires explicit output redirection to refresh retained evidence. |

## Wave 25 external profile interface

| Area | Result | Finding |
|---|---|---|
| Profile boundary | pass | External profiles are mappings over validated neutral descriptors or collections. |
| Vocabulary isolation | pass | Profile-specific terms belong below the generated artifact boundary, not in `rune-core`. |
| First candidate | pass | Documentation packet profile is the preferred first implementation candidate. |
| Implementation | blocked | No external profile artifact is emitted until the next implementation wave. |

## Wave 26 first external profile

RUNE now retains generated documentation packet artifacts for:

- `crates\rune-cli\tests\fixtures\annotated_customer.documentation_packet.json`
- `crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.documentation_packet.json`

| Area | Result | Finding |
|---|---|---|
| Descriptor packet | pass | A single descriptor produces a compact documentation packet with field and trace-link summaries. |
| Collection packet | pass | A descriptor collection produces ordered documentation summaries for all retained contract kinds. |
| Profile boundary | pass | Output uses profile-owned vocabulary without rewriting the neutral descriptor model. |
| Adapter readiness | partial | Documentation packets are useful to reviewers and AI tools, but product adapters remain separate future work. |

## Wave 27 downstream adapter interface

| Area | Result | Finding |
|---|---|---|
| Adapter design | pass | Adapters are separate mappings from validated RUNE evidence or profile outputs. |
| Core neutrality | pass | Product-specific terms remain outside `rune-core` and derive attributes. |
| First candidates | pass | Review packet, context map, and transition input adapters are viable first implementation families. |
| Implementation | blocked | No adapter output is emitted until a later implementation wave. |

## Wave 28 first adapter implementation

RUNE now retains adapter evidence for:

- `crates\rune-cli\tests\fixtures\adapter_list.json`
- `crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.review_packet.json`

| Area | Result | Finding |
|---|---|---|
| Review packet adapter | pass | Validated descriptor collections produce deterministic review packet items. |
| Adapter catalog | pass | `rune adapter list` exposes approved adapter metadata. |
| Compatibility diagnostics | pass | Unknown adapters, malformed inputs, and unsupported kinds fail closed. |
| Core neutrality | pass | Adapter vocabulary remains outside `rune-core`. |

## Wave 29 CLI hardening

| Area | Result | Finding |
|---|---|---|
| Operator clarity | pass | `status` reports current v1 surfaces, profiles, and adapters. |
| Failure clarity | pass | Unknown commands, missing flags, invalid argument order, and bad JSON produce explicit stderr. |
| Regression coverage | pass | `crates\rune-cli\tests\cli_hardening.rs` covers hardening cases before release readiness. |
| Scope control | pass | Hardening does not add new discovery, profile, or adapter behavior. |

## Wave 30 adopter examples

RUNE now retains adopter example evidence for:

- `examples\rune-adopter\tests\fixtures\adopter_contract_collection.json`
- `examples\rune-adopter\tests\fixtures\adopter_documentation_packet.json`
- `examples\rune-adopter\tests\fixtures\adopter_review_packet.json`

| Area | Result | Finding |
|---|---|---|
| End-to-end adopter path | pass | The example crate covers derive, registry, retained collection, profile output, and adapter output. |
| AI/reviewer usefulness | pass | Documentation and review packet artifacts expose contract shape without source scraping. |
| Determinism | pass | Tests compare generated evidence to retained fixtures. |
| Broad repo adoption | partial | The example is representative but still a controlled workspace crate, not an external repo bakeoff. |

## Wave 31 representative repo bakeoff: QUIVER

| Field | Value |
|---|---|
| Scenario repo | `C:\src\quiver` |
| Scenario type | Non-RUNE Rust workspace, reviewed as a scenario only |
| Source files sampled | `quiver-core`, `quiver-manifest`, `quiver-runtime`, `quiver-cli` |
| RUNE descriptor evidence | `docs\vtrace\fixtures\quiver_bakeoff_descriptor_collection.json` |
| Compatibility evidence | `docs\vtrace\fixtures\quiver_bakeoff.check.json` |
| Documentation packet evidence | `docs\vtrace\fixtures\quiver_bakeoff.documentation_packet.json` |
| Adapter evidence | `docs\vtrace\fixtures\quiver_bakeoff.review_packet.json` |

### Source-only baseline

Source-only review can identify `Capability`, `RepoTarget`, `RunPlan`, and
`RunEvent` by reading Rust structs and serde derives. It does not directly answer
which identifiers should be stable for AI tooling, which contract kind each type
plays, whether a profile can represent them, or what review prompts should be
carried downstream.

### RUNE evidence comparison

| Review question | Source-only burden | RUNE evidence result |
|---|---|---|
| Stable contract identities | medium | Descriptor collection declares `quiver.capability`, `quiver.repo_target`, `quiver.run_plan`, and `quiver.run_event`. |
| Contract versions | medium | Every retained descriptor declares `v0`. |
| Contract kind | medium | RUNE records entity, artifact, and event roles explicitly. |
| Field visibility | low | Documentation packet summarizes fields and counts without source reading. |
| Profile compatibility | high | Check evidence reports `status: ok` for all selected descriptors. |
| Review/adaptation | high | Review packet adapter emits deterministic review prompts per descriptor. |

### Bakeoff decision

| Area | Result | Finding |
|---|---|---|
| AI usefulness | pass | RUNE evidence makes contract identity, role, and review prompts explicit. |
| Review usefulness | pass | Review packet output gives a compact checklist for selected QUIVER contracts. |
| Neutrality | pass | Evidence uses RUNE neutral descriptor vocabulary and a generic review-packet adapter. |
| Integration readiness | partial | The scenario did not modify QUIVER; native derives/registry would be future adoption work. |

## Wave 32 v1 release readiness

| Area | Result | Finding |
|---|---|---|
| Release gate | pass | CI-ready commands are documented in README and release readiness docs. |
| Compatibility policy | pass | Versioning and diagnostic compatibility expectations are documented. |
| Evidence coverage | pass | VTRACE records cover discovery, profiles, adapters, adopter examples, and representative repo bakeoff. |
| V1 scope | pass | Non-goals keep product vocabulary and unreviewed discovery out of the neutral core. |

## Wave 11 source contract coverage

| Area | Result | Finding |
|---|---|---|
| Source scenario | pass | `ContractSourceReference` retains descriptor, check, generated artifact, and collection evidence. |
| Registry coverage | pass | The source scenario is included in the explicit known-contract registry. |
| Neutrality | pass | Evidence uses neutral RUNE descriptor vocabulary only. |
| Broad adoption | blocked | Scenario coverage does not replace discovery/interface validation. |

## Wave 10 artifact contract coverage

| Area | Result | Finding |
|---|---|---|
| Artifact scenario | pass | `ContractEvidenceArtifact` retains descriptor, check, generated artifact, and collection evidence. |
| Registry coverage | pass | The artifact scenario is included in the explicit known-contract registry. |
| Neutrality | pass | Evidence uses neutral RUNE descriptor vocabulary only. |
| Broad adoption | blocked | Scenario coverage does not replace discovery/interface validation. |

## Validation command

```powershell
cargo run -p rune-cli -- inspect --fixture crates\rune-cli\tests\fixtures\valid_descriptor.json
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo test --workspace
```

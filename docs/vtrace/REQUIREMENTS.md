# RUNE Requirements

## Foundation and stage control

| ID | Requirement | Verification |
|---|---|---|
| RUNE-REQ-001 | RUNE shall maintain a VTRACE package that links mission, stakeholder needs, requirements, trace, verification, and review evidence. | Review `docs/vtrace/` and active wave status. |
| RUNE-REQ-002 | RUNE shall advance through explicit VTRACE stage gates before expanding contract-stable descriptor, macro, CLI, generator, or adoption surfaces. | Review `docs/vtrace/STAGES.md` and active wave status. |
| RUNE-REQ-003 | RUNE base specs shall avoid downstream product vocabulary in core descriptor names and stable requirements. | Review `README.md`, `PRODUCT_PLAN.md`, `docs/architecture/descriptor-model.md`, and `rune-core` public API. |

## Neutral descriptor requirements

| ID | Requirement | Verification |
|---|---|---|
| RUNE-REQ-010 | RUNE shall define neutral contract descriptor traits and records for Rust code. | `cargo test --workspace` and descriptor model review. |
| RUNE-REQ-011 | RUNE descriptors shall include stable identity and version fields before generated artifacts are treated as durable. | Descriptor interface spec review and generated artifact inspection. |
| RUNE-REQ-012 | RUNE descriptors shall support the foundation concept set: entity, event, command, state, artifact, invariant, source, evidence, contract, and version. | Descriptor vocabulary review. |
| RUNE-REQ-013 | RUNE descriptors shall preserve traceability links from generated contracts back to requirements, design elements, implementation surfaces, verification methods, and evidence when supplied. | Trace fixture review and generated artifact inspection. |

## Macro requirements

| ID | Requirement | Verification |
|---|---|---|
| RUNE-REQ-020 | RUNE shall provide procedural derive support for generating descriptors from Rust types after the macro interface is specified. | Macro interface spec review and `cargo test --workspace`. |
| RUNE-REQ-021 | RUNE macros shall make generated descriptor behavior inspectable through tests, CLI output, or generated artifacts. | Macro compile tests and inspection command output. |
| RUNE-REQ-022 | RUNE macros shall not encode downstream platform vocabulary in core derives or required attributes. | Macro API review. |

## CLI and generator requirements

| ID | Requirement | Verification |
|---|---|---|
| RUNE-REQ-030 | RUNE shall expose a CLI entry point for future inspection and generator workflows. | `cargo run -p rune-cli -- status` |
| RUNE-REQ-031 | RUNE shall treat generated schemas, IDL-style definitions, trace/event contracts, state maps, documentation, review packets, and AI-facing metadata as downstream profiles over neutral descriptors. | Review `PRODUCT_PLAN.md`, `docs/architecture/descriptor-model.md`, and future generator designs. |
| RUNE-REQ-032 | RUNE generator profiles shall report unsupported descriptor concepts through diagnostics rather than silent omission. | Generator interface spec review and future generated artifact checks. |
| RUNE-REQ-033 | RUNE shall define read-only descriptor inspection before approving profile generators or broad validation bakeoffs. | Review `docs/architecture/inspection-surface.md` and future CLI inspection tests. |
| RUNE-REQ-034 | RUNE shall define generator/profile compatibility, input, output, and diagnostics contracts before implementing generated artifact profiles. | Review `docs/architecture/generator-profile-interface.md`. |
| RUNE-REQ-035 | RUNE shall expose approved generator profiles through a read-only profile catalog before external profiles are added. | `cargo run -p rune-cli -- profile list` and CLI profile tests. |
| RUNE-REQ-036 | RUNE CLI inspection and generation shall use the shared core descriptor document boundary instead of maintaining independent descriptor JSON models. | Core descriptor draft tests and CLI fixture tests. |
| RUNE-REQ-037 | RUNE approved profile metadata shall be owned by a shared neutral profile catalog before profile facts are used by CLI generation. | Core profile catalog tests and CLI profile/generate tests. |
| RUNE-REQ-038 | RUNE generation shall fail closed when a descriptor version or descriptor kind is not supported by the selected profile. | Core profile compatibility tests and CLI generation fail-closed tests. |
| RUNE-REQ-039 | RUNE shall provide a read-only compatibility check that validates descriptor/profile compatibility without emitting generated artifacts. | `cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture <path>` and CLI check tests. |
| RUNE-REQ-043 | RUNE profiles shall declare supported descriptor concept families and fail closed when invariants, trace links, or extensions cannot be represented. | Core concept compatibility tests and profile catalog review. |
| RUNE-REQ-044 | RUNE shall define a deterministic derive evidence automation boundary before replacing manually maintained descriptor fixtures. | Review `docs/architecture/derive-evidence-automation.md` and Wave 5 pulse records. |
| RUNE-REQ-046 | RUNE shall define an explicit known-contract registry interface with deterministic ordering and fail-closed duplicate identity diagnostics before adding crate discovery. | Review `docs/architecture/explicit-registry-interface.md` and core registry tests. |

## Verification and validation requirements

| ID | Requirement | Verification |
|---|---|---|
| RUNE-REQ-040 | RUNE shall define verification commands for each pulse before treating a slice as complete. | Review active pulse validation block. |
| RUNE-REQ-041 | RUNE shall add macro compile tests and generated artifact checks before declaring macro or generator surfaces stable. | Future verification suite review. |
| RUNE-REQ-042 | RUNE shall validate usefulness through representative Rust repo bakeoffs before broad adoption. | Bakeoff scenario findings and review notes. |
| RUNE-REQ-045 | RUNE shall prove descriptor evidence can be produced or regenerated from annotated Rust code before broad adoption. | Future derive evidence automation tests. |
| RUNE-REQ-047 | RUNE shall retain scenario evidence across multiple neutral descriptor kinds before approving broad collection or downstream adapters. | Entity, command, and event known-contract scenario tests. |
| RUNE-REQ-048 | RUNE shall retain state descriptor scenario evidence before approving broad collection or downstream adapters. | State known-contract scenario tests. |
| RUNE-REQ-049 | RUNE shall retain artifact descriptor scenario evidence before approving broad collection or downstream adapters. | Artifact known-contract scenario tests. |
| RUNE-REQ-050 | RUNE shall retain source descriptor scenario evidence before approving broad collection or downstream adapters. | Source known-contract scenario tests. |
| RUNE-REQ-051 | RUNE shall retain evidence descriptor scenario evidence before approving broad collection or downstream adapters. | Evidence known-contract scenario tests. |
| RUNE-REQ-052 | RUNE shall retain multi-contract descriptor evidence in a durable collection document with collection identity and version before approving discovery or CLI collection. | Descriptor collection document tests and retained collection fixture. |
| RUNE-REQ-053 | RUNE shall fail closed on malformed descriptor collection evidence before approving discovery or CLI collection. | Descriptor collection draft validation tests. |
| RUNE-REQ-054 | RUNE shall expose read-only fixture-backed descriptor collection inspection before approving discovery-backed collection. | `rune inspect-collection --fixture <path>` and CLI inspection tests. |
| RUNE-REQ-055 | RUNE shall expose read-only fixture-backed descriptor collection/profile compatibility checks before approving discovery-backed collection. | `rune check-collection --profile rune.neutral_descriptor_json --fixture <path>` and CLI collection check tests. |
| RUNE-REQ-056 | RUNE shall emit retained neutral generated artifacts for descriptor collections only after collection validation and compatibility checks are verified. | `rune generate-collection --profile rune.neutral_descriptor_json --fixture <path>` and CLI collection generation tests. |
| RUNE-REQ-057 | RUNE shall emit retained neutral inventory summaries for descriptor collections before approving discovery-backed collection. | `rune inventory-collection --fixture <path>` and CLI inventory tests. |
| RUNE-REQ-058 | RUNE v1 shall include deterministic discovery, external profiles, and downstream adapters while keeping the neutral core descriptor model product-neutral. | V1 scope docs, later discovery/profile/adapter interface reviews, and representative repo bakeoffs. |
| RUNE-REQ-059 | RUNE v1 derives shall fail closed at compile time when durable descriptor identity or version is omitted. | `trybuild` missing id/version compile-fail tests. |
| RUNE-REQ-060 | RUNE shall support adopter-owned explicit registries with deterministic order, retained collection evidence, and duplicate descriptor diagnostics before discovery is implemented. | `examples/rune-adopter` registry workflow tests. |
| RUNE-REQ-061 | RUNE discovery shall be deterministic, manifest-controlled, fail closed on malformed inputs, and avoid arbitrary source scraping before source analysis is separately reviewed. | `docs/architecture/deterministic-discovery-interface.md` and later discovery tests. |
| RUNE-REQ-062 | RUNE shall implement manifest-based discovery over retained descriptor collection fixtures with deterministic merge order and retained discovered collection evidence. | `rune discover --manifest <path>`, core discovery tests, CLI discovery tests, and retained discovered collection fixture. |
| RUNE-REQ-063 | RUNE shall provide a read-only retained collection evidence workflow that bundles inspection, compatibility, inventory, and generated neutral artifact evidence from approved collection or discovery inputs. | `rune evidence-collection --profile rune.neutral_descriptor_json`, evidence CLI tests, and retained evidence bundle fixtures. |
| RUNE-REQ-064 | RUNE external profiles shall be reviewed profile-owned mappings over validated neutral descriptor or collection documents, with product vocabulary kept out of core descriptors. | `docs/architecture/external-profile-interface.md` and later external profile implementation tests. |
| RUNE-REQ-065 | RUNE shall implement a first external profile with retained generated artifacts and fail-closed compatibility checks while preserving neutral core vocabulary. | `rune.documentation_packet_json`, documentation packet fixtures, profile catalog tests, and generation tests. |
| RUNE-REQ-066 | RUNE downstream adapters shall be separate reviewed mappings from validated RUNE evidence or external profile outputs into consumer artifacts, with adapter vocabulary kept outside the neutral core. | `docs/architecture/downstream-adapter-interface.md` and later adapter implementation tests. |
| RUNE-REQ-067 | RUNE shall implement a first downstream adapter as a separate adapter surface with retained output evidence and fail-closed compatibility diagnostics. | `rune-adapters`, `rune.review_packet_json`, adapter catalog tests, CLI adapter tests, and retained review packet fixture. |
| RUNE-REQ-068 | RUNE CLI shall retain hardening coverage for current status text, unknown commands, usage failures, malformed JSON diagnostics, and adapter argument handling before v1 release readiness. | `crates/rune-cli/tests/cli_hardening.rs` and full workspace validation. |
| RUNE-REQ-069 | RUNE shall provide adopter-facing v1 examples and guide documentation covering derive, registry, retained evidence, external profile output, and adapter output. | `examples/rune-adopter`, `docs/adopter-guide.md`, adopter v1 workflow tests, and retained adopter fixtures. |
| RUNE-REQ-070 | RUNE shall validate representative Rust repo usefulness through a scenario bakeoff comparing source-only review against retained RUNE descriptor, profile, and adapter evidence. | QUIVER bakeoff fixtures, `docs/vtrace/BAKEOFF.md`, and CLI validation commands. |
| RUNE-REQ-071 | RUNE v1 shall close release readiness with CI-ready validation, crate surface documentation, compatibility policy, non-goals, and final VTRACE evidence. | `docs/release-readiness.md`, README validation block, final workspace validation, and Wave 32 records. |
| RUNE-REQ-072 | RUNE shall provide an adoption documentation package with indexed concepts, how-tos, tutorials, examples, trace walkthroughs, and corpus update rules for v1 adopters. | `docs/README.md`, `docs/CORPUS.md`, `docs/concepts/`, `docs/how-to/`, `docs/tutorials/`, `docs/examples/`, `docs/traces/`, and Wave 33 records. |
| RUNE-REQ-073 | RUNE shall provide an ordered adopter validation runbook over retained v1 fixtures and approved CLI commands. | `docs/runbooks/adopter-evidence-validation.md`, adopter fixtures, discovery manifest fixtures, and Wave 34 records. |
| RUNE-REQ-074 | RUNE shall support metadata-driven data-contract scenarios with derive-authored invariants/extensions, retained data-contract profile evidence, and games adoption guidance that starts from stable data/event/state/simulation surfaces before trait-only extraction. | `examples/rune-shape-calculator`, `rune.data_contract_json`, shape fixtures, `docs/concepts/data-contracts.md`, `docs/examples/shape-calculator.md`, and `docs/traces/games-contract-candidates.md`. |
| RUNE-REQ-075 | RUNE shall record a design change request before implementing field-level data metadata so descriptor shape, derive authoring, profile preservation, games adoption sequencing, and non-goals are reviewed before code changes. | `docs/vtrace/DCR.md#dcr-rune-001-field-level-data-metadata`. |
| RUNE-REQ-076 | RUNE shall implement explicit field-level data metadata with derive authoring, descriptor preservation, retained data-contract profile evidence, and fail-closed diagnostics for unsupported field metadata keys. | `FieldMetadataDescriptor`, `FieldMetadataDocument`, `#[rune_field(...)]`, shape fixtures, derive compile tests, and Wave 36 records. |
| RUNE-REQ-077 | RUNE shall record a Mission 2.0 controlled direction for becoming a managed native semantic runtime layer while preserving native Rust execution and product-neutral core vocabulary. | `docs/vtrace/MISSION_2_0.md`, `docs/vtrace/DCR.md#dcr-rune-002-mission-20-managed-native-semantic-runtime`, and Wave 41 records. |
| RUNE-REQ-078 | RUNE shall define a semantic registry lane for explicit crate/process contract registries before adding runtime discovery or host surfaces. | `docs/architecture/semantic-registry-interface.md`; future registry document tests. |
| RUNE-REQ-079 | RUNE shall define state graph and transition evidence lanes before exposing live state inspection. | `docs/architecture/state-graph-interface.md`; future retained state graph fixtures. |
| RUNE-REQ-080 | RUNE shall define evidence runtime packet lanes for diagnostics, validation, trace, health, and audit artifacts tied to descriptor ids. | `docs/architecture/evidence-runtime-packets.md`; future retained packet fixtures. |
| RUNE-REQ-081 | RUNE shall define an agent-safe protocol lane with read-first operations, capability declarations, and fail-closed diagnostics before exposing AI-facing runtime queries. | `docs/architecture/agent-protocol-interface.md`; future protocol fixture tests. |
| RUNE-REQ-082 | RUNE shall define compatibility negotiation across descriptor collections, profiles, adapters, and optional runtime hosts before automatic migration or degraded-mode claims. | `docs/architecture/compatibility-negotiation.md`; future compatibility report fixtures. |
| RUNE-REQ-083 | RUNE shall define capability and sensitivity policy metadata before exposing private data, mutating actions, or runtime-host endpoints. | `docs/architecture/capability-sensitivity-policy.md`; future policy compatibility checks. |
| RUNE-REQ-084 | RUNE shall keep optional runtime host implementation blocked until registry, state graph, evidence runtime, agent protocol, compatibility negotiation, and capability policy lanes have approved boundaries. | `docs/architecture/runtime-host-design.md` and Mission 2.0 readiness review. |
| RUNE-REQ-085 | RUNE shall implement a retained semantic registry document model with explicit scope, collection/profile/adapter references, capabilities, retained fixtures, and fail-closed validation while keeping runtime host behavior blocked. | `SemanticRegistryDraft`, `SemanticRegistryDocument`, semantic registry fixtures, and Wave 42 records. |
| RUNE-REQ-086 | RUNE CLI shall provide read-only semantic registry validation over retained registry fixtures and fail closed on unknown or unsupported declared profile/adapter catalog references. | `rune check-registry --fixture <path>`, registry CLI tests, and retained registry check fixture. |
| RUNE-REQ-087 | RUNE CLI shall validate retained semantic registry collection source refs against local descriptor collection fixtures and fail closed when a ref is missing, malformed, or mismatched. | `rune check-registry --fixture <path>`, `RUNE-REGISTRY-005`, registry source-ref tests, and retained collection fixtures. |
| RUNE-REQ-088 | RUNE CLI shall provide a read-only semantic registry inspection report that preserves validated registry metadata and summarizes retained collection refs without runtime behavior. | `rune inspect-registry --fixture <path>`, retained inspection fixture, and registry CLI tests. |
| RUNE-REQ-089 | RUNE shall provide final communications and readiness documentation that makes v1 readiness, Mission 2.0 Wave 42 registry evidence, adopter validation, and blocked runtime behaviors discoverable and synchronized. | `docs/vtrace/COMMUNICATIONS_STRATEGY.md`, registry how-to/runbook/example/trace docs, release readiness, corpus rules, and Wave 49 records. |
| RUNE-REQ-090 | RUNE shall implement retained state graph evidence documents keyed by semantic registry refs, descriptor-backed nodes, command/event transitions, ownership, evidence refs, and fail-closed diagnostics while keeping live state inspection and runtime host behavior blocked. | `StateGraphDraft`, `StateGraphDocument`, state graph fixtures, `rune check-state-graph --fixture <path> --registry <path>`, and Wave 43 records. |

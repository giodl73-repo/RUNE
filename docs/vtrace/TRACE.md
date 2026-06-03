# RUNE Trace

| Need | Requirement | Design element | Implementation | Verification | Evidence |
|---|---|---|---|---|---|
| NEED-004 | RUNE-REQ-001 | VTRACE proof package | `docs/vtrace/` | Review | This trace package |
| NEED-004 | RUNE-REQ-002 | Ordered VTRACE stage gates | `docs/vtrace/STAGES.md` | `git diff --check` and review | Stage gate review |
| NEED-003 | RUNE-REQ-003 | Product-neutral base API | `README.md`, `PRODUCT_PLAN.md`, `docs/architecture/descriptor-model.md`, `rune-core` | Review | Foundation review notes |
| NEED-001 | RUNE-REQ-010 | Neutral descriptor model | `crates/rune-core` | `cargo test --workspace` and review | Workspace test output; descriptor review |
| NEED-002 | RUNE-REQ-011 | Stable descriptor identity and versioning | `docs/architecture/interface-control.md` | Interface review | Stage 4 interface evidence |
| NEED-002 | RUNE-REQ-012 | Foundation concept vocabulary | `docs/architecture/descriptor-model.md` | Vocabulary review | Stage 3 review notes |
| NEED-004 | RUNE-REQ-013 | Traceability links in descriptors | `docs/architecture/interface-control.md`, `rune-derive`, `rune-core::DescriptorDocument` | Trace fixture review and derive-to-document test | Stage 4/6 evidence |
| NEED-001 | RUNE-REQ-020 | Derive macro surface | `docs/architecture/interface-control.md`, `crates/rune-derive` | Macro interface review and `cargo test --workspace` | Macro compile evidence |
| NEED-002 | RUNE-REQ-021 | Inspectable generated behavior | `docs/architecture/interface-control.md`, future tests and CLI output | Macro compile tests and inspection output | Stage 6 evidence |
| NEED-003 | RUNE-REQ-022 | Product-neutral macro API | `docs/architecture/interface-control.md` | Macro API review | Stage 4 review notes |
| NEED-002 | RUNE-REQ-030 | CLI inspection surface | `crates/rune-cli` | `cargo run -p rune-cli -- status` | CLI output |
| NEED-005 | RUNE-REQ-031 | Profile and generator boundary | `PRODUCT_PLAN.md`, `docs/architecture/descriptor-model.md` | Review | Foundation review notes |
| NEED-003 | RUNE-REQ-032 | Generator diagnostics for unsupported concepts | `docs/architecture/interface-control.md` | Generator spec review | Stage 4/6 evidence |
| NEED-002 | RUNE-REQ-033 | Read-only inspection surface | `docs/architecture/inspection-surface.md` | Interface review and future CLI tests | Contract-surface wave evidence |
| NEED-005 | RUNE-REQ-034 | Generator/profile interface | `docs/architecture/generator-profile-interface.md` | Interface review | Generator/profile wave evidence |
| NEED-005 | RUNE-REQ-035 | Read-only profile catalog | `docs/architecture/generator-profile-interface.md`, `rune-cli profile list` | CLI profile catalog test | Wave 3 profile catalog evidence |
| NEED-002 | RUNE-REQ-036 | Shared descriptor document boundary | `rune-core::DescriptorDraft`, `rune-core::DescriptorDocument`, `rune-cli` | Core draft validation tests and CLI fixture tests | Wave 3 descriptor document consolidation evidence |
| NEED-005 | RUNE-REQ-037 | Shared profile catalog boundary | `rune-core::ProfileCatalog`, `rune-core::ProfileDescriptor`, `rune-cli` | Core profile catalog tests and CLI profile/generate tests | Wave 3 profile metadata consolidation evidence |
| NEED-005 | RUNE-REQ-038 | Profile compatibility checks | `ProfileDescriptor::validate_descriptor_with_codes`, `rune-cli generate` | Core compatibility tests and CLI unsupported version/kind tests | Wave 3 profile compatibility evidence |
| NEED-004 | RUNE-REQ-039 | Read-only compatibility check | `CheckReportDocument`, `rune-cli check` | CLI check report and fail-closed tests | Wave 3 check-command evidence |
| NEED-005 | RUNE-REQ-043 | Profile concept compatibility | `ProfileDescriptor.supported_concepts`, concept compatibility diagnostics | Core unsupported concept tests and profile catalog output | Wave 3 concept compatibility evidence |
| NEED-004 | RUNE-REQ-044 | Derive evidence automation boundary | `docs/architecture/derive-evidence-automation.md` | Interface gate review | Wave 5 automation gate evidence |
| NEED-004 | RUNE-REQ-046 | Explicit known-contract registry interface | `docs/architecture/explicit-registry-interface.md`, `ContractRegistration`, `collect_known_contract_documents` | Core registry collection tests and derive collection fixture test | Wave 7 registry interface evidence |
| NEED-004 | RUNE-REQ-040 | Pulse verification commands | `context/waves/` | Pulse review | Active pulse validation blocks |
| NEED-004 | RUNE-REQ-041 | Macro and artifact verification | Future verification suite | Verification review | Stage 6 evidence |
| NEED-005 | RUNE-REQ-042 | Representative repo bakeoffs | Future bakeoff scenarios | Validation review | Stage 7 findings |
| NEED-004 | RUNE-REQ-045 | Deterministic descriptor evidence from annotated code | `DescriptorDocument::from_contract`, known-contract derive tests, retained descriptor collection fixture | Derive evidence and known-contract collection tests | Wave 5 automation and Wave 6 collection evidence |
| NEED-005 | RUNE-REQ-047 | Multi-kind scenario coverage | Entity, command, and event annotated contracts | Derive/check/generate tests and retained collection fixture | Wave 8 contract-kind coverage evidence |
| NEED-005 | RUNE-REQ-048 | State scenario coverage | `CustomerLifecycleState` annotated state contract | Derive/check/generate tests and retained collection fixture | Wave 9 state contract coverage evidence |
| NEED-005 | RUNE-REQ-049 | Artifact scenario coverage | `ContractEvidenceArtifact` annotated artifact contract | Derive/check/generate tests and retained collection fixture | Wave 10 artifact contract coverage evidence |
| NEED-005 | RUNE-REQ-050 | Source scenario coverage | `ContractSourceReference` annotated source contract | Derive/check/generate tests and retained collection fixture | Wave 11 source contract coverage evidence |
| NEED-005 | RUNE-REQ-051 | Evidence scenario coverage | `ContractVerificationEvidence` annotated evidence contract | Derive/check/generate tests and retained collection fixture | Wave 12 evidence contract coverage evidence |
| NEED-004 | RUNE-REQ-052 | Durable descriptor collection document | `DescriptorCollectionDocument`, retained known-contract collection fixture | Core collection document test and derive collection fixture test | Wave 13 collection document evidence |
| NEED-004 | RUNE-REQ-053 | Collection document validation | `DescriptorCollectionDraft::validate_with_codes` | Core collection draft validation tests | Wave 14 collection validation evidence |
| NEED-002 | RUNE-REQ-054 | Read-only collection inspection | `rune inspect-collection --fixture`, `docs/architecture/inspection-surface.md` | CLI collection inspection tests | Wave 15 collection inspection evidence |
| NEED-004 | RUNE-REQ-055 | Read-only collection/profile compatibility checks | `CheckCollectionReportDocument`, `rune check-collection --profile` | Core collection report test and CLI collection check tests | Wave 16 collection check evidence |
| NEED-005 | RUNE-REQ-056 | Neutral collection generated artifacts | `GeneratedCollectionArtifactDocument`, `rune generate-collection --profile` | Core collection artifact test and CLI collection generation tests | Wave 17 collection generation evidence |
| NEED-002 | RUNE-REQ-057 | Neutral collection inventory summaries | `DescriptorCollectionInventoryDocument`, `rune inventory-collection --fixture` | Core inventory test and CLI inventory tests | Wave 18 collection inventory evidence |
| NEED-005 | RUNE-REQ-058 | V1 completion boundary | `README.md`, `PRODUCT_PLAN.md`, later discovery/profile/adapter specs | Documentation review and later interface reviews | Wave 19 v1 scope evidence |
| NEED-001 | RUNE-REQ-059 | V1 derive identity/version diagnostics | `docs/architecture/derive-v1-ergonomics.md`, `rune-derive` | Missing id/version compile-fail tests | Wave 20 derive ergonomics evidence |
| NEED-004 | RUNE-REQ-060 | Crate-owned explicit registry workflow | `docs/architecture/crate-owned-registry-workflow.md`, `examples/rune-adopter` | Adopter registry workflow tests | Wave 21 crate-owned registry evidence |
| NEED-004 | RUNE-REQ-061 | Deterministic discovery interface | `docs/architecture/deterministic-discovery-interface.md` | Interface review and later discovery tests | Wave 22 discovery interface evidence |
| NEED-004 | RUNE-REQ-062 | Manifest discovery implementation | `DiscoveryManifestDocument`, `DescriptorCollectionDocument::from_discovered_collections`, `rune discover --manifest` | Core discovery tests and CLI discovery tests | Wave 23 discovery implementation evidence |
| NEED-004 | RUNE-REQ-063 | Read-only retained evidence workflow | `docs/architecture/retained-evidence-workflow.md`, `CollectionEvidenceBundleDocument`, `rune evidence-collection` | Core bundle test and CLI evidence tests | Wave 24 retained evidence workflow evidence |
| NEED-005 | RUNE-REQ-064 | External profile boundary | `docs/architecture/external-profile-interface.md` | Interface review and later profile tests | Wave 25 external profile interface evidence |
| NEED-005 | RUNE-REQ-065 | Documentation packet profile | `DocumentationPacketDocument`, `rune.documentation_packet_json`, `rune generate`, `rune generate-collection` | Core documentation packet tests and CLI generation tests | Wave 26 first external profile evidence |
| NEED-003 | RUNE-REQ-066 | Downstream adapter boundary | `docs/architecture/downstream-adapter-interface.md` | Interface review and later adapter tests | Wave 27 downstream adapter interface evidence |
| NEED-003 | RUNE-REQ-067 | Review packet adapter | `rune-adapters`, `AdapterCatalog`, `ReviewPacketDocument`, `rune adapt-collection` | Adapter crate tests and CLI adapter tests | Wave 28 first adapter evidence |
| NEED-004 | RUNE-REQ-068 | CLI hardening coverage | `rune-cli` status, command dispatch, usage parsers, JSON readers, adapter parser | CLI hardening tests and workspace validation | Wave 29 CLI hardening evidence |
| NEED-001 | RUNE-REQ-069 | Adopter v1 workflow | `examples/rune-adopter`, `docs/adopter-guide.md` | Adopter registry and v1 workflow tests | Wave 30 adopter example evidence |
| NEED-005 | RUNE-REQ-070 | Representative repo bakeoff | `docs/vtrace/fixtures/quiver_bakeoff_*`, `docs/vtrace/BAKEOFF.md` | RUNE CLI check/generate/adapt commands and workspace validation | Wave 31 representative repo bakeoff evidence |
| NEED-004 | RUNE-REQ-071 | V1 release readiness | `docs/release-readiness.md`, `README.md`, `PRODUCT_PLAN.md`, final VTRACE records | CI-ready validation commands and final workspace validation | Wave 32 v1 release readiness evidence |
| NEED-001 | RUNE-REQ-072 | Adoption documentation package | `docs/README.md`, `docs/CORPUS.md`, `docs/concepts/`, `docs/how-to/`, `docs/tutorials/`, `docs/examples/`, `docs/traces/` | Documentation review and `git diff --check` | Wave 33 adoption docs evidence |
| NEED-004 | RUNE-REQ-073 | Adopter validation runbook | `docs/runbooks/adopter-evidence-validation.md`, retained adopter and discovery fixtures | Runbook command sequence and workspace validation | Wave 34 adoption validation runbook evidence |
| NEED-005 | RUNE-REQ-074 | Metadata-driven data contracts and games adoption guidance | `rune.data_contract_json`, `examples/rune-shape-calculator`, `docs/concepts/data-contracts.md`, `docs/examples/shape-calculator.md`, `docs/traces/games-contract-candidates.md` | Shape example tests, CLI profile tests, workspace validation | Wave 35 shape and games contract evidence |
| NEED-005 | RUNE-REQ-075 | Field metadata DCR | `docs/vtrace/DCR.md` | DCR review and `git diff --check` | DCR-RUNE-001 |
| NEED-005 | RUNE-REQ-076 | Field-level metadata implementation | `FieldMetadataDescriptor`, `FieldMetadataDocument`, `#[rune_field(...)]`, `examples/rune-shape-calculator` | Core, derive, shape, CLI, and workspace tests | Wave 36 field metadata evidence |
| NEED-006 / NEED-007 / NEED-008 / NEED-009 / NEED-010 | RUNE-REQ-077 | Mission 2.0 managed native semantic runtime direction | `docs/vtrace/MISSION_2_0.md`, `docs/vtrace/DCR.md` | DCR review and `git diff --check` | Wave 41 Mission 2.0 control package |
| NEED-006 | RUNE-REQ-078 | Semantic registry lane | `docs/architecture/semantic-registry-interface.md` | Interface review and future registry fixtures | Wave 42 planning complete |
| NEED-006 | RUNE-REQ-085 | Semantic registry implementation | `SemanticRegistryDraft`, `SemanticRegistryDocument`, registry refs, capabilities, retained fixtures | `cargo test -p rune-core --quiet` and workspace validation | Wave 42 semantic registry evidence |
| NEED-006 / NEED-007 | RUNE-REQ-079 | State graph and transition evidence lane | `docs/architecture/state-graph-interface.md` | Interface review and future state graph fixtures | Wave 43 planning complete |
| NEED-008 | RUNE-REQ-080 | Evidence runtime packet lane | `docs/architecture/evidence-runtime-packets.md` | Interface review and future packet fixtures | Wave 44 planning complete |
| NEED-007 / NEED-010 | RUNE-REQ-081 | Agent-safe protocol lane | `docs/architecture/agent-protocol-interface.md` | Interface review and future protocol fixtures | Wave 45 planning complete |
| NEED-009 | RUNE-REQ-082 | Compatibility negotiation lane | `docs/architecture/compatibility-negotiation.md` | Interface review and future compatibility report fixtures | Wave 46 planning complete |
| NEED-010 | RUNE-REQ-083 | Capability and sensitivity policy lane | `docs/architecture/capability-sensitivity-policy.md` | Interface review and future policy checks | Wave 47 planning complete |
| NEED-006 / NEED-007 / NEED-008 / NEED-009 / NEED-010 | RUNE-REQ-084 | Optional runtime host design blocker | `docs/architecture/runtime-host-design.md` | Interface review and future host fixtures | Wave 48 planning complete |

## Specification Visibility

| Spec ID | Source IDs | Trace Surface | Evidence |
|---|---|---|---|
| SPEC-RUNE-001 | RUNE-REQ-010 / RUNE-REQ-011 / RUNE-REQ-012 / RUNE-REQ-013 | Neutral descriptor model | Descriptor model docs, core tests, retained descriptor evidence |
| SPEC-RUNE-002 | RUNE-REQ-020 / RUNE-REQ-021 / RUNE-REQ-044 / RUNE-REQ-059 | Derive evidence boundary | Derive tests, trybuild tests, retained descriptor fixture |
| SPEC-RUNE-003 | RUNE-REQ-030 / RUNE-REQ-033 / RUNE-REQ-039 / RUNE-REQ-054 / RUNE-REQ-068 | CLI inspection and compatibility | CLI status, inspect, check, and hardening tests |
| SPEC-RUNE-004 | RUNE-REQ-052 / RUNE-REQ-053 / RUNE-REQ-055 / RUNE-REQ-056 / RUNE-REQ-057 / RUNE-REQ-063 | Retained collection evidence | Collection check, generate, inventory, and evidence bundle fixtures |
| SPEC-RUNE-005 | RUNE-REQ-034 / RUNE-REQ-035 / RUNE-REQ-037 / RUNE-REQ-038 / RUNE-REQ-043 / RUNE-REQ-064 / RUNE-REQ-065 / RUNE-REQ-074 | Profile and generator boundary | Profile catalog, documentation packet profile evidence, and data-contract profile evidence |
| SPEC-RUNE-006 | RUNE-REQ-060 / RUNE-REQ-061 / RUNE-REQ-062 / RUNE-REQ-066 / RUNE-REQ-067 | Discovery and adapter boundary | Adopter registry, discovery manifest, and review packet adapter evidence |
| SPEC-RUNE-007 | RUNE-REQ-040 / RUNE-REQ-041 / RUNE-REQ-042 / RUNE-REQ-045 / RUNE-REQ-070 / RUNE-REQ-071 / RUNE-REQ-072 / RUNE-REQ-073 / RUNE-REQ-074 | Validation, release, adoption docs, runbooks, and games adoption guidance | Workspace validation, QUIVER bakeoff, release readiness, docs package, adopter validation runbook, shape scenario, and games survey |
| SPEC-RUNE-008 | RUNE-REQ-075 / RUNE-REQ-076 | Design change control and field metadata | DCR-RUNE-001 records planned field-level metadata scope before implementation; Wave 36 implements the approved first slice with retained evidence. |
| SPEC-RUNE-009 | RUNE-REQ-077 / RUNE-REQ-078 / RUNE-REQ-079 / RUNE-REQ-080 / RUNE-REQ-081 / RUNE-REQ-082 / RUNE-REQ-083 / RUNE-REQ-084 / RUNE-REQ-085 | Mission 2.0 managed native semantic runtime | DCR-RUNE-002 upgrades the mission and `docs/architecture/mission-2-planning-index.md` closes planning for semantic registry, state graph, evidence runtime, agent protocol, compatibility negotiation, capability policy, and optional runtime host lanes; DCR-RUNE-003 implements the first retained semantic registry document slice. |

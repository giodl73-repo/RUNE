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

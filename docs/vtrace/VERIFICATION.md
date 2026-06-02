# RUNE Verification

## Foundation validation commands

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
```

## Verification expectations

| Command | Checks |
|---|---|
| `cargo fmt --check` | Rust formatting is stable. |
| `cargo test --workspace` | Core descriptor tests, derive runtime tests, and proc-macro compile pass/fail tests pass. |
| `cargo run -p rune-cli -- status` | The CLI entry point is present and reports the foundation surface. |
| `git diff --check` | Markdown and source changes have no whitespace errors. |

## Foundation verification coverage

| Surface | Verification | Status |
|---|---|---|
| Descriptor shape | `rune-core` unit test checks `id`, `version`, `kind`, fields, invariants, trace links, and extensions. | covered |
| Derive descriptor output | `rune-derive` integration test checks `#[derive(RuneContract)]` output, including neutral requirement trace links. | covered |
| Macro compile-pass behavior | `trybuild` pass fixture checks accepted `id`, `version`, `kind`, and `requirement` attributes. | covered |
| Macro compile-fail behavior | `trybuild` fail fixture checks unsupported `#[rune(schema = ...)]` is rejected. | covered |
| CLI status behavior | `cargo run -p rune-cli -- status` reports approved surfaces and deferred commands. | covered |
| Inspection pass behavior | `rune-cli` integration test compares fixture-backed inspection output to retained expected JSON. | covered |
| Inspection fail-closed behavior | `rune-cli` integration test verifies missing descriptor identity returns `RUNE-INSP-001`. | covered |
| Neutral generated artifact checks | `rune-cli` integration tests compare `rune.neutral_descriptor_json` output to retained expected JSON and verify `RUNE-GEN-001` / `RUNE-GEN-003` failures for missing identity, unsupported profile, and unsupported descriptor kind. | covered |
| Derive-to-document checks | `rune-derive` integration test serializes an annotated `Customer` type through `rune-core::DescriptorDocument` and compares it to the retained descriptor fixture. | covered |
| Profile catalog checks | `rune-cli` integration test compares `profile list` output to retained expected JSON and rejects unknown profile subcommands. | covered |
| Shared descriptor-document validation | `rune-core` unit tests validate descriptor drafts into descriptor documents and reject missing identity; `rune-cli` consumes this shared model for inspect and generate. | covered |
| Shared profile catalog validation | `rune-core` unit test verifies the approved neutral descriptor profile metadata; `rune-cli profile list` and `generate` consume the shared catalog. | covered |
| Profile compatibility validation | `rune-core` unit tests and `rune-cli` integration tests reject profile-unsupported descriptor versions and valid-but-profile-unsupported descriptor kinds. | covered |
| Read-only compatibility checks | `rune-cli` integration tests compare `check` output to retained expected JSON and reject unsupported descriptor version/kind without emitting generated artifacts. | covered |
| Profile concept compatibility validation | `rune-core` unit tests reject invariants, trace links, and extensions when a profile does not declare the corresponding supported concept. | covered |
| Deterministic derive evidence path | `rune-derive` integration tests serialize the annotated `Customer` descriptor deterministically and provide an opt-in retained evidence writer. | covered |
| Retained evidence regeneration | Targeted `rune-derive` integration test can regenerate `annotated_customer_descriptor.json` from the annotated `Customer` type when `RUNE_UPDATE_EVIDENCE=1`. | covered |
| Known command contract evidence | `rune-derive` and `rune-cli` integration tests cover the annotated `CreateCustomer` command descriptor, compatibility check, and neutral generated artifact. | covered |
| Known-contract collection evidence | `rune-derive` integration tests serialize the explicit `Customer` and `CreateCustomer` known-contract set and compare it to a retained collection fixture. | covered |
| Explicit registry helper | `rune-core` unit tests verify ordered known-contract collection and fail-closed duplicate descriptor id diagnostics. | covered |
| Known event contract evidence | `rune-derive` and `rune-cli` integration tests cover the annotated `CustomerCreated` event descriptor, compatibility check, neutral generated artifact, and registry collection fixture. | covered |
| Known state contract evidence | `rune-derive` and `rune-cli` integration tests cover the annotated `CustomerLifecycleState` state descriptor, compatibility check, neutral generated artifact, and registry collection fixture. | covered |
| Known artifact contract evidence | `rune-derive` and `rune-cli` integration tests cover the annotated `ContractEvidenceArtifact` artifact descriptor, compatibility check, neutral generated artifact, and registry collection fixture. | covered |
| Known source contract evidence | `rune-derive` and `rune-cli` integration tests cover the annotated `ContractSourceReference` source descriptor, compatibility check, neutral generated artifact, and registry collection fixture. | covered |
| Known evidence contract evidence | `rune-derive` and `rune-cli` integration tests cover the annotated `ContractVerificationEvidence` evidence descriptor, compatibility check, neutral generated artifact, and registry collection fixture. | covered |
| Descriptor collection document | `rune-core` unit tests verify collection identity/version metadata, and `rune-derive` integration tests compare the retained collection document fixture. | covered |
| Descriptor collection validation | `rune-core` unit tests validate collection drafts and reject missing collection identity, missing collection version, and duplicate descriptor ids. | covered |
| Collection inspection CLI | `rune-cli` integration tests compare fixture-backed collection inspection output and reject malformed collection fixtures. | covered |
| Collection compatibility check CLI | `rune-core` unit tests preserve collection check report context; `rune-cli` integration tests compare fixture-backed collection check output and reject malformed or profile-unsupported collection fixtures. | covered |
| Collection neutral generation CLI | `rune-core` unit tests preserve collection artifact context; `rune-cli` integration tests compare fixture-backed collection generation output and reject malformed, unknown-profile, or profile-unsupported collection fixtures. | covered |
| Collection inventory CLI | `rune-core` unit tests count descriptor kinds; `rune-cli` integration tests compare fixture-backed collection inventory output and reject malformed collection fixtures. | covered |
| V1 scope documentation | README, product plan, VTRACE requirements, trace, validation, bakeoff, review, and Wave 19 records define the expanded v1 boundary. | covered |
| Adopter quick start documentation | README demonstrates derive metadata, explicit registration, retained evidence commands, and read-only validation posture. | covered |
| V1 derive identity/version diagnostics | `rune-derive` compile-fail tests reject derives that omit required `id` or `version` attributes. | covered |
| Crate-owned registry workflow | `rune-adopter` tests verify an adopter-owned registry, retained collection evidence, deterministic ordering, and duplicate descriptor diagnostics. | covered |
| Deterministic discovery interface | Architecture and VTRACE records define manifest-based discovery ordering, diagnostics, and non-goals before implementation. | covered |
| Deterministic discovery implementation | `rune-core` tests validate discovery manifest documents and deterministic discovered collection merges; `rune-cli` tests compare retained discovery output and fail-closed diagnostics. | covered |
| Retained evidence workflow | `rune-core` tests preserve evidence bundle surfaces; `rune-cli` tests compare bundle fixtures for retained collection and discovery manifest inputs. | covered |
| External profile interface | Architecture and VTRACE records define the external profile boundary, metadata, inputs, outputs, diagnostics, and non-goals before implementation. | covered |
| First external profile | `rune-core` tests cover documentation packet profile/catalog behavior; `rune-cli` tests compare retained descriptor and collection documentation packet fixtures. | covered |
| Downstream adapter interface | Architecture and VTRACE records define adapter boundary, inputs, metadata, outputs, diagnostics, and non-goals before implementation. | covered |
| First downstream adapter | `rune-adapters` tests cover catalog, compatibility, and review packet output; `rune-cli` tests compare adapter catalog and retained review packet fixtures. | covered |
| CLI hardening | `rune-cli` hardening tests cover status text, unknown commands, usage failures, malformed JSON diagnostics, invalid adapter argument order, and adapter subcommand usage. | covered |
| Adopter v1 workflow | `rune-adopter` tests cover crate-owned registry evidence, documentation packet evidence, and review packet adapter evidence. | covered |
| Representative repo bakeoff | QUIVER scenario fixtures retain descriptor collection, compatibility check, documentation packet, and review packet evidence generated through RUNE CLI commands. | covered |
| V1 release readiness | Release readiness docs define crate surfaces, CI-ready validation, compatibility policy, retained evidence, and v1 non-goals. | covered |

## Evidence rule

Each pulse records the validation commands it expects. Future generated contract
artifacts should be retained as evidence or regenerated deterministically.

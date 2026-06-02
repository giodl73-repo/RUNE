# RUNE Validation

## Engineering decision supported

Define how RUNE will prove usefulness in representative Rust repos without
letting consumer scenarios rewrite the neutral core model.

## Validation posture

RUNE validation is **partially executed**. The current slice can derive, verify,
inspect, check, and generate neutral fixture-backed descriptor evidence. It does
not yet expose arbitrary Rust crate analysis, external profiles, or downstream
profile output.

Stage 7 records fixture-backed bakeoffs and the remaining gate before broader
repo validation.

## First bakeoff scenario

| Field | Value |
|---|---|
| Scenario | Inspect a retained neutral descriptor fixture and compare RUNE inspection output against source-only/prose-only interpretation. |
| Candidate target | `crates/rune-cli/tests/fixtures/valid_descriptor.json`, a small non-critical fixture, not an enterprise consumer integration. |
| Baseline | Human/agent reads Rust source and README prose without generated RUNE metadata. |
| RUNE path | Read-only inspection exposes neutral descriptor metadata with id, version, kind, fields, invariants, trace links, and extensions. |
| Comparison question | Does RUNE make the type's contract surface easier to inspect, validate, and map without guessing from source text? |

## Required preconditions

| ID | Precondition | Status |
|---|---|---|
| VAL-PRE-001 | Neutral descriptor slice is verified. | met |
| VAL-PRE-002 | Inspection or generator interface is approved by Stage 4-equivalent review. | met for read-only inspection |
| VAL-PRE-003 | Generated artifact or inspection output can be retained as evidence. | met for fixture-backed inspection and neutral generated artifact |
| VAL-PRE-004 | Bakeoff target is selected as a scenario, not a core dependency. | met for fixture-backed inspection |
| VAL-PRE-005 | Compatibility can be checked without emitting generated artifacts. | met for `rune check` |
| VAL-PRE-006 | Profile compatibility covers version, kind, and first-slice concept families. | met for neutral profile |

## Success criteria

The first bakeoff passes only if it produces evidence for all of these:

| Criterion | Evidence |
|---|---|
| Inspectability | pass: the contract surface can be read without expanding macros manually. |
| Neutrality | pass: the output avoids downstream product vocabulary. |
| Identity/versioning | pass: durable output preserves descriptor `id` and `version`. |
| Unsupported concepts | partial pass: missing identity fails closed; broader unsupported concept coverage remains future work. |
| AI usefulness | provisional pass: basic contract-shape questions are answerable without source scraping. |
| Review usefulness | pass: a reviewer can trace claims back to RUNE requirements and verification evidence. |

## Deferred execution gate

Do not run a broader repo bakeoff until RUNE can inspect a small annotated Rust
type or emit retained descriptor evidence from a build/test path. This gate is
partially met for a single annotated type through retained fixture evidence; it
is not met for arbitrary crate discovery.

The next valid validation path is a scenario-only comparison that uses existing
neutral RUNE outputs:

- `rune check` as read-only descriptor/profile compatibility evidence,
- `rune generate --profile rune.neutral_descriptor_json` as retained neutral
  artifact evidence,
- source/prose-only review as the baseline.

Each option requires an interface-control update, verification updates, and role
review before execution.

## First bakeoff evidence

Detailed findings are recorded in `docs/vtrace/BAKEOFF.md`.

The Wave 3 annotated-type bakeoff now covers a small `Customer` type with
retained descriptor and neutral generated artifact evidence. Automatic crate-wide
discovery remains a future validation gate.

The derive-to-document bridge now proves that the retained descriptor fixture can
be regenerated from the annotated `Customer` type through `RuneContract` and
`DescriptorDocument`. This is still a single-type validation, not a repository
discovery pass.

## Wave 4 scenario gate

The first repo-adoption bakeoff is constrained to the existing annotated
`Customer` contract and retained evidence under `crates/rune-cli/tests/fixtures/`.
It will compare source/prose-only review against RUNE check and generated neutral
artifact evidence. It must not add external profiles, product adapters, or broad
crate discovery.

## Wave 4 source-only baseline

The source-only baseline has been captured for
`crates/rune-derive/tests/derive_contract.rs`. It shows that a reviewer can see
the Rust type and fields directly, but must infer or already know RUNE macro
semantics to understand durable identity, version, kind, and requirement trace
semantics. Profile compatibility, generated artifact shape, and fail-closed
behavior are not answerable from source-only review.

## Wave 4 RUNE evidence comparison

RUNE evidence has been compared against the source-only baseline for the
annotated `Customer` scenario. The descriptor fixture, compatibility check
report, and neutral generated artifact make identity, version, kind, trace link,
profile compatibility, profile metadata, and artifact shape explicit.

This is a positive scenario-level validation result, with limits: evidence is
still fixture-backed, scoped to one annotated type, and does not demonstrate
arbitrary crate discovery or external profile generation.

## Wave 4 readiness decision

Wave 4 passes for controlled scenario-level usefulness and remains blocked for
broad adoption. RUNE evidence is better than source-only review for the annotated
`Customer` scenario, but the evidence still depends on retained fixtures.

The next validation prerequisite is descriptor evidence automation: RUNE must
produce or collect descriptor JSON from annotated Rust code through a
deterministic build/test-oriented path before external profiles, product
adapters, or broad repo adoption are approved.

## Wave 5 automation gate

The derive evidence automation boundary is defined in
`docs/architecture/derive-evidence-automation.md`. The first implementation slice
may serialize known annotated contracts to `DescriptorDocument` evidence through
tests or a controlled build/test-oriented example. Arbitrary crate discovery,
external profiles, and product adapters remain blocked.

The first deterministic evidence path is test-only and opt-in. Setting
`RUNE_UPDATE_EVIDENCE=1` for the targeted derive integration test rewrites the
retained annotated customer descriptor from `DescriptorDocument::from_contract`.
Normal validation compares the retained fixture without mutating it.

The retained annotated customer descriptor has an explicit regeneration check:
the opt-in writer can rewrite the fixture from the annotated type, while the
standard derive integration test verifies the retained output still matches the
derived descriptor document.

## Wave 5 readiness decision

Wave 5 passes for deterministic regeneration of one known annotated contract.
This reduces the manual fixture dependency for the controlled `Customer`
descriptor, but it does not prove multi-contract evidence collection or arbitrary
crate discovery.

The next validation prerequisite is known-contract evidence collection: RUNE must
show that multiple explicitly registered annotated contracts can produce stable
descriptor evidence before broad adoption, source discovery, or external profiles
are approved.

## Wave 6 command scenario

RUNE now includes a second explicitly registered known contract:
`CreateCustomer`, a neutral command descriptor with fields `customer_id` and
`email`. It has retained descriptor, compatibility check, and neutral generated
artifact evidence. This expands coverage from one entity contract to entity plus
command scenarios while still avoiding arbitrary crate discovery.

## Wave 6 multi-contract collection

The derive integration test now serializes the explicit known-contract set into
`crates/rune-cli/tests/fixtures/known_contract_descriptor_collection.json`. The
collection currently includes `Customer` and `CreateCustomer` in a stable order.

This meets the scenario-level gate for deterministic multi-contract evidence
collection through an explicit test registry. It remains blocked for arbitrary
crate discovery, external profiles, and product adapters.

## Wave 6 readiness decision

Wave 6 passes for controlled known-contract evidence collection. RUNE now has
retained deterministic evidence for one entity descriptor, one command
descriptor, and one stable descriptor collection containing both known contracts.

The next validation gate is not broader adoption. Before adding repo-scale
collection, RUNE must define a reviewed explicit registry or discovery interface
with deterministic ordering, diagnostics, and evidence-retention rules.

## Wave 7 explicit registry interface

RUNE now defines a core explicit registry boundary:
`ContractRegistration` plus `collect_known_contract_documents`. The helper
preserves caller-provided order and rejects duplicate descriptor ids with
`RUNE-REG-001`.

This improves the Wave 6 known-contract evidence path by moving collection from
an ad hoc test array into a neutral core helper. It still does not approve crate
discovery, CLI collection, external profiles, product adapters, or broad repo
adoption.

## Wave 7 readiness decision

Wave 7 passes for controlled explicit registry collection. The registry helper is
sufficient for known-contract evidence paths that explicitly name descriptor
providers and retain output evidence.

The next validation prerequisite is a separate discovery or CLI collection
interface. Broad adoption remains blocked until that surface is specified,
implemented, and validated with retained evidence.

## Wave 8 contract-kind coverage

RUNE now includes a third explicitly registered known contract:
`CustomerCreated`, a neutral event descriptor with fields `customer_id` and
`occurred_at`. It has retained descriptor, compatibility check, neutral
generated artifact, and collection evidence.

Wave 8 passes for bounded scenario coverage across entity, command, and event
descriptor kinds. This remains explicit-registry evidence only; broad discovery,
external profiles, and product adapters are still blocked.

## Wave 9 state contract coverage

RUNE now includes a fourth explicitly registered known contract:
`CustomerLifecycleState`, a neutral state descriptor with fields `customer_id`,
`status`, and `updated_at`. It has retained descriptor, compatibility check,
neutral generated artifact, and collection evidence.

Wave 9 passes for bounded state descriptor evidence. This remains
explicit-registry evidence only; broad discovery, external profiles, and product
adapters are still blocked.

## Wave 10 artifact contract coverage

RUNE now includes a fifth explicitly registered known contract:
`ContractEvidenceArtifact`, a neutral artifact descriptor with fields
`descriptor_id`, `artifact_uri`, and `artifact_kind`. It has retained descriptor,
compatibility check, neutral generated artifact, and collection evidence.

Wave 10 passes for bounded artifact descriptor evidence. This remains
explicit-registry evidence only; broad discovery, external profiles, and product
adapters are still blocked.

## Wave 11 source contract coverage

RUNE now includes a sixth explicitly registered known contract:
`ContractSourceReference`, a neutral source descriptor with fields
`descriptor_id`, `source_path`, and `source_symbol`. It has retained descriptor,
compatibility check, neutral generated artifact, and collection evidence.

Wave 11 passes for bounded source descriptor evidence. This remains
explicit-registry evidence only; broad discovery, external profiles, and product
adapters are still blocked.

## Wave 12 evidence contract coverage

RUNE now includes a seventh explicitly registered known contract:
`ContractVerificationEvidence`, a neutral evidence descriptor with fields
`descriptor_id`, `verification_id`, and `evidence_uri`. It has retained
descriptor, compatibility check, neutral generated artifact, and collection
evidence.

Wave 12 passes for bounded coverage of all descriptor kinds currently supported
by the approved neutral profile. This remains explicit-registry evidence only;
broad discovery, external profiles, product adapters, and the valid but
profile-unsupported `other` kind are still blocked.

## Wave 13 descriptor collection document

RUNE now wraps retained known-contract descriptor evidence in a durable
`DescriptorCollectionDocument` with `collection_id`, `collection_version`, and
ordered descriptor documents.

Wave 13 passes for durable collection evidence from the explicit registry. This
does not approve crate discovery, source inference, CLI collection, external
profiles, product adapters, or the valid but profile-unsupported `other` kind.

## Wave 14 collection document validation

RUNE now validates descriptor collection drafts into durable collection
documents. Validation fails closed for missing collection identity, missing
collection version, and duplicate descriptor ids.

Wave 14 passes for core collection validation readiness. This does not approve
crate discovery, source inference, CLI collection, external profiles, product
adapters, or the valid but profile-unsupported `other` kind.

## Wave 15 collection inspection CLI

RUNE now exposes `rune inspect-collection --fixture <path>` as a read-only
fixture-backed inspection surface for validated descriptor collection documents.
The command emits retained neutral collection JSON and fails closed for missing
collection identity, missing collection version, and duplicate descriptor ids.

Wave 15 passes for read-only collection inspection. This does not approve crate
discovery, source inference, external profiles, product adapters, or the valid
but profile-unsupported `other` kind.

## Wave 16 collection compatibility check CLI

RUNE now exposes
`rune check-collection --profile rune.neutral_descriptor_json --fixture <path>`
as a read-only fixture-backed compatibility surface for descriptor collection
documents. The command emits retained collection check evidence and fails closed
for missing collection identity, duplicate descriptor ids, and profile-
unsupported descriptor kinds.

Wave 16 passes for read-only collection compatibility checks. This does not
approve crate discovery, source inference, generated collection artifacts,
external profiles, product adapters, or the valid but profile-unsupported
`other` kind.

## Wave 17 collection neutral generation CLI

RUNE now exposes
`rune generate-collection --profile rune.neutral_descriptor_json --fixture <path>`
as a fixture-backed neutral generation surface for descriptor collection
documents. The command emits retained collection artifact evidence and fails
closed for missing collection identity, duplicate descriptor ids, unknown
profiles, and profile-unsupported descriptor kinds.

Wave 17 passes for fixture-backed neutral collection generation. This does not
approve crate discovery, source inference, external profiles, product adapters,
or the valid but profile-unsupported `other` kind.

## Wave 18 collection inventory CLI

RUNE now exposes `rune inventory-collection --fixture <path>` as a read-only
fixture-backed inventory surface for descriptor collection documents. The command
emits retained collection inventory evidence with total descriptor count and
deterministic kind counts.

Wave 18 passes for fixture-backed neutral collection inventory. This does not
approve crate discovery, source inference, external profiles, product adapters,
or the valid but profile-unsupported `other` kind.

## Wave 19 v1 scope and documentation reconciliation

RUNE v1 is now scoped to include deterministic discovery, external profiles, and
downstream adapters in addition to the neutral core, derive macro, CLI evidence
surfaces, and release readiness. The core constraint remains unchanged:
downstream vocabulary must stay out of `rune-core` and the base descriptor
model.

Wave 19 passes for documentation and planning readiness. It does not implement
or approve discovery, external profiles, or adapters; each requires a later
interface-control wave with retained evidence.

## Wave 20 derive v1 ergonomics

RUNE now treats missing derive `id` or `version` attributes as compile-time
errors. The v1 derive ergonomics spec records the supported authoring surface and
explicitly defers field-level metadata, enum variant descriptors, invariant
authoring macros, doc-comment capture, source inference, and adapter hints.

Wave 20 passes for durable derive identity/version hardening. This does not
approve product-specific macro vocabulary or unreviewed descriptor expansion.

## Wave 21 crate-owned registry workflow

RUNE now includes `examples/rune-adopter`, a workspace example crate that owns
annotated contracts, exposes a deterministic `RUNE_CONTRACTS` registry, builds a
descriptor collection document, compares retained collection evidence, and fails
closed on duplicate descriptor ids.

Wave 21 passes for explicit crate-owned registries. This does not approve crate
scanning, Cargo metadata discovery, source inference, external profiles, product
adapters, or linker-section inventory.

## Wave 22 deterministic discovery interface

RUNE now defines a deterministic discovery interface. The first implementation
candidate is manifest-based discovery over retained descriptor collection
fixtures, preserving manifest order and source order while failing closed on
malformed manifests, unsupported source kinds, unreadable sources, and duplicate
descriptor ids.

Wave 22 passes for interface-control only. It does not implement or approve
executable registry hooks, source analysis, Cargo metadata traversal, external
profiles, adapters, or product-specific discovery vocabulary.

## Wave 23 deterministic discovery implementation

RUNE now exposes `rune discover --manifest <path>` for deterministic
manifest-based discovery over retained descriptor collection fixtures. The
adopter example collection can be merged into retained discovered collection
evidence without scanning source, traversing Cargo metadata, or executing crate
hooks.

| Area | Result | Finding |
|---|---|---|
| Manifest-controlled discovery | pass | Discovery input is an explicit manifest with manifest and output collection identity/version. |
| Deterministic merge order | pass | Output preserves manifest entry order and each source collection's descriptor order. |
| Fail-closed diagnostics | pass | Missing manifest identity, unsupported source kinds, malformed source collections, and duplicate descriptor ids fail closed. |
| Scope control | pass | External profiles, product adapters, source scraping, and arbitrary crate discovery remain deferred. |

## Wave 24 retained evidence workflow

RUNE now exposes
`rune evidence-collection --profile rune.neutral_descriptor_json` for read-only
evidence bundle generation from either a retained descriptor collection fixture
or an approved discovery manifest.

| Area | Result | Finding |
|---|---|---|
| Read-only posture | pass | The command writes to stdout; fixture mutation is only by explicit redirection. |
| Evidence coverage | pass | The bundle includes collection, check report, inventory, and generated neutral artifact surfaces. |
| Discovery input | pass | Approved manifest discovery can feed the same evidence bundle shape. |
| Scope control | pass | External profiles, adapters, source scraping, and Cargo traversal remain deferred. |

## Wave 25 external profile interface

RUNE now defines how external profiles may be added without changing neutral
core descriptors. Profiles are reviewed mappings over validated descriptor or
collection documents and own their output vocabulary below the profile artifact
boundary.

| Area | Result | Finding |
|---|---|---|
| Core neutrality | pass | External profile vocabulary is forbidden from core descriptor fields and derive attributes. |
| Input contract | pass | Profiles consume validated descriptor or collection documents only. |
| Diagnostics | pass | Unsupported versions, kinds, concept families, and serialization failures require fail-closed diagnostics. |
| Implementation | blocked | The first external profile waits for the next wave. |

## Wave 26 first external profile

RUNE now implements `rune.documentation_packet_json` as the first external
profile. It emits deterministic documentation packets for validated descriptor
and collection inputs while reusing existing profile compatibility checks.

| Area | Result | Finding |
|---|---|---|
| External profile output | pass | Descriptor and collection documentation packet fixtures are retained. |
| Profile catalog | pass | `profile list` exposes the external profile metadata. |
| Core neutrality | pass | Documentation packet vocabulary is isolated to generated profile artifacts. |
| Scope control | pass | No downstream adapter, source scraping, Cargo traversal, or executable hook was added. |

## Wave 27 downstream adapter interface

RUNE now defines downstream adapters as separately reviewed mappings from
validated RUNE evidence or external profile outputs into consumer artifacts.
Adapter vocabulary remains outside the neutral core.

| Area | Result | Finding |
|---|---|---|
| Adapter boundary | pass | Adapters consume validated RUNE evidence or profile outputs, not raw source. |
| Vocabulary isolation | pass | Consumer terms are allowed only inside adapter artifacts and adapter docs. |
| Diagnostics | pass | Unsupported inputs, kinds, concepts, malformed evidence, and serialization require fail-closed diagnostics. |
| Implementation | blocked | First adapter implementation waits for the next wave. |

## Wave 28 first adapter implementation

RUNE now includes a separate `rune-adapters` crate and the first generic
downstream adapter, `rune.review_packet_json`. The adapter consumes validated
descriptor collections and emits deterministic review packet artifacts.

| Area | Result | Finding |
|---|---|---|
| Separate adapter surface | pass | Adapter types live in `rune-adapters`, outside `rune-core`. |
| Retained adapter evidence | pass | The review packet fixture is retained under CLI fixtures. |
| Fail-closed diagnostics | pass | Unknown adapters, malformed collections, and unsupported descriptor kinds fail closed. |
| Scope control | pass | No product-specific adapter, source scraping, Cargo traversal, or executable hook was added. |

## Wave 29 CLI hardening

RUNE now has targeted CLI hardening coverage for current v1 status text,
unknown commands, missing flags, malformed JSON diagnostics, invalid adapter
argument ordering, and adapter subcommand usage.

| Area | Result | Finding |
|---|---|---|
| Current status text | pass | Status output names v1 surfaces, approved profiles, and approved adapters. |
| Usage failures | pass | Missing flags and invalid adapter argument order fail with usage messages. |
| Parse diagnostics | pass | Malformed JSON fails closed with the expected diagnostic family. |
| Command dispatch | pass | Unknown commands and adapter subcommands fail closed. |

## Wave 30 adopter examples

RUNE now includes adopter-facing v1 workflow documentation and retained example
evidence in `examples\rune-adopter`.

| Area | Result | Finding |
|---|---|---|
| Adopter guide | pass | `docs/adopter-guide.md` documents derive, registry, retained evidence, profile output, and adapter output. |
| Example workflow | pass | `rune-adopter` tests compare retained collection, documentation packet, and review packet evidence. |
| Retained fixtures | pass | Adopter profile and adapter outputs are retained under the example crate. |
| Scope control | pass | The example remains explicit and deterministic without source scraping or Cargo traversal. |

## Wave 31 representative repo bakeoff

RUNE was applied to the non-RUNE `C:\src\quiver` Rust workspace as a scenario
only. The source-only baseline required reading `quiver-core`,
`quiver-manifest`, `quiver-runtime`, and `quiver-cli` to infer contract roles.
The RUNE path retained a descriptor collection plus compatibility, documentation
packet, and review packet evidence.

| Area | Result | Finding |
|---|---|---|
| Source-only baseline | pass with burden | Rust source exposes fields and serde derives, but stable contract ids, review prompts, and profile compatibility are not explicit. |
| RUNE descriptor evidence | pass | `quiver_bakeoff_descriptor_collection.json` makes selected contract ids, versions, kinds, fields, and trace links explicit. |
| Profile evidence | pass | `quiver_bakeoff.documentation_packet.json` summarizes contract shape for AI/reviewer use. |
| Adapter evidence | pass | `quiver_bakeoff.review_packet.json` emits deterministic review items. |
| Adoption limit | partial | The scenario uses retained RUNE evidence without modifying QUIVER; native derive adoption would be a future repo-specific integration. |

## Wave 32 v1 release readiness

RUNE now records v1 release readiness, compatibility policy, CI-ready validation,
crate surfaces, retained evidence expectations, and v1 non-goals.

| Area | Result | Finding |
|---|---|---|
| CI-ready validation | pass | `cargo fmt --check`, `cargo test --workspace`, `cargo run -p rune-cli -- status`, and `git diff --check` are the release gate. |
| Compatibility policy | pass | Descriptor, collection, profile, adapter, fixture, and diagnostic compatibility rules are documented. |
| Evidence completeness | pass | V1 evidence covers neutral core, derive, discovery, evidence bundles, external profile, adapter, adopter guide, and bakeoff. |
| Release scope | pass | V1 non-goals explicitly exclude source scraping, Cargo traversal, executable hooks, product vocabulary in core, and automatic downstream publication. |

## Validation command

```powershell
git diff --check
```

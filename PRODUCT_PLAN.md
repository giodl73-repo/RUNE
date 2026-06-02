# RUNE Product Plan

## Thesis

RUNE makes Rust projects directly legible to AI-era tooling by deriving neutral,
versioned contracts from Rust code. Those contracts can feed schema generators,
agent systems, trace systems, state machines, documentation, review gates,
contract tests, and future platform adapters without making core Rust libraries
depend on any downstream product.

## Users

- Rust library authors who want their APIs and entities to be machine-readable.
- AI tool builders who need typed, versioned metadata instead of source scraping.
- Platform teams defining schemas, manifests, traces, receipts, or state maps.
- Review and assurance teams that need traceable evidence from code to contract.
- Portfolio owners who want comparable contract quality across Rust repos.

## Foundation scope

RUNE begins as a small Rust workspace:

- `rune-core` defines neutral descriptor traits and records.
- `rune-derive` derives descriptors from structs and enums.
- `rune-cli` emits foundation status now and may add inspection/generation
  commands after stage review.
- `docs/vtrace/` records mission, requirements, trace, verification, and review
  evidence for the project itself.

## Extension model

RUNE core stays neutral. Downstream systems should integrate through adapters,
profiles, or generators:

```text
rune descriptor -> json schema
rune descriptor -> idl/csdl/openapi/profile-specific schema
rune descriptor -> trace/event contract
rune descriptor -> state/transition graph
rune descriptor -> documentation/review packet
```

The base specs should name generic concepts such as entity, event, command,
state, artifact, source, requirement, invariant, evidence, and version. They
should not name a specific agent platform or context product.

## VTRACE posture

RUNE treats VTRACE as its engineering standard:

- mission and stakeholder needs before broad implementation,
- requirements before macro surface expansion,
- interface and descriptor design before generator claims,
- verification commands for every pulse,
- evidence records for generated outputs,
- review gates before downstream adoption.

## V1 completion boundary

RUNE v1 is intentionally broader than the neutral foundation. It includes:

- neutral core descriptor and collection stability,
- deterministic crate-owned registration and discovery,
- retained evidence generation from approved registry/discovery inputs,
- external profile generation through reviewed profile boundaries,
- downstream adapters through reviewed adapter boundaries,
- representative repo bakeoffs,
- release readiness.

The v1 boundary does not allow downstream vocabulary to move into `rune-core` or
the base descriptor model. External profiles and adapters must remain separate,
traceable surfaces.

## Waves

### Wave 1: Foundation

Create the repo, neutral product thesis, VTRACE proof package, initial crates,
and first descriptor derive.

### Wave 2: Contract surface

Define stable descriptor families for entity, event, command, state, artifact,
invariant, source reference, evidence, and version compatibility.

### Wave 3: Generators and profiles

Add generator interfaces for schema, docs, trace, state-map, and IDL-style
outputs. Keep product-specific adapters outside the core model.

### Wave 4: Repo adoption bakeoff

Apply RUNE to selected Rust repos as scenarios. Measure whether generated
contracts improve AI comprehension, change review, validation, and handoff.

### Wave 5: Derive evidence automation

Generate or collect descriptor evidence from annotated Rust code through a
deterministic build/test-oriented path, removing manual fixture dependency before
external profiles or broad adoption.

### Wave 6: Known contract evidence collection

Collect descriptor evidence for multiple explicitly registered annotated
contracts before attempting arbitrary crate discovery or external profile
generation.

### Wave 7: Explicit registry interface

Define and verify a neutral explicit registry boundary for ordered known-contract
collection before adding crate discovery, repo-wide scanning, or downstream
adapters.

### Wave 8: Contract kind coverage

Expand retained known-contract scenarios across more neutral descriptor kinds
while continuing to use explicit registry collection only.

### Wave 9: State contract coverage

Add retained state descriptor evidence as the next neutral contract-kind scenario
before considering discovery or adapter surfaces.

### Wave 10: Artifact contract coverage

Add retained artifact descriptor evidence as the next neutral contract-kind
scenario while keeping collection explicit and product-neutral.

### Wave 11: Source contract coverage

Add retained source descriptor evidence as the next neutral contract-kind scenario
while keeping collection explicit and product-neutral.

### Wave 12: Evidence contract coverage

Add retained evidence descriptor coverage and close the first profile-supported
neutral kind set before any discovery or adapter work.

### Wave 13: Descriptor collection document

Wrap known-contract evidence in a durable collection document with its own
identity and version before adding discovery or CLI collection surfaces.

### Wave 14: Collection document validation

Fail closed on invalid collection evidence before any future collection input is
accepted by CLI or discovery surfaces.

### Wave 15: Collection inspection CLI

Expose read-only fixture-backed inspection for validated descriptor collection
documents without adding crate discovery.

### Wave 16: Collection compatibility check CLI

Expose read-only fixture-backed profile compatibility checks for descriptor
collection documents before any discovery-backed collection workflow.

### Wave 17: Collection neutral generation CLI

Emit retained neutral generated artifacts for descriptor collection documents
after collection inspection and compatibility checks are verified.

### Wave 18: Collection inventory CLI

Emit retained neutral inventory summaries for descriptor collection documents so
AI and review tools can assess contract-kind coverage without reading every
descriptor.

### Wave 19: V1 scope and documentation reconciliation

Record the expanded v1 boundary and reconcile product-facing docs with the
implemented CLI surface before discovery, external profiles, or adapters begin.

### Wave 20: Derive and descriptor ergonomics review

Review the derive macro and descriptor model for v1 usability in real Rust
crates, adding or explicitly deferring field metadata, enum/state ergonomics,
documentation capture, and diagnostics.

### Wave 21: Crate-owned explicit registry workflow

Define how a consumer crate exposes an explicit known-contract registry with
deterministic ordering and retained evidence.

### Wave 22: Deterministic discovery interface

Specify discovery before implementation, preferring deterministic manifest or
module registration over broad source scraping.

### Wave 23: Deterministic discovery implementation

Implement the approved discovery boundary with retained collection evidence and
fail-closed diagnostics.

Status: implemented for manifest-based discovery over retained descriptor
collection fixtures.

### Wave 24: Retained evidence generation workflow

Provide a standard opt-in workflow for regenerating descriptor, collection,
check, inventory, and generated artifact evidence from approved registry or
discovery inputs.

Status: implemented for read-only collection evidence bundles from retained
collection fixtures or approved discovery manifests.

### Wave 25: External profile interface

Define how non-neutral profile outputs are added without changing the neutral
core descriptor model.

Status: interface-control complete. External profiles are mappings over
validated descriptor or collection documents with profile-owned vocabulary and
fail-closed diagnostics.

### Wave 26: First external profile

Implement one external profile with retained artifacts and unsupported-concept
diagnostics.

Status: implemented as `rune.documentation_packet_json` for descriptor and
collection documentation packets.

### Wave 27: Adapter interface

Define how downstream adapters stay outside `rune-core` while remaining
traceable and testable.

Status: interface-control complete. Adapters are deterministic mappings from
validated RUNE evidence or external profile outputs into named consumer
artifacts, with consumer vocabulary kept outside the neutral core.

### Wave 28: First adapters

Implement selected adapter surfaces with retained evidence and compatibility
diagnostics.

Status: implemented first generic adapter as `rune.review_packet_json` in the
separate `rune-adapters` crate.

### Wave 29: CLI hardening

Normalize CLI usage, diagnostics, parse errors, and output shapes across
descriptor, collection, profile, discovery, and adapter commands.

Status: implemented regression coverage for status output, unknown commands,
usage failures, malformed JSON diagnostics, and adapter argument handling.

### Wave 30: Examples and adopter guide

Add a complete adopter path covering derive, registry, discovery, profiles,
adapters, and retained evidence.

Status: implemented adopter guide and extended `examples\rune-adopter` with
retained documentation packet and review packet evidence.

### Wave 31: Representative repo bakeoff

Apply RUNE to representative Rust repo scenarios and compare source-only review
against RUNE evidence, profile output, and adapters.

Status: executed against the non-RUNE `C:\src\quiver` workspace as a scenario
only, with retained RUNE descriptor, check, documentation packet, and review
packet evidence.

### Wave 32: V1 release readiness

Close CI, Cargo metadata, compatibility policy, docs, validation, and release
evidence for v1.

Status: complete. Release readiness and compatibility policy are recorded in
`docs/release-readiness.md`, with full workspace validation as the CI-ready gate.

### Wave 33: Adoption docs package

Turn the single adopter guide into a Craftworks-style documentation package with
concepts, how-tos, tutorials, examples, trace walkthroughs, and corpus
governance.

Status: complete as a documentation-only adoption package. No new CLI behavior,
discovery expansion, or product-specific core vocabulary was added.

## Non-goals

- Do not encode BAKER, LATTICE, FLETCHER, ARCADE, or any other product names in
  base RUNE specs.
- Do not make RUNE depend on enterprise consumer repos.
- Do not promise full Rust semantic analysis in the foundation wave.
- Do not hide generated behavior behind macros that cannot be inspected.
- Do not accept unversioned durable contracts.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
```

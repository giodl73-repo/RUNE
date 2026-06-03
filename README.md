# RUNE

**Rust Universal Notation Engine.**

RUNE is an enterprise Rust infrastructure project for deriving AI-readable
contracts from Rust code without coupling the base model to any one agent
platform, context substrate, workflow runtime, or product vocabulary.

It is the Rust-era sibling of IDL/MIDL/WSDL/CSDL-style contract systems:
source types and functions remain idiomatic Rust, while RUNE emits stable,
versioned contract descriptors that downstream systems can transform into the
formats they need.

## Purpose

AI systems increasingly need to understand Rust projects as more than source
text. They need durable answers to questions such as:

- What entity, event, command, state, or artifact does this Rust type represent?
- What fields, identifiers, versions, invariants, and lifecycle states exist?
- What source, requirement, design, verification, and evidence links justify it?
- What can be generated for schemas, manifests, traces, receipts, state maps,
  examples, and review packets?
- What changed when a type or contract evolves?

RUNE starts from compile-time derivation and explicit metadata, not runtime
reflection.

```text
Rust code + RUNE derives/attributes
  -> neutral contract descriptors
  -> generated schemas, manifests, maps, traces, receipts, docs, and adapters
```

Mission 2.0 extends this into a managed native semantic runtime direction:
native Rust binaries can gain managed-runtime-style semantic affordances
through explicit registries, retained evidence, state/evidence documents,
agent-safe protocols, compatibility negotiation, and capability metadata without
turning RUNE into a VM or product orchestrator.

## What RUNE is not

- Not a new Rust compiler or custom Rust syntax fork.
- Not an agent runtime.
- Not a context substrate.
- Not a product-specific AgentMap, lattice, workflow, or conversation format.
- Not tied to one AI provider, orchestrator, schema language, or serialization
  format.

Other systems may consume RUNE output, but the core RUNE specs do not encode
their terms.

## Foundation crates

| Crate | Purpose |
|---|---|
| `rune-adapters` | Downstream adapter surfaces over validated RUNE evidence. |
| `rune-core` | Neutral contract descriptor types and traits. |
| `rune-derive` | Procedural macros that derive RUNE descriptors from Rust types. |
| `rune-cli` | Fixture-backed inspection, compatibility checks, neutral generation, collection inventory, profile catalog, adapter catalog, and semantic registry check commands. |

## Current CLI surface

RUNE currently supports neutral, fixture-backed evidence workflows:

```powershell
cargo run -p rune-cli -- status
cargo run -p rune-cli -- profile list
cargo run -p rune-cli -- inspect --fixture <descriptor.json>
cargo run -p rune-cli -- inspect-collection --fixture <collection.json>
cargo run -p rune-cli -- inventory-collection --fixture <collection.json>
cargo run -p rune-cli -- discover --manifest <discovery-manifest.json>
cargo run -p rune-cli -- evidence-collection --profile rune.neutral_descriptor_json --fixture <collection.json>
cargo run -p rune-cli -- evidence-collection --profile rune.neutral_descriptor_json --manifest <discovery-manifest.json>
cargo run -p rune-cli -- adapter list
cargo run -p rune-cli -- adapt-collection --adapter rune.review_packet_json --fixture <collection.json>
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture <descriptor.json>
cargo run -p rune-cli -- check-collection --profile rune.neutral_descriptor_json --fixture <collection.json>
cargo run -p rune-cli -- check-registry --fixture <semantic-registry.json>
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture <descriptor.json>
cargo run -p rune-cli -- generate-collection --profile rune.neutral_descriptor_json --fixture <collection.json>
cargo run -p rune-cli -- generate --profile rune.documentation_packet_json --fixture <descriptor.json>
cargo run -p rune-cli -- generate-collection --profile rune.documentation_packet_json --fixture <collection.json>
```

These commands are deliberately bounded. Manifest discovery merges retained
descriptor collection fixtures in declared order; arbitrary crate scanning and
downstream adapter adoption remain out of scope for the core surface.

The `evidence-collection` command is read-only. It emits a portable evidence
bundle containing the validated collection, compatibility check report,
inventory, and neutral generated collection artifact; callers opt in to mutation
by redirecting the output to a retained fixture.

The first external profile is `rune.documentation_packet_json`. It emits a
deterministic documentation packet over validated descriptors or collections
without adding documentation-packet vocabulary to the neutral descriptor model.

The first downstream adapter is `rune.review_packet_json`. It lives in
`rune-adapters` and converts validated descriptor collections into deterministic
review packets without adding adapter vocabulary to `rune-core`.

The first semantic registry command is `check-registry`. It validates retained
registry metadata and cross-checks declared profile/adapter ids against approved
catalogs. It also validates retained descriptor collection source refs relative
to the registry fixture. It does not inspect Cargo metadata, scrape source,
discover plugins, mutate registry state, or enable runtime host behavior.

CLI hardening covers current status text, unknown commands, usage failures,
malformed JSON diagnostics, invalid adapter argument order, and adapter
subcommand usage.

## Quick start

Annotate Rust types with stable neutral contract metadata:

```rust
use rune_core::{ContractRegistration, RuneContract};
use rune_derive::RuneContract as DeriveRuneContract;

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.customer",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-034"
)]
struct Customer {
    id: String,
    email: String,
}

const KNOWN_CONTRACTS: &[ContractRegistration] = &[
    ContractRegistration {
        name: "Customer",
        descriptor: Customer::descriptor,
    },
];
```

The derive emits a neutral descriptor with stable identity, version, kind, Rust
type, fields, and requirement trace links. Explicit registries keep collection
order deterministic and make duplicate descriptor identities fail closed.

Retained evidence is JSON generated from those descriptors. Current fixture-backed
commands let reviewers and tools inspect, summarize, check, and generate neutral
artifacts from retained descriptor or collection evidence:

```powershell
cargo run -p rune-cli -- inspect --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- inspect-collection --fixture crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.json
cargo run -p rune-cli -- inventory-collection --fixture crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.json
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- check-collection --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.json
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- generate-collection --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.json
```

Normal validation is read-only. Retained evidence regeneration is intentionally
opt-in in the current derive test harness so fixtures do not mutate during
ordinary test runs.

## Crate-owned registry example

`examples\rune-adopter` shows the v1 adopter pattern:

- the crate owns annotated contracts,
- the crate exposes an explicit `RUNE_CONTRACTS` registry,
- the registry order is deterministic,
- retained collection evidence is compared in tests,
- duplicate descriptor identities fail closed.

See `docs\adopter-guide.md` for the v1 adopter path through retained
collection evidence, documentation packet profile output, and review packet
adapter output.

Run it with:

```powershell
cargo test -p rune-adopter
```

## Documentation paths

The full adoption documentation package starts at `docs\README.md`:

- `docs\concepts\` explains descriptors, evidence, discovery, profiles, and
  adapters.
- `docs\how-to\` gives task-oriented procedures for the current CLI workflows.
- `docs\tutorials\` walks through the adopter path lesson by lesson.
- `docs\examples\` indexes copyable adopter and fixture references.
- `docs\traces\` connects commands and retained artifacts to review decisions.
- `docs\CORPUS.md` records documentation update obligations.

## V1 completion target

RUNE v1 is complete when it provides:

- stable neutral core descriptor and collection models,
- production-ready derive ergonomics for real Rust crates,
- deterministic crate-owned registration and discovery,
- retained evidence generation workflows,
- at least one external profile implemented outside the neutral core model,
- downstream adapter surfaces that keep product vocabulary out of `rune-core`,
- representative repo bakeoff evidence,
- release-ready docs, validation, and compatibility policy.

Release readiness and compatibility policy are recorded in
`docs\release-readiness.md`.

## VTRACE operating model

RUNE uses VTRACE-style engineering from the beginning. Every non-trivial
contract feature should trace from mission need to requirement, design,
implementation, verification, validation, and objective evidence.

The source-of-truth proof package starts under `docs/vtrace/`.
Mission 2.0 is recorded in `docs\vtrace\MISSION_2_0.md` and
`docs\vtrace\DCR.md`.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
git diff --check
```

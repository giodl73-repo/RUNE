# RUNE Principles

These entries summarize durable RUNE decision rules for neutral Rust contracts,
retained evidence, profiles, adapters, compatibility, and agent-safe surfaces.

## RUNE-P-01: Neutral Core Before Adapters

**Status:** ACTIVE

**Statement:** RUNE core descriptors use product-neutral Rust contract concepts;
downstream product, agent, workflow, schema, or context vocabulary belongs in
profiles, adapters, or generated artifacts outside `rune-core`.

**Rationale:** RUNE is useful across repos only if its base model is not quietly
captured by one consumer's language.

**Decision rule:** A new term may enter `rune-core` only when it is a neutral
descriptor concept; consumer-specific terms must be mapped by an external
profile or adapter.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`docs/vtrace/SPECIFICATION_BASELINE.md`, and `.roles/ROLE.md`.

## RUNE-P-02: Durable Contracts Start With Identity And Version

**Status:** ACTIVE

**Statement:** Descriptor, collection, registry, evidence, compatibility, and
protocol artifacts need explicit identity and version before they are treated as
durable evidence.

**Rationale:** Agents, generators, reviewers, and adopters cannot compare or
retain contract evidence safely when ids or versions are implicit.

**Decision rule:** Missing identity, missing version, duplicate ids, unsupported
versions, or mismatched retained refs fail closed instead of falling back to
best-effort inference.

**Evidence:** `docs/vtrace/REQUIREMENTS.md`, `docs/vtrace/TRACE.md`,
`docs/architecture/interface-control.md`, and `docs/release-readiness.md`.

## RUNE-P-03: Retained Evidence Beats Source Scraping

**Status:** ACTIVE

**Statement:** RUNE validates retained descriptor, collection, registry, state,
evidence, protocol, and compatibility fixtures before adding broad crate
scanning, live runtime inspection, or source scraping.

**Rationale:** Retained evidence gives reviewers stable artifacts that can be
checked, regenerated, compared, and cited without depending on prompt
conventions or hidden traversal.

**Decision rule:** New discovery or runtime surfaces must start from explicit
registry, manifest, or retained evidence boundaries and preserve deterministic
order.

**Evidence:** `README.md`, `docs/architecture/retained-evidence-workflow.md`,
`docs/architecture/deterministic-discovery-interface.md`, and
`docs/vtrace/VERIFICATION.md`.

## RUNE-P-04: Profiles And Adapters Fail Closed

**Status:** ACTIVE

**Statement:** Profiles and adapters map validated RUNE evidence into external
formats only when unsupported versions, kinds, concepts, or degradation are
reported explicitly.

**Rationale:** Silent omission or unapproved degradation makes generated output
look safer and more complete than the retained evidence supports.

**Decision rule:** Compatibility checks must run before generated, profile, or
adapter output is accepted; unsupported or degraded output remains blocked or
warning-labeled.

**Evidence:** `docs/architecture/generator-profile-interface.md`,
`docs/architecture/downstream-adapter-interface.md`,
`docs/architecture/compatibility-negotiation.md`, and
`docs/vtrace/TRACE.md`.

## RUNE-P-05: Runtime Power Requires Capability Gates

**Status:** ACTIVE

**Statement:** Agent protocol, runtime host, private data, mutating actions,
policy enforcement, and automatic migration remain blocked until capability,
sensitivity, compatibility, and runtime-host boundaries are explicitly approved.

**Rationale:** A contract system can become an unsafe control plane if read-only
evidence checks quietly become live authority.

**Decision rule:** AI-facing and runtime-facing work starts as retained
read-first validation; mutation, live endpoints, restricted data, runtime host,
and migration paths require separate reviewed gates.

**Evidence:** `docs/architecture/agent-protocol-interface.md`,
`docs/architecture/capability-sensitivity-policy.md`,
`docs/architecture/runtime-host-design.md`, and `docs/vtrace/REVIEW.md`.

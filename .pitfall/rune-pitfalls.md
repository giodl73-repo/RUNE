# RUNE Pitfalls

These entries capture recurring Rust contract-infrastructure failure classes
and cite RUNE's existing countermeasures.

## RUNE-PF-01: Downstream Vocabulary Leaks Into Core

**Status:** MITIGATED

**Pattern:** A consumer, agent platform, workflow system, schema format, or
context product gets its own terms encoded in `rune-core` descriptors.

**Domain:** Descriptor model, derive attributes, generated artifacts, profile
catalog, adapters, docs, and Mission 2.0 lanes.

**Detection difficulty:** Consumer vocabulary often feels like harmless
metadata until it becomes a compatibility promise for every adopter.

**Structural solution:** Keep core descriptor terms neutral, route consumer
vocabulary through reviewed profiles/adapters, and require role review before
new stable concepts enter core.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`,
`docs/vtrace/SPECIFICATION_BASELINE.md`, and `.roles/ROLE.md`.

## RUNE-PF-02: Macro Convenience Hides Contract Behavior

**Status:** MITIGATED

**Pattern:** Derive macros generate durable AI-facing contract facts that are
not inspectable, deterministic, trace-linked, or compile-time checked.

**Domain:** `rune-derive`, descriptor fixtures, field metadata, adopter
examples, retained evidence regeneration, and documentation examples.

**Detection difficulty:** Macros make adoption easy, so reviewers may miss
missing id/version fields, unsupported attributes, or hidden output changes.

**Structural solution:** Use explicit derive metadata, compare generated
descriptors to retained fixtures, keep evidence rewrites opt-in, and fail closed
with compile-fail tests for unsupported or incomplete attributes.

**Evidence:** `docs/vtrace/VERIFICATION.md`,
`docs/architecture/derive-evidence-automation.md`,
`crates/rune-derive/tests/compile.rs`, and
`crates/rune-derive/tests/derive_contract.rs`.

## RUNE-PF-03: Source Scraping Replaces Retained Evidence

**Status:** MITIGATED

**Pattern:** RUNE or an adopter infers contracts by broad source traversal,
Cargo scanning, prompt conventions, or live runtime inspection instead of
approved registries, manifests, and retained fixtures.

**Domain:** Discovery, collection evidence, semantic registry, state graph,
evidence packets, runtime host planning, and agent protocol requests.

**Detection difficulty:** Source scraping can look more automatic and complete
than explicit retained evidence, but it weakens reproducibility and review.

**Structural solution:** Use crate-owned registries, manifest-controlled
discovery, retained descriptor collection refs, read-only fixture validation,
and explicit runtime-host blockers.

**Evidence:** `README.md`, `docs/architecture/deterministic-discovery-interface.md`,
`docs/architecture/retained-evidence-workflow.md`, and
`docs/vtrace/TRACE.md`.

## RUNE-PF-04: Compatibility Degrades Silently

**Status:** MITIGATED

**Pattern:** Unsupported versions, unsupported concepts, adapter limitations,
or degraded behavior are treated as compatible output or automatic migration.

**Domain:** Profile generation, adapter output, semantic registry refs,
state graph refs, evidence packets, compatibility reports, and release
readiness.

**Detection difficulty:** Generated output can appear structurally valid even
when fields, concepts, or refs were omitted or downgraded.

**Structural solution:** Run explicit compatibility checks, fail closed on
unsupported versions or concepts, require approved degradation, and keep
automatic migration blocked.

**Evidence:** `docs/architecture/compatibility-negotiation.md`,
`docs/release-readiness.md`, `docs/vtrace/VERIFICATION.md`, and
`cargo test -p rune-cli --test compatibility_cli`.

## RUNE-PF-05: Read-First Protocol Becomes Runtime Authority

**Status:** MITIGATED

**Pattern:** A retained agent protocol, state graph, evidence packet, or
compatibility report is mistaken for permission to run a live endpoint, mutate
state, expose restricted data, enforce policy, or host runtime behavior.

**Domain:** Agent protocol, capability policy, optional runtime host, private
data access, live state inspection, and future AI tooling integrations.

**Detection difficulty:** A validated request shape can be mistaken for an
authorization or execution surface because it resembles an API contract.

**Structural solution:** Keep protocol validation read-first, block mutating and
restricted-data requests, separate capability/sensitivity policy, and leave
runtime host implementation gated behind explicit Mission 2.0 review.

**Evidence:** `docs/architecture/agent-protocol-interface.md`,
`docs/architecture/capability-sensitivity-policy.md`,
`docs/architecture/runtime-host-design.md`, and `docs/vtrace/REVIEW.md`.

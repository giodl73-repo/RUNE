# RUNE Invariants

These entries summarize properties that must remain true for RUNE descriptor
models, derive behavior, retained evidence, adapters, and agent-safe protocols.

## RUNE-I-01: Core Descriptors Stay Neutral

**Status:** VERIFIED

**Claim:** Core descriptors carry stable id, version, kind, fields,
invariants, trace links, and extensions without downstream product vocabulary.

**Why it matters:** Neutral descriptors let multiple repos, profiles, and
adapters consume the same Rust contract evidence without rewriting the core
model for each product.

**Enforcement:** `SPEC-RUNE-001`, `RUNE-REQ-003`, and the Contract Model Steward
role preserve product-neutral descriptor vocabulary.

**Evidence:** `docs/vtrace/SPECIFICATION_BASELINE.md`, `docs/vtrace/TRACE.md`,
`docs/architecture/descriptor-model.md`, and `cargo test --workspace`.

## RUNE-I-02: Derive Output Is Inspectable And Fail-Closed

**Status:** VERIFIED

**Claim:** Procedural macros emit deterministic descriptor evidence and reject
missing identity, missing version, and unsupported attributes at compile time.

**Why it matters:** Generated contract behavior must be reviewable by humans and
agents instead of hidden behind macro expansion.

**Enforcement:** `SPEC-RUNE-002`, `RUNE-REQ-020`, `RUNE-REQ-021`,
`RUNE-REQ-059`, derive tests, and `trybuild` fixtures cover accepted and
rejected macro shapes.

**Evidence:** `docs/vtrace/VERIFICATION.md`,
`crates/rune-derive/tests/compile.rs`,
`crates/rune-derive/tests/derive_contract.rs`, and `cargo test --workspace`.

## RUNE-I-03: Registry And Discovery Are Explicit And Deterministic

**Status:** VERIFIED

**Claim:** Crate-owned registries, manifest discovery, and retained collection
workflows preserve declared order and fail closed on malformed, missing,
duplicate, or mismatched refs.

**Why it matters:** Contract collection evidence is only reusable when another
repo can reproduce exactly which descriptors were admitted and in what order.

**Enforcement:** `RUNE-REQ-046`, `RUNE-REQ-060`, `RUNE-REQ-061`,
`RUNE-REQ-062`, registry tests, discovery tests, and retained fixtures enforce
explicit collection boundaries.

**Evidence:** `README.md`, `docs/architecture/crate-owned-registry-workflow.md`,
`docs/architecture/deterministic-discovery-interface.md`, and
`cargo test -p rune-adopter`.

## RUNE-I-04: Profiles And Adapters Stay Outside The Neutral Core

**Status:** VERIFIED

**Claim:** External profiles and downstream adapters are reviewed mappings over
validated RUNE evidence or profile output, not extensions of the core descriptor
language.

**Why it matters:** Product-specific adapters are useful only when they do not
silently redefine what a RUNE descriptor means.

**Enforcement:** `SPEC-RUNE-005`, `SPEC-RUNE-006`, profile catalog tests,
adapter tests, compatibility checks, and retained review/documentation packet
fixtures preserve the boundary.

**Evidence:** `PRODUCT_PLAN.md`,
`docs/architecture/external-profile-interface.md`,
`docs/architecture/downstream-adapter-interface.md`, and
`cargo test --workspace`.

## RUNE-I-05: Agent And Runtime Surfaces Stay Read-First Until Approved

**Status:** VERIFIED

**Claim:** Agent protocol, state graph, evidence packet, compatibility, and
runtime-host surfaces validate retained fixtures without mutation, live
inspection, source scraping, private payload capture, automatic migration, or
policy enforcement.

**Why it matters:** AI-facing contract access must not become an accidental
runtime authority or private-data exposure path.

**Enforcement:** `RUNE-REQ-081` through `RUNE-REQ-084`, `RUNE-REQ-090` through
`RUNE-REQ-093`, Mission 2.0 review, and CLI tests block mutation, restricted
data, runtime host behavior, and migration.

**Evidence:** `docs/vtrace/TRACE.md`, `docs/vtrace/REVIEW.md`,
`docs/architecture/agent-protocol-interface.md`, and
`docs/architecture/runtime-host-design.md`.

# RUNE Mission

## Mission need

Rust projects need a durable, extensible way to expose their domain types,
states, events, commands, artifacts, invariants, sources, and evidence as
machine-readable contracts.

RUNE provides a Rust-native notation and derivation layer for generating those
contracts from idiomatic Rust code, so downstream tools can emit schemas,
IDL-style definitions, trace/event contracts, state maps, documentation, review
packets, and AI-facing metadata without scraping source text or encoding
product-specific assumptions in the core model.

## Stakeholder needs

| ID | Need |
|---|---|
| NEED-001 | Rust maintainers need contract metadata that is generated from code and reviewed like code. |
| NEED-002 | AI tooling needs stable, versioned descriptors for entity types, state, events, commands, artifacts, and evidence. |
| NEED-003 | Downstream platforms need extension points for their own schema and manifest formats without coupling RUNE core to those platforms. |
| NEED-004 | Assurance reviewers need trace links from generated contracts back to requirements, design decisions, implementation surfaces, and verification evidence. |
| NEED-005 | Contract and platform authors need RUNE to act like a Rust-era IDL/MIDL/WSDL/CSDL layer: neutral at the core, extensible through profiles, and stable enough for generated artifacts. |

## Intended outcome

RUNE should let a Rust crate declare its contract surface once, in code, and then
generate multiple downstream artifacts from a neutral, versioned source of truth.

## Mission 2.0

RUNE's next mission is a managed native semantic runtime layer for Rust: keep
Rust native, explicit, and product-neutral, but add the managed-runtime
affordances AI and tooling need: semantic registries, state graphs, evidence
runtime packets, agent-safe query protocols, compatibility negotiation, and
capability/sensitivity metadata.

Current Mission 2.0 implementation is DCR-gated: DCR-RUNE-003 implements retained
semantic registry evidence, and DCR-RUNE-004 implements hardened retained state
graph validation. Runtime host behavior, live state inspection, mutation/replay,
Cargo traversal, source scraping, plugin discovery, automatic migration, and
policy enforcement remain blocked.

See `MISSION_2_0.md` for the controlled mission upgrade and DCR status.

# Descriptors as contracts

RUNE treats selected Rust items as explicit contracts for AI and review tooling.
The Rust type remains the implementation surface; the RUNE descriptor is the
stable evidence surface.

## Contract shape

A v1 descriptor records:

| Field | Why it matters |
|---|---|
| `id` | Durable identity for tools and reviewers. |
| `version` | Compatibility boundary for generated artifacts. |
| `kind` | Neutral role such as entity, command, event, state, artifact, source, or evidence. |
| `rust_type` | The Rust item that produced the descriptor. |
| `fields` | The inspectable data shape. |
| `trace_links` | Optional links to requirements, design, verification, or evidence. |

Descriptors are neutral. They do not name a specific agent runtime, context
substrate, workflow engine, or product-specific map.

## Why not source-only review?

Source-only review can show a struct and its fields, but it usually leaves these
questions implicit:

- Which identifier should downstream tools use?
- Which version is durable?
- Which profile can safely emit an artifact?
- Which requirements or evidence justify the contract?
- Which downstream vocabulary is adapter-owned rather than core-owned?

RUNE makes those answers explicit and retains them as JSON evidence.

## Authoring rule

Use `#[derive(RuneContract)]` and `#[rune(...)]` only for stable contract facts.
Keep convenience prose, product routing hints, and consumer-specific terms in
profile or adapter surfaces.

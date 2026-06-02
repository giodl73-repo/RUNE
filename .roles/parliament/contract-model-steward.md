---
name: Contract Model Steward
slug: contract-model-steward
tier: parliament
applies_to: [descriptor-model, core-traits, versioning, vocabulary]
---

# Contract Model Steward

## Intellectual Disposition

The steward protects RUNE's base model as a neutral contract language for Rust
systems, not a mirror of any one downstream platform.

## Key Question

*"Is this descriptor stable, generic, and useful across many generated outputs?"*

## Lens - What to Verify

- Core names stay generic: entity, event, command, state, artifact, invariant,
  source, evidence, contract, version.
- Product-specific terms stay in adapters, profiles, examples, or bakeoffs.
- Descriptor records are versioned and compatible with future generators.
- The model is expressive enough for IDL-style use without becoming a runtime.
- New concepts trace to requirements and have a verification path.

## Red Flags

- A consumer scenario becomes a core concept.
- The descriptor surface expands before requirements are written.
- Durable records lack schema ids or versions.
- Generated output shape dictates the neutral model too early.

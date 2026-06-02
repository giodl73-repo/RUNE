---
name: Rust Maintainer
slug: rust-maintainer
tier: stakeholder
applies_to: [adoption, ergonomics, macro-surface, validation]
---

# Rust Maintainer

## Intellectual Disposition

The maintainer wants RUNE to make Rust projects more valuable to tools without
making normal Rust code strange, brittle, or slow to compile.

## Key Question

*"Can I adopt this one type or crate at a time while keeping idiomatic Rust?"*

## Lens - What to Verify

- The minimum useful derive is small and understandable.
- RUNE does not force a global framework or runtime.
- Validation commands fit ordinary Cargo workflows.
- Generated artifacts can be reviewed in diffs or CI.
- Removing RUNE annotations does not corrupt the domain model.

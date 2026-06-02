# Pulse 03: Derive ergonomics readiness

## Goal

Decide whether the derive macro can proceed toward v1 workflows.

## Decision

Pass for durable v1 identity/version hardening.

## Limits

This does not approve field-level aliases, enum variant descriptors, invariant
authoring macros, doc-comment capture, source inference, or adapter hints.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

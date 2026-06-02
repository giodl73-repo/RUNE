# RUNE Descriptor Model

## Engineering decision supported

Use a small neutral descriptor model as the first durable RUNE interface, then
let downstream generators transform descriptors into their own schemas,
manifests, maps, traces, receipts, or documents.

The descriptor model is the stable middle. It should be expressive enough to
support IDL-style generation, AI-readable metadata, traceability, and review, but
it must not become a runtime, workflow engine, context substrate, schema format,
or product-specific manifest.

## Core concepts

| Concept | Meaning |
|---|---|
| Entity | Durable domain object or value type. |
| Event | Fact that something happened. |
| Command | Requested operation or intent. |
| State | Lifecycle or automaton state. |
| Artifact | Generated or consumed durable output. |
| Invariant | Condition that must remain true. |
| Source | Pointer or reference to supporting input. |
| Evidence | Verification or validation record. |
| Contract | Versioned descriptor for one of the above. |
| Version | Compatibility boundary for a descriptor, profile, or generated artifact. |

## Minimum descriptor shape

A durable RUNE descriptor should carry enough neutral structure for many
generators to map it safely:

| Field | Purpose |
|---|---|
| `id` | Stable descriptor identity. |
| `version` | Compatibility boundary for the descriptor. |
| `kind` | One foundation concept: entity, event, command, state, artifact, source, evidence, or other reviewed extension. |
| `rust_type` | Rust type or item the descriptor was derived from. |
| `fields` | Named field descriptors when applicable. |
| `invariants` | Conditions that generated artifacts or consumers must preserve when supplied. |
| `trace_links` | Optional links to need, requirement, design, implementation, verification, validation, and evidence records. |
| `extensions` | Namespaced data reserved for reviewed profiles, not core vocabulary expansion. |

Stage 4 owns the exact Rust trait, struct, macro, and CLI interface for this
shape. This document only fixes the concept boundary.

## Vocabulary rules

1. Core descriptor names stay generic and domain-neutral.
2. Product-specific terms belong in profiles, adapters, examples, or bakeoffs.
3. A generated output format must not become the hidden source of truth.
4. Missing or unsupported concepts must remain visible to generators and
   diagnostics; they must not be silently dropped.
5. New core concepts require a requirement, trace row, and role review before
   adoption.

## Profile boundary

Profiles transform RUNE descriptors into downstream artifacts such as schemas,
IDL-style definitions, trace/event contracts, state maps, documentation, review
packets, and AI-facing metadata.

Profiles may:

- rename neutral concepts into target-system vocabulary,
- add target-specific metadata under namespaced extensions,
- report unsupported concepts through diagnostics,
- define compatibility mappings for generated artifacts.

Profiles may not:

- require core RUNE descriptors to use target-system vocabulary,
- hide unsupported descriptor concepts,
- make generated artifacts durable without descriptor identity and version data,
- bypass VTRACE stage gates for stable interfaces.

## Non-goal

The base descriptor model does not define product-specific state machines,
context algebra, agent maps, or workflow runtimes. Those belong in adapters.

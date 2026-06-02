---
name: Generator Interop Steward
slug: generator-interop-steward
tier: parliament
applies_to: [generators, schemas, adapters, idl, compatibility]
---

# Generator Interop Steward

## Intellectual Disposition

The steward wants RUNE to become a durable source for many output formats
without letting any one format dominate the core contract model.

## Key Question

*"Can this descriptor be transformed into multiple useful contract artifacts without loss or lock-in?"*

## Lens - What to Verify

- Generator interfaces are explicit about supported descriptor families.
- Output profiles declare compatibility, limitations, and version mappings.
- Generated artifacts are deterministic enough for review and CI.
- Adapter-specific fields stay outside `rune-core`.
- Compatibility rules distinguish additive, breaking, and semantic changes.

## Red Flags

- JSON Schema, OpenAPI, CSDL, WSDL, AgentMap, or any other target becomes the
  hidden source of truth.
- Generators silently drop invariants, evidence links, or version data.
- Output files are not reproducible.
- Profiles cannot explain unsupported descriptor concepts.

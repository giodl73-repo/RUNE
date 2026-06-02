---
name: Platform Adapter Author
slug: platform-adapter-author
tier: stakeholder
applies_to: [adapters, profiles, generators, downstream-systems]
---

# Platform Adapter Author

## Intellectual Disposition

The adapter author needs RUNE core to be stable and neutral so a downstream
system can map descriptors into its own vocabulary.

## Key Question

*"Can I build a profile-specific generator without patching RUNE core?"*

## Lens - What to Verify

- Descriptor records expose enough structure for mapping.
- Extension points are documented.
- Unsupported mappings produce diagnostics, not silent omissions.
- Adapter-specific naming belongs in the adapter.
- Version and compatibility data survive the mapping.

# Downstream adapter interface

## Purpose

Downstream adapters convert validated RUNE evidence into consumer-specific input
documents. They are separate from neutral descriptors and external profiles:
descriptors define what the Rust contracts are, profiles define generated output
formats, and adapters map approved outputs into a named consumer context.

## Boundary rule

Adapter vocabulary must not be added to `rune-core`, derive attributes, neutral
descriptor fixtures, or the base profile catalog unless it is profile metadata.
Consumer names belong in adapter modules, adapter fixtures, and adapter docs.

## Approved inputs

The first adapter implementations may consume:

- `DescriptorDocument`,
- `DescriptorCollectionDocument`,
- `CollectionEvidenceBundleDocument`,
- `DocumentationPacketDocument`.

Adapters must validate or receive already validated inputs. They must not infer
missing descriptor identities, rewrite descriptor versions, scan Rust source,
traverse Cargo metadata, or execute crate hooks.

## Required adapter metadata

Every adapter must declare:

- stable `adapter_id`,
- `adapter_version`,
- accepted input artifact kinds,
- supported descriptor kinds,
- supported concept families,
- output artifact kind,
- fail-closed diagnostic family,
- retained input and output fixtures.

## Output contract

Adapter output must be deterministic JSON and include adapter metadata plus the
source RUNE identity or collection identity that produced it. Adapter output may
use consumer vocabulary only inside the adapter artifact.

## Diagnostics

Adapter implementations must fail closed for:

- unsupported adapter id,
- unsupported input artifact kind,
- unsupported descriptor or collection version,
- unsupported descriptor kind,
- unsupported concept family,
- malformed retained input,
- serialization errors.

## First implementation

The first adapter is `rune.review_packet_json`. It consumes validated descriptor
collection documents and emits deterministic review packets for humans and AI
reviewers. Context map and transition input adapters remain future candidates.

## Non-goals

- No BAKER, LATTICE, AgentMaps, or other product-specific terms in `rune-core`.
- No product-specific adapter implementation.
- No source scraping or Cargo traversal.
- No executable hooks.
- No automatic publication to downstream systems.

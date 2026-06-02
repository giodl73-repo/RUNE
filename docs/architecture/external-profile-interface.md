# External profile interface

## Purpose

External profiles let RUNE emit non-core artifacts for specific consumer shapes
without changing the neutral descriptor model. They are mappings from validated
RUNE descriptor or collection documents into a named output contract.

## Boundary rule

External profile vocabulary must not be added to `rune-core` descriptor fields,
derive attributes, or neutral collection records. Product-specific names belong
in the external profile implementation, retained artifacts, and adapter-facing
docs.

## Required profile metadata

Each external profile must declare:

- stable `profile_id`,
- `profile_version`,
- accepted descriptor and collection versions,
- supported descriptor kinds,
- supported concept families,
- output artifact kind,
- unsupported-concept policy,
- diagnostics for every fail-closed rejection.

## Input contract

External profiles consume only validated `DescriptorDocument` or
`DescriptorCollectionDocument` values. They must not:

- inspect Rust source directly,
- traverse Cargo metadata,
- execute crate hooks,
- infer missing descriptor identity or version,
- mutate retained evidence during normal validation.

## Output contract

External profile output must be deterministic JSON with profile metadata,
source descriptor or collection identity, and generated payload. The output may
use profile-specific vocabulary, but only below the profile artifact boundary.

## Diagnostics

The first external profile implementation must reserve profile-specific
diagnostics for:

- unsupported profile id,
- unsupported descriptor or collection version,
- unsupported descriptor kind,
- unsupported invariant representation,
- unsupported trace link representation,
- unsupported extension representation,
- serialization errors.

## First implementation

The first external profile is `rune.documentation_packet_json`. It summarizes
validated descriptors or collections for humans and AI reviewers while avoiding
product-specific adapter terms.

## Non-goals

- No downstream adapter implementation.
- No product-specific core vocabulary.
- No source scraping or Cargo traversal.
- No schema language lock-in for all future profiles.
- No automatic publishing of generated artifacts.

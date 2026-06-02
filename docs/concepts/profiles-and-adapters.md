# Profiles and adapters

Profiles and adapters let RUNE become useful outside its neutral descriptor
model without making `rune-core` product-specific.

## Profiles

A profile maps validated RUNE descriptors into a named output shape. The first
external profile is:

| Profile | Output |
|---|---|
| `rune.documentation_packet_json` | Documentation packets for AI and reviewer consumption. |

Profiles may introduce profile-owned vocabulary in their outputs, but they do
not rewrite the neutral descriptor vocabulary.

## Adapters

An adapter maps validated RUNE evidence into a downstream artifact. The first
adapter is:

| Adapter | Output |
|---|---|
| `rune.review_packet_json` | Deterministic review packets over descriptor collections. |

Adapters live outside `rune-core`; v1 uses the `rune-adapters` crate.

## Boundary test

If a term only makes sense to one downstream product, it belongs in an adapter or
profile output. If it describes a general Rust contract fact, it may belong in a
neutral descriptor after requirements and interface review.

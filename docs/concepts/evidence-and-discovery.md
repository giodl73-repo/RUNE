# Evidence and discovery

RUNE v1 is evidence-first. Tools consume retained descriptor documents and
descriptor collections, not ad hoc source scraping.

## Evidence flow

```text
Annotated Rust type
  -> explicit crate-owned registry
  -> descriptor collection document
  -> checks, inventory, generated artifacts, profiles, adapters
```

Retained evidence makes reviews repeatable. A reviewer can inspect the JSON
without expanding macros, running arbitrary hooks, or traversing a whole Cargo
workspace.

## Explicit registry

An adopter crate owns a `RUNE_CONTRACTS` registry. The registry gives RUNE:

- deterministic ordering,
- an explicit adoption boundary,
- fail-closed duplicate identity diagnostics,
- a clean place to decide which Rust items are contract surfaces.

## Manifest discovery

`rune discover --manifest <path>` merges retained descriptor collection fixtures
in manifest order. This is intentionally narrower than crate scanning.

Manifest discovery is useful when a repo or portfolio wants one reviewed
collection assembled from multiple retained evidence sources.

## Non-goals

RUNE v1 does not:

- scan arbitrary Rust source,
- traverse Cargo metadata for discovery,
- execute registry hooks,
- publish automatically to downstream systems.

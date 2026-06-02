# Wave: Adoption docs package

## Goal

Turn the single adopter guide into a Craftworks-style documentation package with
indexed concepts, how-tos, tutorials, examples, traces, and corpus governance.

## Engineering decision supported

RUNE v1 is not only release-ready for maintainers; it is learnable by adopters
through task-oriented and progressive documentation without expanding the code
surface.

## Scope

This wave may add documentation indexes, concept guides, procedural how-tos,
tutorial walkthroughs, example references, trace walkthroughs, corpus governance,
README/product-plan links, and VTRACE records. It must not add new CLI behavior,
arbitrary source scraping, Cargo traversal, executable hooks, automatic
publishing, or product-specific vocabulary in `rune-core`.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Documentation index and corpus | complete |
| 02 | Concept guides | complete |
| 03 | How-to guides | complete |
| 04 | Tutorials and examples | complete |
| 05 | Trace walkthroughs and VTRACE closure | complete |

## Decision

Wave 33 passes as a documentation-only adoption package for RUNE v1.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
git diff --check
```

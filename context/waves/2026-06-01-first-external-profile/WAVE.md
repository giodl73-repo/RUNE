# Wave: First external profile

## Goal

Implement the first external profile without changing neutral core descriptor
vocabulary.

## Engineering decision supported

`rune.documentation_packet_json` is a low-risk first external profile because it
summarizes validated descriptors and collections for humans and AI reviewers
without becoming a downstream product adapter.

## Scope

This wave may add documentation packet profile metadata, generated packet
documents, CLI generation support, retained fixtures, tests, and VTRACE records.
It must not add adapters, source scraping, Cargo metadata traversal, executable
hooks, or product-specific derive/core fields.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Documentation packet model | complete |
| 02 | Profile generation fixtures | complete |
| 03 | External profile closure | complete |

## Decision

Wave 26 passes for the first external profile implementation. Downstream adapter
interface-control remains the next gated v1 area.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

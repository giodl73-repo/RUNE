# Wave: Known contract evidence collection

## Goal

Extend deterministic descriptor evidence from one annotated contract to multiple
explicitly registered known contracts, without arbitrary crate scanning or
external profile generation.

## Engineering decision supported

RUNE can approach repo-scale adoption only after it proves that multiple known
annotated contracts can be collected into stable descriptor evidence through a
controlled registry or test-oriented evidence path.

## Scope

This wave may add explicitly registered test contracts, deterministic evidence
collection helpers, and retained descriptor evidence checks. It must not scan
arbitrary crates, infer contracts from source files, or add JSON Schema, CSDL,
OpenAPI, AgentMap, BAKER, LATTICE, or product adapters.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Known-contract collection gate | complete |
| 02 | Second annotated contract fixture | complete |
| 03 | Multi-contract evidence check | complete |
| 04 | Collection readiness decision | complete |

## Decision

Wave 6 passes for controlled known-contract evidence collection. RUNE can now
derive and retain evidence for multiple explicitly registered annotated
contracts in a stable test-owned collection.

The wave does not approve arbitrary crate scanning, source inference, external
profiles, or product adapters. The next expansion must define a reviewed
explicit registry or discovery interface before implementation.

## Validation

```powershell
git diff --check
cargo test --workspace
```

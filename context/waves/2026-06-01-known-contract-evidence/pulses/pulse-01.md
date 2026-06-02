# Pulse 01: Known-contract collection gate

## Goal

Define the known-contract collection boundary before adding more annotated
contract scenarios.

## Engineering decision supported

RUNE may collect deterministic evidence for multiple explicitly registered
contracts, but must still avoid arbitrary crate scanning and external profile
generation.

## Decision

Known-contract collection is limited to annotated contracts named in tests or a
future explicit registry. Each contract must have:

- derived descriptor assertions,
- retained descriptor evidence,
- compatibility check evidence,
- neutral generated artifact evidence,
- deterministic regeneration or comparison from `DescriptorDocument`.

## Result

Complete. The first additional scenario may be a command contract.

## Validation

```powershell
git diff --check
cargo test --workspace
```

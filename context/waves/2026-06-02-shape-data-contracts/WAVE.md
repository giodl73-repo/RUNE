# Wave 35: Shape data contracts

## Goal

Prove that RUNE can model metadata-driven data contracts and use games repo
review to steer adoption beyond trait-only extraction.

## Scope

- Add derive-authored invariants and namespaced extensions.
- Add a small shape calculator example with retained descriptor, documentation,
  and data-contract profile evidence.
- Add `rune.data_contract_json` as a profile-owned output over neutral
  descriptors.
- Record games contract candidate findings as scenario guidance.

## Non-goals

- Do not add game-specific vocabulary to `rune-core`.
- Do not edit games repos as part of this wave.
- Do not add field-level metadata or trait/function descriptors yet.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
git diff --check
```

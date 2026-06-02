# RUNE Explicit Registry Interface

## Engineering decision supported

Define the first reviewed collection interface after Wave 6 proved that multiple
known annotated contracts can be retained as stable evidence.

## Problem

Wave 6 used a test-owned known-contract collection to prove deterministic
multi-contract evidence. The collection was intentionally explicit, but the
registry boundary was not yet a core interface. Broadening from one-off test
arrays requires a neutral interface with ordering and diagnostics rules before
any crate discovery or product adapter work is allowed.

## Approved interface

The first registry interface is explicit and code-owned:

```rust
pub struct ContractRegistration {
    pub name: &'static str,
    pub descriptor: fn() -> ContractDescriptor,
}

pub fn collect_known_contract_documents(
    registrations: &[ContractRegistration],
    duplicate_id_code: &'static str,
) -> Result<Vec<DescriptorDocument>, String>;
```

## Rules

1. Registrations are ordered by the caller-provided slice.
2. Collection must preserve that order in output evidence.
3. Each registration points to a known `RuneContract::descriptor` function.
4. Collection must fail closed on duplicate descriptor ids.
5. Collection emits neutral `DescriptorDocument` values only.
6. The interface must not scan source files, enumerate crates, invoke Cargo
   metadata, load plugins, or call downstream systems.

## Diagnostics

The first diagnostic family is reserved for registry collection:

| Code | Meaning |
|---|---|
| `RUNE-REG-001` | Duplicate descriptor id in an explicit registry. |

Future registry diagnostics may cover unsupported descriptor versions, missing
required evidence, unstable ordering, or invalid registry entries after those
surfaces are specified.

## Non-goals

- No arbitrary crate discovery.
- No proc-macro inventory or linker-section registry.
- No CLI collection command.
- No external profile generation.
- No product-specific adapter vocabulary.

## Validation command

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

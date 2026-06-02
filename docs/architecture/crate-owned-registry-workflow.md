# RUNE Crate-Owned Registry Workflow

## Engineering decision supported

Define how an adopter crate owns an explicit RUNE contract registry before
discovery, external profiles, or downstream adapters are implemented.

## Workflow

An adopter crate should:

1. Annotate its Rust contracts with `#[derive(RuneContract)]`.
2. Expose a deterministic `RUNE_CONTRACTS: &[ContractRegistration]` slice.
3. Preserve caller-provided registry order as evidence order.
4. Build a `DescriptorCollectionDocument` with a stable collection id and version.
5. Retain generated collection evidence under test or release artifacts.
6. Fail closed on duplicate descriptor ids.

## Example

```rust
pub const RUNE_CONTRACTS: &[ContractRegistration] = &[
    ContractRegistration {
        name: "Account",
        descriptor: Account::descriptor,
    },
    ContractRegistration {
        name: "OpenAccount",
        descriptor: OpenAccount::descriptor,
    },
];

pub fn descriptor_collection() -> Result<DescriptorCollectionDocument, String> {
    DescriptorCollectionDocument::from_registrations(
        "example.adopter_contracts",
        "v0",
        RUNE_CONTRACTS,
        "RUNE-REG-001",
    )
}
```

## Rules

- The adopter crate owns the registry and collection identity.
- Registry order is source order in the explicit slice.
- Duplicate descriptor ids fail closed with `RUNE-REG-001`.
- The workflow emits neutral descriptor collection evidence only.
- The workflow must not scan crates, infer source files, call Cargo metadata, load
  plugins, or invoke downstream adapters.

## Evidence

The workspace includes `examples/rune-adopter`, which demonstrates:

- annotated entity and command contracts,
- a crate-owned `RUNE_CONTRACTS` registry,
- retained collection fixture evidence,
- duplicate descriptor id failure behavior.

## Validation command

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

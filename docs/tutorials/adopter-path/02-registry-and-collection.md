# Lesson 2: Registry and collection

## What you will learn

- How to expose a crate-owned contract registry.
- How RUNE keeps collection order deterministic.
- Why duplicate descriptor ids fail closed.

## Step 1: Register known contracts

```rust
pub const RUNE_CONTRACTS: &[ContractRegistration] = &[
    ContractRegistration {
        name: "Customer",
        descriptor: Customer::descriptor,
    },
];
```

## Step 2: Build collection evidence

Use the adopter test pattern in `examples\rune-adopter\tests` to build a
`DescriptorCollectionDocument` from the registry and compare it to a retained
fixture.

## Step 3: Retain the fixture

```text
examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

## What you learned

The adopter owns the registry boundary. RUNE does not decide that every Rust type
is a contract.

## Next

[Lesson 3: Evidence commands](03-evidence-commands.md)

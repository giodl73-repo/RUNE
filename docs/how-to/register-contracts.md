# Register contracts

Use an explicit registry so an adopter crate controls which Rust items become
RUNE evidence.

## Create a registry

```rust
use rune_core::{ContractRegistration, RuneContract};

pub const RUNE_CONTRACTS: &[ContractRegistration] = &[
    ContractRegistration {
        name: "Customer",
        descriptor: Customer::descriptor,
    },
    ContractRegistration {
        name: "CreateCustomer",
        descriptor: CreateCustomer::descriptor,
    },
];
```

## Build a collection

Use `collect_known_contract_documents` from `rune-core` in tests or adopter
tooling to produce a deterministic `DescriptorCollectionDocument`.

The registry order is preserved. Duplicate descriptor ids fail closed.

## Retain the collection

Retain the generated collection under the adopter crate, for example:

```text
tests\fixtures\adopter_contract_collection.json
```

Normal tests should compare against the retained fixture. Regeneration should be
an explicit maintainer action, not a side effect of ordinary validation.

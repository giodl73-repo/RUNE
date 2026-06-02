# Derive a first contract

Use the derive macro when a Rust type should become a durable, AI-readable
contract.

## Add the derive

```rust
use rune_derive::RuneContract as DeriveRuneContract;

#[derive(DeriveRuneContract)]
#[rune(
    id = "example.customer",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-034"
)]
pub struct Customer {
    pub id: String,
    pub email: String,
}
```

## Required metadata

| Attribute | Required | Purpose |
|---|---:|---|
| `id` | yes | Stable descriptor identity. |
| `version` | yes | Durable compatibility boundary. |
| `kind` | yes | Neutral descriptor kind. |
| `requirement` | no | Trace link to a requirement or design record. |

## Check it

Run the workspace tests:

```powershell
cargo test --workspace
```

The derive should fail closed if required identity or version metadata is
missing.

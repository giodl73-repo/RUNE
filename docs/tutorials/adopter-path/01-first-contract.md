# Lesson 1: First contract

## What you will learn

- How to mark a Rust type as a RUNE contract.
- Which metadata is required for durable evidence.
- Why the metadata stays product-neutral.

## Step 1: Choose a stable contract type

Start with a type that downstream tools should understand, such as an entity,
command, event, state, artifact, source, or evidence record.

## Step 2: Add RUNE metadata

```rust
#[derive(DeriveRuneContract)]
#[rune(id = "example.customer", version = "v0", kind = "entity")]
pub struct Customer {
    pub id: String,
    pub email: String,
}
```

## Step 3: Run tests

```powershell
cargo test --workspace
```

## What you learned

RUNE starts with explicit, stable metadata. It does not infer product-specific
meaning from source names.

## Next

[Lesson 2: Registry and collection](02-registry-and-collection.md)

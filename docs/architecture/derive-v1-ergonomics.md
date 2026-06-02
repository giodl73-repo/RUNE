# RUNE Derive V1 Ergonomics

## Engineering decision supported

Make the derive macro safe enough for durable v1 evidence while preserving the
neutral descriptor model and deferring richer authoring conveniences until they
have reviewed semantics.

## V1 derive surface

The v1 durable derive surface is:

```rust
#[derive(RuneContract)]
#[rune(
    id = "example.customer",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-034",
    invariant(id = "customer.email.present", text = "email is not empty"),
    extension(namespace = "example.customer", name = "owner", value = "crm")
)]
struct Customer {
    #[rune_field(required = true, sensitivity = "internal", stability = "stable")]
    id: String,
    #[rune_field(required = true, sensitivity = "private", example = "a@example.com")]
    email: String,
}
```

| Type attribute | V1 status | Meaning |
|---|---|---|
| `id` | required | Stable descriptor identity. |
| `version` | required | Descriptor compatibility boundary. |
| `kind` | optional, default `entity` | Neutral contract kind. |
| `requirement` | optional, repeatable | Adds a neutral requirement trace link. |
| `invariant(id = "...", text = "...")` | optional, repeatable | Adds declared contract rules as metadata. |
| `extension(namespace = "...", name = "...", value = "...")` | optional, repeatable | Adds adopter-owned metadata without changing neutral core vocabulary. |

| Field attribute | V1 status | Meaning |
|---|---|---|
| `required = true/false` | optional | Declares whether consumers may omit the field. |
| `unit = "..."` | optional | Declares a measurement or domain unit. |
| `min = "..."` / `max = "..."` | optional | Declares string-preserved bounds without imposing numeric coercion semantics. |
| `sensitivity = "..."` | optional | Declares review/privacy handling metadata. |
| `example = "..."` | optional | Provides a copyable example value. |
| `stability = "..."` | optional | Declares compatibility expectation for the field. |
| `alias = "..."` | optional, repeatable | Adds alternate consumer-facing field names in author order. |

Missing `id` or `version` is a compile-time error because generated artifacts and
retained evidence are not durable without explicit identity and compatibility
boundaries.

## Supported input shapes

| Rust input | V1 behavior |
|---|---|
| Named-field structs | Field names, Rust type strings, and explicit `#[rune_field(...)]` metadata are emitted. |
| Tuple structs | Accepted, but no fields are emitted yet. |
| Unit structs | Accepted, but no fields are emitted. |
| Enums | Accepted, but no variant metadata is emitted yet. |
| Unions | Accepted by the macro boundary, but no fields are emitted. |

## Deferred ergonomics

The following are deliberately deferred until separate requirements and retained
evidence exist:

- enum variant descriptors and state-transition semantics,
- source-link inference,
- generated documentation capture from Rust doc comments,
- product-specific adapter hints.

These features must not be silently inferred into durable output. Each needs an
interface-control update, diagnostics, retained fixtures, and VTRACE review.

## Diagnostics

The derive macro must fail closed for:

| Diagnostic | Meaning |
|---|---|
| `missing required rune attribute: id` | Durable descriptor identity was omitted. |
| `missing required rune attribute: version` | Durable descriptor compatibility boundary was omitted. |
| `unsupported rune attribute` | The author supplied an unreviewed `#[rune(...)]` key or nested metadata key. |
| `unsupported rune field attribute` | The author supplied an unreviewed `#[rune_field(...)]` key. |

## Validation command

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

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
    requirement = "RUNE-REQ-034"
)]
struct Customer {
    id: String,
    email: String,
}
```

| Attribute | V1 status | Meaning |
|---|---|---|
| `id` | required | Stable descriptor identity. |
| `version` | required | Descriptor compatibility boundary. |
| `kind` | optional, default `entity` | Neutral contract kind. |
| `requirement` | optional, repeatable | Adds a neutral requirement trace link. |

Missing `id` or `version` is a compile-time error because generated artifacts and
retained evidence are not durable without explicit identity and compatibility
boundaries.

## Supported input shapes

| Rust input | V1 behavior |
|---|---|
| Named-field structs | Field names and Rust type strings are emitted. |
| Tuple structs | Accepted, but no fields are emitted yet. |
| Unit structs | Accepted, but no fields are emitted. |
| Enums | Accepted, but no variant metadata is emitted yet. |
| Unions | Accepted by the macro boundary, but no fields are emitted. |

## Deferred ergonomics

The following are deliberately deferred until separate requirements and retained
evidence exist:

- field-level aliases, docs, examples, sensitivity, optionality, or stability
  metadata,
- enum variant descriptors and state-transition semantics,
- invariant authoring macros,
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
| `unsupported rune attribute` | The author supplied an unreviewed `#[rune(...)]` key. |

## Validation command

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

# RUNE Interface and Control Spec

## Engineering decision supported

Define the first contract-stability boundary for RUNE before expanding
implementation. Stage 5 may implement only the surfaces named here; anything
outside this file remains scaffold, experiment, or later-stage work.

## Controlled interfaces

| Interface | Owner crate or surface | Stage 4 status |
|---|---|---|
| Neutral descriptor records | `rune-core` | approved for Stage 5 slice |
| Descriptor trait | `rune-core` | approved for Stage 5 slice |
| Derive macro | `rune-derive` | approved for Stage 5 slice |
| CLI status command | `rune-cli` | approved for Stage 5 slice |
| CLI inspect/generate commands | `rune-cli` | deferred |
| Generator profile API | future crate or module | deferred |
| Diagnostics envelope | future core/generator surface | specified conceptually, implementation deferred |

## Descriptor records

The first stable descriptor slice uses neutral records only. Exact Rust naming
may evolve during Stage 5, but the public shape must preserve these fields:

| Record | Required fields | Purpose |
|---|---|---|
| `ContractDescriptor` | `id`, `version`, `kind`, `rust_type`, `fields`, `invariants`, `trace_links`, `extensions` | Durable descriptor for a Rust item. |
| `FieldDescriptor` | `name`, `rust_type` | Field-level metadata for named Rust fields. |
| `InvariantDescriptor` | `id`, `text` | Human-readable condition that generated artifacts or consumers must preserve. |
| `TraceLink` | `relation`, `target` | Link to need, requirement, design, implementation, verification, validation, or evidence. |
| `ExtensionDescriptor` | `namespace`, `name`, `value` | Namespaced profile data that does not expand core vocabulary. |

## Descriptor trait

The first trait surface is intentionally small:

```rust
pub trait RuneContract {
    fn descriptor() -> ContractDescriptor;
}
```

Rules:

1. The trait returns descriptor metadata only.
2. The trait must not execute runtime behavior, invoke tools, read files, or call
   downstream systems.
3. The descriptor must include stable identity and version before generated
   artifacts are treated as durable.

## Contract kind vocabulary

The foundation `ContractKind` vocabulary is:

```text
entity, event, command, state, artifact, source, evidence, other
```

`invariant`, `contract`, and `version` are descriptor concepts, not first-slice
`ContractKind` variants. They are represented by descriptor fields and
compatibility rules.

The `other` variant is allowed only as an escape hatch for reviewed extensions.
Generated profiles must report unsupported `other` values explicitly.

## Derive macro interface

The first macro is:

```rust
#[derive(RuneContract)]
#[rune(
    id = "example.customer",
    version = "v0",
    kind = "entity",
    requirement = "RUNE-REQ-034"
)]
pub struct Customer {
    pub id: String,
}
```

Supported attributes:

| Attribute | Required | Meaning |
|---|---:|---|
| `id` | required for durable output | Stable descriptor identity. |
| `version` | required for durable output | Descriptor compatibility boundary. |
| `kind` | optional, default `entity` | Foundation contract kind. |
| `requirement` | optional, repeatable | Adds a neutral trace link with relation `requirement`. |

V1 derive verification treats missing `id` or `version` as compile-time errors.
Durable generated artifacts require explicit `id` and `version`.

Macro rules:

1. The macro generates only descriptor metadata.
2. The macro must not encode downstream platform vocabulary.
3. Missing or unsupported authoring input should become a compile error or a
   diagnostic in the generated/inspection path; it must not silently produce a
   success-shaped durable artifact.
4. Generated behavior must be inspectable through tests or CLI output before the
   macro surface is called stable.

## CLI contract

The only Stage 5-approved CLI command is:

```powershell
cargo run -p rune-cli -- status
```

Expected responsibilities:

- report available foundation surfaces,
- avoid claiming generator or profile support before those interfaces exist,
- exit non-zero for unknown commands.

Deferred commands:

```text
rune inspect
rune generate
rune check
rune profile list
```

Those commands require generator and diagnostics specs before implementation.

## Generator profile boundary

Generators consume `ContractDescriptor` records and produce downstream artifacts.
The profile interface is deferred, but the boundary is controlled now:

1. Profiles map neutral descriptors into target vocabulary.
2. Profiles declare supported descriptor kinds and fields.
3. Profiles preserve descriptor `id` and `version` in durable outputs.
4. Profiles report unsupported concepts through diagnostics.
5. Profiles do not add core vocabulary without a new requirement and review.

## Diagnostics concept

RUNE diagnostics should eventually include:

| Field | Purpose |
|---|---|
| `code` | Stable diagnostic code. |
| `severity` | `error`, `warning`, or `info`. |
| `message` | Human-readable finding. |
| `target` | Descriptor, macro input, profile, or generated artifact. |
| `requirement` | Optional RUNE requirement ID. |
| `suggested_fix` | Actionable repair when known. |

Stage 5 does not need a diagnostics implementation beyond CLI error exits.
Stage 6 must add checks before macro or generator surfaces become stable.

## Stage 5 implementation allowance

Stage 5 may update code only to align the scaffold with this spec:

- rename descriptor `schema_id` to `id`,
- add invariant and extension descriptor records,
- align derive attributes with `id`, `version`, and `kind`,
- allow neutral requirement trace links in derived descriptors,
- keep CLI `status` honest and bounded,
- add focused tests for the approved slice.

Stage 5 must not implement profile generators, inspect/generate CLI commands, or
consumer-specific adapters.

## Validation command

```powershell
git diff --check
```

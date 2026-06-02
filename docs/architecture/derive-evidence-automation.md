# RUNE Derive Evidence Automation

## Engineering decision supported

Define the allowed automation path for producing descriptor evidence from
annotated Rust code before adding broad repository discovery, external profiles,
or product-specific adapters.

## Problem

Wave 4 showed that RUNE evidence improves review for a controlled annotated type,
but the evidence is still fixture-backed. A reviewer can trust the current
fixtures only because tests compare them to derived descriptor documents. The
next adoption gate requires deterministic evidence production or regeneration
from annotated Rust code.

## Approved automation boundary

The first automation slice may add one of these:

| Option | Allowed | Boundary |
|---|---:|---|
| Test-only evidence writer | yes | A test helper may serialize known annotated test contracts to retained JSON evidence. |
| Build/test-oriented example binary | yes | A controlled example may print descriptor JSON for known annotated contracts. |
| CLI descriptor document command for known fixture input | yes | CLI may serialize or validate descriptor documents without crate discovery. |
| Arbitrary crate scanning | no | Discovery across crates requires a later interface and validation wave. |
| External profile output | no | JSON Schema, CSDL, OpenAPI, AgentMap, and product adapters remain out of scope. |

## Required evidence properties

Any automation slice must:

1. Use `rune-core::DescriptorDocument` as the serialized descriptor shape.
2. Be deterministic: repeated runs produce stable JSON for the same annotated
   contract.
3. Preserve descriptor `id`, `version`, `kind`, `rust_type`, `fields`,
   `invariants`, `trace_links`, and `extensions`.
4. Fail closed when required durable descriptor fields are missing.
5. Keep generated evidence product-neutral.
6. Retain expected output under tests or validation fixtures.

## Initial target

The first target remains the controlled annotated `Customer` contract used in
Wave 4:

```text
crates/rune-derive/tests/derive_contract.rs
```

The automation must prove that
`crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json` can be
regenerated or checked from the annotated type without hand-maintaining a
separate descriptor model.

## First evidence path

The first implementation is a test-only writer in
`crates/rune-derive/tests/derive_contract.rs`.

```powershell
$env:RUNE_UPDATE_EVIDENCE = '1'
cargo test -p rune-derive --test derive_contract update_retained_customer_descriptor_when_requested
Remove-Item Env:RUNE_UPDATE_EVIDENCE
```

Normal test runs do not write files. When the environment variable is set, the
test serializes `DescriptorDocument::from_contract::<Customer>()` and rewrites
`crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json`.

This is intentionally narrow: it proves deterministic evidence regeneration for
one known annotated contract before any crate discovery surface exists.

## Non-goals

- Do not scan arbitrary Rust crates.
- Do not add a workspace-wide descriptor registry.
- Do not add external target formats.
- Do not add downstream product vocabulary.
- Do not treat manually maintained fixtures as sufficient for broad adoption.

## Validation command

```powershell
git diff --check
cargo test --workspace
```

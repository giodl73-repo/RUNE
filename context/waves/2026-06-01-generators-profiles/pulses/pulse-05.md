# Pulse 05: Derive-to-document bridge

## Goal

Close the automation gap between an annotated Rust type and retained neutral
descriptor JSON.

## Engineering decision supported

RUNE can expose a reusable core document shape for descriptors emitted by
`#[derive(RuneContract)]`, so tests and future tooling can compare generated
descriptor evidence without hand-rebuilding the JSON shape in each consumer.

## Trace links expected

- `docs/architecture/interface-control.md`
- `crates/rune-core/src/lib.rs`
- `crates/rune-derive/src/lib.rs`
- `crates/rune-derive/tests/derive_contract.rs`
- `crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VERIFICATION.md`

## Evidence produced

- `rune-core::DescriptorDocument` owned serializable descriptor shape.
- `DescriptorDocument::from_contract::<T>()` bridge from `RuneContract`.
- Neutral `requirement` derive attribute for requirement trace links.
- Derive integration test comparing the annotated `Customer` descriptor document
  to the retained descriptor fixture.

## Result

Complete with limits. This bridges derive output to retained neutral descriptor
JSON for test/evidence use. It does not yet implement arbitrary crate discovery,
CLI build integration, or external profile generation.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

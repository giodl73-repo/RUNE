# Pulse 05: Scaffold alignment

## Goal

Align the scaffolded `rune-core`, `rune-derive`, and `rune-cli` surfaces with
the approved Stage 4 interface-control spec, without adding generators,
adapters, or broader CLI commands.

## Engineering decision supported

The existing Rust scaffold becomes the first Stage 5 implementation slice only
for neutral descriptors, the `RuneContract` trait, the first derive macro, and a
bounded `status` command.

## Trace links expected

- `docs/architecture/interface-control.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/REVIEW.md`

## Evidence produced

- `ContractDescriptor` aligned with `id`, `version`, `kind`, `rust_type`,
  `fields`, `invariants`, `trace_links`, and `extensions`.
- `#[derive(RuneContract)]` aligned with neutral `id`, `version`, and `kind`
  attributes.
- Focused tests for the approved descriptor and derive slice.
- Bounded CLI status output that names deferred commands honestly.

## Result

Complete. Stage 5 aligned the scaffold to the Stage 4 interface-control spec
without adding generators, adapters, or broader CLI commands.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
git diff --check
```

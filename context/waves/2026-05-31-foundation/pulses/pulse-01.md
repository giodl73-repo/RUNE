# Pulse 01: Foundation scaffold

## Goal

Create the initial RUNE repo scaffold, product thesis, VTRACE package, workspace
crates, and first neutral descriptor surface.

## Engineering decision supported

RUNE starts as a product-neutral Rust contract derivation layer. Its first
consumer scenarios may include agent systems, context substrates, trace systems,
schema systems, IDL-style definitions, and workflow runtimes, but its base specs
do not depend on any of them.

## Trace links expected

- `docs/vtrace/MISSION.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VERIFICATION.md`
- `docs/vtrace/REVIEW.md`

## Evidence produced

- Rust workspace with `rune-core`, `rune-derive`, and `rune-cli`.
- Initial neutral descriptor trait and derive macro.
- Validation command results recorded before commit.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
```

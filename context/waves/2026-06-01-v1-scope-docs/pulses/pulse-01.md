# Pulse 01: V1 boundary decision

## Goal

Record the expanded v1 completion target.

## Engineering decision supported

V1 includes deterministic discovery, external profiles, and downstream adapters,
while preserving product-neutral core vocabulary.

## Evidence produced

- `README.md`
- `PRODUCT_PLAN.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`

## Result

Complete. The v1 target is explicit and keeps implementation of discovery,
profiles, and adapters behind later interface gates.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

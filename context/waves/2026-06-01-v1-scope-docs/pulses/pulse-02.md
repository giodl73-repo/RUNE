# Pulse 02: Documentation reconciliation

## Goal

Align user-facing documentation with the current implemented CLI surface.

## Engineering decision supported

The documented CLI surface should match retained evidence commands before new
v1 work begins.

## Evidence produced

- `README.md`
- `PRODUCT_PLAN.md`

## Result

Complete. README now lists the implemented status, profile, inspect, inventory,
check, and generate commands, plus a quick start for derive metadata, explicit
registration, and retained evidence commands.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

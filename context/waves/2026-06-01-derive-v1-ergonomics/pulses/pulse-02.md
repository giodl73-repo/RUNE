# Pulse 02: Required identity/version diagnostics

## Goal

Make missing durable identity and version fail at compile time.

## Engineering decision supported

Generated descriptors should not rely on scaffold defaults for durable v1
evidence.

## Evidence produced

- `crates/rune-derive/src/lib.rs`
- `crates/rune-derive/tests/ui/fail_missing_id.rs`
- `crates/rune-derive/tests/ui/fail_missing_version.rs`
- `crates/rune-derive/tests/compile.rs`

## Result

Complete. Missing `id` and missing `version` produce compile-time errors.

## Validation

```powershell
cargo test -p rune-derive --test compile
cargo fmt --check
cargo test --workspace
git diff --check
```

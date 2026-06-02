# Pulse 02: Inspection implementation slice

## Goal

Implement the minimal fixture-backed `rune inspect --fixture <path>` command as
a read-only descriptor inspection surface.

## Engineering decision supported

RUNE can expose neutral descriptor metadata for review and later validation
without introducing profile generators or downstream product mappings.

## Trace links expected

- `docs/architecture/inspection-surface.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VALIDATION.md`

## Evidence produced

- `rune-cli inspect --fixture <path>` reads a neutral descriptor fixture.
- Inspection preserves `id`, `version`, `kind`, `rust_type`, `fields`,
  `invariants`, `trace_links`, and `extensions`.
- Missing descriptor identity fails closed with `RUNE-INSP-001`.
- CLI tests cover pass and fail behavior.

## Result

Complete. The implementation adds fixture-backed read-only inspection, keeps
profile generation deferred, and validates pass/fail CLI behavior.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
git diff --check
```

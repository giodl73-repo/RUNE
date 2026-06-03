# Wave 42: Semantic registry

## Goal

Implement the first Mission 2.0 semantic registry slice as retained, validated
evidence over existing descriptor collection/profile/adapter concepts.

## Thesis

RUNE can make native Rust systems more manageable without runtime reflection by
letting crates and workspaces retain explicit semantic registry documents.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Retained registry model | complete | Added semantic registry document types, validation, retained fixtures, and VTRACE records. |

## Success criteria

- Registry documents expose id, version, owner, scope, collections, profiles,
  adapters, and capabilities.
- Missing id/version, duplicate collection refs, unsupported scopes, and runtime
  capability declarations fail closed.
- Runtime host behavior, live state inspection, Cargo traversal, source scraping,
  and mutation remain blocked.
- Retained fixtures cover crate, workspace, duplicate collection, and runtime
  blocked cases.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
git diff --check
```


# Pulse 01: Retained compatibility validation

## Goal

Deliver the retained compatibility negotiation implementation approved by
DCR-RUNE-007.

## Implementation

- Added `CompatibilityReportDraft` and `CompatibilityReportDocument`.
- Added retained compatibility artifact refs and degraded concept records.
- Added validation for source refs, target refs, source/target versions,
  unsupported compatible claims, unapproved degraded behavior, automatic migration
  requests, and runtime-host blocking.
- Added retained collection/profile, collection/adapter, and registry/state graph
  fixtures plus fail-closed failure fixtures.
- Added `check-compatibility --fixture <path> --registry <path>`.

## Boundaries

No automatic migration, best-effort conversion, runtime host negotiation, live
endpoint behavior, live state inspection, mutation/replay, private data exposure,
Cargo traversal, source scraping, plugin discovery, or policy enforcement was
added.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
cargo run -q -p rune-cli -- check-compatibility --fixture crates\rune-cli\tests\fixtures\compatibility_collection_profile.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
git diff --check
```

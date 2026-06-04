# Compatibility validation

Run these commands before treating retained compatibility reports as ready.

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -p rune-cli -- check-compatibility --fixture crates\rune-cli\tests\fixtures\compatibility_collection_profile.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -p rune-cli -- check-compatibility --fixture crates\rune-cli\tests\fixtures\compatibility_collection_adapter.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
git diff --check
```

Expected result: the retained compatible and incompatible reports validate, while
failure fixtures reject unknown refs, unsupported versions, unapproved degraded
behavior, unsupported compatible claims, and runtime-host negotiation.

Boundary: this runbook validates retained report evidence only. It does not
approve automatic migration, best-effort conversion, runtime host behavior, live
inspection, mutation, source scraping, Cargo traversal, plugin discovery, or
policy enforcement.

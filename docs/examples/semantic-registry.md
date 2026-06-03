# Semantic registry example

The semantic registry fixtures show how a workspace can retain a deterministic
index over descriptor collections, profiles, adapters, and capability flags.

| Fixture | Purpose |
|---|---|
| `crates\rune-cli\tests\fixtures\semantic_registry_workspace.json` | Valid workspace registry with two retained collection refs. |
| `crates\rune-cli\tests\fixtures\semantic_registry_workspace.check.json` | Expected `check-registry` report. |
| `crates\rune-cli\tests\fixtures\semantic_registry_workspace.inspect.json` | Expected `inspect-registry` report. |
| `crates\rune-cli\tests\fixtures\semantic_registry_duplicate_collection.json` | Duplicate collection failure case. |
| `crates\rune-cli\tests\fixtures\semantic_registry_mismatched_collection.json` | Collection source-ref mismatch failure case. |
| `crates\rune-cli\tests\fixtures\semantic_registry_runtime_blocked.json` | Runtime capability rejection case. |

Run the retained example commands:

```powershell
cargo run -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -p rune-cli -- inspect-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

Registry fixtures are evidence, not runtime discovery. Source refs point to
retained local descriptor collection JSON and are validated read-only.


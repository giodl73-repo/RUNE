# Validate a compatibility report

Use `check-compatibility` when you have a retained compatibility report that
claims whether a collection, profile, adapter, registry, state graph, or protocol
ref is compatible, compatible with warnings, incompatible, or blocked.

```powershell
cargo run -p rune-cli -- check-compatibility --fixture crates\rune-cli\tests\fixtures\compatibility_collection_profile.json --registry crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
```

The command validates the report against the retained semantic registry and
approved registry refs. It fails closed when the report references unknown source
or target artifacts, unsupported versions, unsupported concepts in a compatible
claim, unapproved degraded behavior, or runtime-host negotiation.

`check-compatibility` does not migrate artifacts, downgrade behavior, inspect live
state, mutate state, call a runtime host, scrape source, traverse Cargo metadata,
or enforce policy.

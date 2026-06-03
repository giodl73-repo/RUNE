# Semantic registry walkthrough

## Decision supported

A RUNE adopter can publish retained semantic registry evidence that identifies
its descriptor collections, profile and adapter compatibility claims, and
capability boundaries without requiring runtime reflection or source scraping.

## Inputs

| Input | Location |
|---|---|
| Workspace registry | `crates\rune-cli\tests\fixtures\semantic_registry_workspace.json` |
| Check report | `crates\rune-cli\tests\fixtures\semantic_registry_workspace.check.json` |
| Inspection report | `crates\rune-cli\tests\fixtures\semantic_registry_workspace.inspect.json` |

## Walkthrough

1. The registry declares a durable id, version, owner, and `workspace` scope.
2. It references retained descriptor collections by id, version, source ref, and
   owner.
3. `check-registry` validates identity, scope, duplicate refs, profile/adapter
   catalog refs, collection source refs, and blocked runtime capability.
4. `inspect-registry` emits the validated registry plus collection summaries and
   descriptor-kind inventories.
5. Failure fixtures prove that duplicate refs, mismatched collection refs,
   unknown profiles, and runtime capability claims fail closed.

## Validation commands

```powershell
cargo run -p rune-cli -- check-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo run -p rune-cli -- inspect-registry --fixture crates\rune-cli\tests\fixtures\semantic_registry_workspace.json
cargo test -p rune-cli --test registry_cli
```

## Boundary

The walkthrough does not require source scraping, Cargo traversal, executable
hooks, plugin discovery, mutation, live state inspection, or runtime host
behavior.


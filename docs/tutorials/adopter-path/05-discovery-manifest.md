# Lesson 5: Discovery manifest

## What you will learn

- How manifest discovery assembles retained evidence.
- Why RUNE v1 discovery is deterministic and reviewed.
- What discovery intentionally does not do.

## Step 1: Create a manifest

List retained descriptor collection fixtures in the order they should be merged.

```json
{
  "id": "example.discovery",
  "version": "v0",
  "sources": [
    {
      "kind": "descriptor_collection_fixture",
      "path": "examples/rune-adopter/tests/fixtures/adopter_contract_collection.json"
    }
  ]
}
```

## Step 2: Run discovery

```powershell
cargo run -p rune-cli -- discover --manifest <discovery-manifest.json>
```

## What you learned

Manifest discovery is a deterministic evidence merge. It is not arbitrary source
analysis, Cargo traversal, or hook execution.

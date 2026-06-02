# Use a discovery manifest

Use manifest discovery when you need a deterministic collection assembled from
approved retained collection sources.

## Manifest shape

A manifest lists retained descriptor collection sources in the intended merge
order.

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

## Run discovery

```powershell
cargo run -p rune-cli -- discover --manifest crates\rune-cli\tests\fixtures\adopter_discovery_manifest.json
```

## Rules

- Source order is deterministic.
- Malformed manifests fail closed.
- Unsupported source kinds fail closed.
- Duplicate descriptor ids fail closed.
- Discovery does not scan Rust source or Cargo metadata.

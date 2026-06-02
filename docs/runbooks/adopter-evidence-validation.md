# Adopter evidence validation runbook

## Purpose

Validate that the v1 adopter workflow is still executable from retained fixtures
and approved CLI commands.

Run this from the RUNE repository root.

## Inputs

| Input | Path |
|---|---|
| Adopter crate | `examples\rune-adopter` |
| Adopter collection fixture | `examples\rune-adopter\tests\fixtures\adopter_contract_collection.json` |
| Discovery manifest fixture | `crates\rune-cli\tests\fixtures\adopter_discovery_manifest.json` |
| Discovered collection fixture | `crates\rune-cli\tests\fixtures\adopter_discovered_collection.json` |
| Discovered evidence bundle fixture | `crates\rune-cli\tests\fixtures\adopter_discovered_collection.evidence_bundle.json` |
| Documentation packet fixture | `examples\rune-adopter\tests\fixtures\adopter_documentation_packet.json` |
| Review packet fixture | `examples\rune-adopter\tests\fixtures\adopter_review_packet.json` |

## 1. Verify the adopter-owned registry

```powershell
cargo test -p rune-adopter
```

This proves the adopter crate can derive descriptors, expose an explicit
registry, retain deterministic collection evidence, and fail closed on duplicate
descriptor ids.

## 2. Inspect retained collection evidence

```powershell
cargo run -p rune-cli -- inspect-collection --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

Check that the output preserves collection identity, version, descriptor ids,
descriptor kinds, Rust types, fields, and trace links.

## 3. Check neutral profile compatibility

```powershell
cargo run -p rune-cli -- check-collection --profile rune.neutral_descriptor_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

The report should make profile compatibility explicit instead of relying on
prose or source inspection.

## 4. Inventory the collection

```powershell
cargo run -p rune-cli -- inventory-collection --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

Use the inventory to confirm descriptor-kind coverage before sending the
collection to profiles or adapters.

## 5. Run manifest discovery

```powershell
cargo run -p rune-cli -- discover --manifest crates\rune-cli\tests\fixtures\adopter_discovery_manifest.json
```

Discovery should merge retained descriptor collections in manifest order. It
does not scan Rust source, traverse Cargo metadata, or execute hooks.

## 6. Emit a discovery-backed evidence bundle

```powershell
cargo run -p rune-cli -- evidence-collection --profile rune.neutral_descriptor_json --manifest crates\rune-cli\tests\fixtures\adopter_discovery_manifest.json
```

The bundle combines the validated collection, compatibility check, inventory,
and neutral generated artifact. Treat it as review evidence, not as an automatic
publication step.

## 7. Emit documentation packet profile output

```powershell
cargo run -p rune-cli -- generate-collection --profile rune.documentation_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

Compare the output to:

```text
examples\rune-adopter\tests\fixtures\adopter_documentation_packet.json
```

## 8. Emit review packet adapter output

```powershell
cargo run -p rune-cli -- adapt-collection --adapter rune.review_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

Compare the output to:

```text
examples\rune-adopter\tests\fixtures\adopter_review_packet.json
```

## Pass criteria

- All commands exit successfully.
- Descriptor ids, versions, kinds, fields, and trace links are visible in
  retained evidence.
- Profile and adapter outputs preserve their identity and version metadata.
- Discovery uses only the reviewed manifest boundary.
- No command requires source scraping, Cargo traversal, executable hooks, or
  product-specific vocabulary in `rune-core`.

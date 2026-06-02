# Lesson 4: Profiles and adapters

## What you will learn

- How to emit a documentation packet profile.
- How to emit a review packet adapter.
- Where downstream vocabulary belongs.

## Step 1: Generate a documentation packet

```powershell
cargo run -p rune-cli -- generate-collection --profile rune.documentation_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

## Step 2: Generate a review packet

```powershell
cargo run -p rune-cli -- adapt-collection --adapter rune.review_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

## Step 3: Compare retained outputs

```text
examples\rune-adopter\tests\fixtures\adopter_documentation_packet.json
examples\rune-adopter\tests\fixtures\adopter_review_packet.json
```

## What you learned

Profiles and adapters create useful artifacts while keeping `rune-core` neutral.

## Next

[Lesson 5: Discovery manifest](05-discovery-manifest.md)

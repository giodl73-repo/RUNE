# Lesson 3: Evidence commands

## What you will learn

- How to inspect retained collection evidence.
- How to check profile compatibility.
- How to generate a read-only evidence bundle.

## Step 1: Inspect

```powershell
cargo run -p rune-cli -- inspect-collection --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

## Step 2: Check

```powershell
cargo run -p rune-cli -- check-collection --profile rune.neutral_descriptor_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

## Step 3: Bundle

```powershell
cargo run -p rune-cli -- evidence-collection --profile rune.neutral_descriptor_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

## What you learned

RUNE commands make contract evidence reviewable without mutating fixtures by
default.

## Next

[Lesson 4: Profiles and adapters](04-profiles-and-adapters.md)

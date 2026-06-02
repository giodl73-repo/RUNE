# Shape calculator

`examples\rune-shape-calculator` is a small data-contract scenario for proving
metadata-driven contracts before larger repo adoption.

| Contract | Kind | Metadata shown |
|---|---|---|
| `Circle` | `entity` | radius field, non-negative invariant, area/perimeter formula extensions |
| `Rectangle` | `entity` | width/height fields, non-negative invariants, formula extensions |
| `CalculateShape` | `command` | request fields, precision invariant, operation extension |
| `ShapeCalculated` | `event` | area/perimeter fields and non-negative result invariants |

The example retains three evidence fixtures:

| Fixture | Purpose |
|---|---|
| `shape_contract_collection.json` | Neutral descriptor collection evidence. |
| `shape_documentation_packet.json` | Documentation profile evidence. |
| `shape_data_contract_profile.json` | Data-contract profile evidence preserving fields, invariants, trace links, and extensions. |

Validation:

```powershell
cargo test -p rune-shape-calculator
cargo run -q -p rune-cli -- generate-collection --profile rune.data_contract_json --fixture examples\rune-shape-calculator\tests\fixtures\shape_contract_collection.json
```

The review packet adapter intentionally rejects this collection because it cannot
represent invariants/extensions yet. That fail-closed behavior is part of the
evidence: richer metadata should not be silently dropped.

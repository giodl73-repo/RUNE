# Evidence fixtures example

The adopter example retains three core output fixtures:

| Fixture | Purpose |
|---|---|
| `examples\rune-adopter\tests\fixtures\adopter_contract_collection.json` | Canonical descriptor collection for the adopter crate. |
| `examples\rune-adopter\tests\fixtures\adopter_documentation_packet.json` | External documentation packet profile output. |
| `examples\rune-adopter\tests\fixtures\adopter_review_packet.json` | Review packet adapter output. |

Reviewers can compare these fixtures to command output:

```powershell
cargo run -p rune-cli -- generate-collection --profile rune.documentation_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
cargo run -p rune-cli -- adapt-collection --adapter rune.review_packet_json --fixture examples\rune-adopter\tests\fixtures\adopter_contract_collection.json
```

Fixture changes should be reviewed as contract evidence changes, not treated as
incidental snapshots.

# Pulse 02: RUNE evidence comparison

## Goal

Compare source-only review against retained RUNE evidence.

## Engineering decision supported

RUNE evidence makes contract identity, version, kind, profile compatibility, and
review prompts explicit for selected QUIVER contracts.

## Evidence produced

- `docs\vtrace\fixtures\quiver_bakeoff_descriptor_collection.json`
- `docs\vtrace\fixtures\quiver_bakeoff.check.json`
- `docs\vtrace\fixtures\quiver_bakeoff.documentation_packet.json`
- `docs\vtrace\fixtures\quiver_bakeoff.review_packet.json`

## Result

Complete. RUNE evidence improved inspectability and review usefulness while
remaining scenario-only.

## Validation

```powershell
cargo run -p rune-cli -- check-collection --profile rune.neutral_descriptor_json --fixture docs\vtrace\fixtures\quiver_bakeoff_descriptor_collection.json
cargo run -p rune-cli -- generate-collection --profile rune.documentation_packet_json --fixture docs\vtrace\fixtures\quiver_bakeoff_descriptor_collection.json
cargo run -p rune-cli -- adapt-collection --adapter rune.review_packet_json --fixture docs\vtrace\fixtures\quiver_bakeoff_descriptor_collection.json
cargo test --workspace
git diff --check
```

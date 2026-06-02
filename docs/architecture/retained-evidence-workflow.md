# Retained evidence workflow

## Purpose

RUNE retained evidence should be reproducible without making normal validation
mutate files. The first workflow is a read-only collection evidence bundle that
can be redirected into a fixture when maintainers intentionally refresh evidence.

## Approved inputs

The workflow accepts either:

- a validated descriptor collection fixture, or
- an approved deterministic discovery manifest that resolves to descriptor
  collection fixtures.

Both paths preserve the neutral descriptor collection model. They do not scan
Rust source, traverse Cargo metadata, execute hooks, or invoke adapters.

## Command surface

```powershell
cargo run -p rune-cli -- evidence-collection --profile rune.neutral_descriptor_json --fixture <collection.json>
cargo run -p rune-cli -- evidence-collection --profile rune.neutral_descriptor_json --manifest <discovery-manifest.json>
```

## Output contract

The command emits one JSON bundle with:

- `collection`: validated descriptor collection evidence,
- `check_report`: read-only profile compatibility evidence,
- `inventory`: descriptor kind counts,
- `generated_artifact`: neutral collection artifact for the selected profile.

The bundle stores a portable source label, not a machine-specific absolute path.

## Mutation rule

Normal validation compares retained fixtures and does not rewrite them. Evidence
refresh is opt-in by redirecting command output to a retained fixture path.

## Non-goals

- No source scraping.
- No Cargo metadata discovery.
- No executable registry hooks.
- No external profile output.
- No downstream adapter output.

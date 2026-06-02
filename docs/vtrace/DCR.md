# RUNE Design Change Requests

Design change requests record planned changes to RUNE's controlled contract
surface before implementation. A DCR may approve a direction without claiming the
surface is implemented or stable.

## DCR-RUNE-001: Field-level data metadata

| Field | Value |
|---|---|
| Status | implemented first slice |
| Date | 2026-06-02 |
| Source requirement | RUNE-REQ-075 |
| Trigger | Shape data-contract scenario and games repo survey showed that data contracts need field-level metadata before broad game/simulation adoption. |
| Primary repo | RUNE |
| Coordinating repo | TRACKER |
| Scenario repos | RALLY, COURT, MUDDLE, RACKET, AMAZE, BANISH, QUEST, TIGRIS |

### Decision

RUNE should add a reviewed field-level metadata lane before trait/function
contracts. The next implementation wave should make field metadata explicit,
retained, and profile-visible while preserving the neutral core boundary.

First implementation slice completed under `RUNE-REQ-076`: `FieldDescriptor`
now carries explicit metadata, `#[rune_field(...)]` authors it, and
`rune.data_contract_json` preserves it in retained shape evidence.

### Proposed authoring surface

```rust
#[derive(RuneContract)]
#[rune(id = "example.shape.circle", version = "v0", kind = "entity")]
pub struct Circle {
    #[rune_field(
        required = true,
        unit = "px",
        min = "0",
        sensitivity = "public",
        example = "12.5",
        stability = "stable",
        alias = "radius"
    )]
    pub radius: f64,
}
```

Initial metadata keys:

| Key | Purpose | Initial representation |
|---|---|---|
| `required` | Whether a consumer may omit the field. | boolean |
| `unit` | Measurement unit or domain unit. | string |
| `min` / `max` | Declared boundary metadata. | string values to avoid premature numeric semantics |
| `sensitivity` | Review/privacy handling hint. | string |
| `example` | Copyable example value for docs and AI consumers. | string |
| `stability` | Compatibility expectation for the field. | string |
| `alias` | Alternate consumer-facing field name. | repeatable string |

### Scope

1. Extend RUNE descriptor documents with field metadata while preserving existing
   field `name` and `rust_type` behavior.
2. Add `#[rune_field(...)]` derive parsing with fail-closed diagnostics for
   unsupported keys.
3. Extend `rune.data_contract_json` to retain field metadata.
4. Upgrade `examples\rune-shape-calculator` to express units, bounds, examples,
   and stability for shape fields.
5. Add a games adoption spike document that maps COURT, MUDDLE, RALLY, and
   RACKET candidates to concrete RUNE annotations.
6. Defer trait/function contracts until the data-contract lane is verified.

### Non-goals

- Do not add game-specific vocabulary to `rune-core`.
- Do not make field metadata a runtime validator in this wave.
- Do not infer metadata from Rust doc comments or type names.
- Do not annotate games repos until the RUNE field metadata surface is reviewed,
  implemented, and pushed.
- Do not require TRACKER-relative Rust dependencies.

### Cross-repo ownership

| Repo | Owner action | Notes |
|---|---|---|
| RUNE | update | Implement field metadata, retained fixtures, docs, VTRACE, validation, and wave records. |
| TRACKER | update | Coordinate the portfolio wave and later record the RUNE submodule pointer after RUNE is pushed. |
| RALLY | defer | Preferred first adoption spike after RUNE field metadata lands because simulator/report structs are clean candidates. |
| COURT | defer | Candidate for snapshot/action/validation packet contracts after data metadata proves out. |
| MUDDLE | defer | Candidate for room/command/session/client snapshot contracts after RUNE metadata proves out. |
| RACKET | defer | Candidate for adapter runtime report/frame plan contracts after data metadata proves out. |
| AMAZE / BANISH / QUEST / TIGRIS | no-op for this DCR | Scenario-specific host/state contracts remain downstream candidates. |

### Proposed wave sequence

1. **Wave 36: Field metadata interface** - specify descriptor shape,
   `#[rune_field(...)]`, diagnostics, and compatibility rules.
2. **Wave 37: Field metadata implementation** - implement core, derive, CLI
   fixtures, and data-contract profile preservation.
3. **Wave 38: Shape metadata upgrade** - update the shape calculator evidence to
   include units, bounds, examples, stability, and sensitivity metadata.
4. **Wave 39: Games annotation spike plan** - produce a repo-by-repo annotation
   plan for RALLY, COURT, MUDDLE, and RACKET without editing games repos yet.
5. **Wave 40: First games adoption spike** - annotate one low-coupling game repo
   candidate, likely RALLY, only after RUNE field metadata is validated.

### Validation expectations

RUNE implementation waves must run:

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
git diff --check
```

TRACKER pointer waves must run:

```powershell
git diff --check
git diff --submodule=short -- repos\standards-protocols\rune
```

Games adoption spikes must use each game repo's documented validation commands
and must keep game-specific semantics in field metadata extensions, profile
outputs, or adapters rather than in RUNE core vocabulary.

### Open questions for Wave 36

| Question | Default for first implementation |
|---|---|
| Should `min` and `max` be typed numbers? | No; store as strings first to avoid cross-type coercion semantics. |
| Should aliases be repeatable? | Yes; preserve author order. |
| Should field metadata affect compatibility checks? | Not until profile support is explicit; unsupported concepts must fail closed. |
| Should docs comments be captured automatically? | No; require explicit metadata until doc capture has its own DCR. |
| Should trait/function contracts be designed now? | No; keep as the next lane after field metadata. |

# Pulse 03: RUNE evidence comparison

## Goal

Compare the source-only baseline against retained RUNE descriptor, compatibility
check, and neutral generated artifact evidence.

## Engineering decision supported

RUNE evidence should reduce inference burden for AI handoff and review without
introducing product-specific adapters or downstream target formats.

## Evidence reviewed

- `crates/rune-cli/tests/fixtures/annotated_customer_descriptor.json`
- `crates/rune-cli/tests/fixtures/annotated_customer.check.json`
- `crates/rune-cli/tests/fixtures/annotated_customer.neutral_descriptor_artifact.json`

## Comparison findings

| Question | Source-only burden | RUNE evidence result |
|---|---|---|
| What stable contract id should tools use? | Medium; requires macro semantics. | Explicit `id: example.customer` in descriptor and artifact. |
| What version is durable? | Medium; requires RUNE behavior knowledge. | Explicit `version: v0` and check report confirms descriptor version. |
| What kind is intended? | Medium; vocabulary meaning external to Rust. | Explicit `kind: entity` and check report confirms descriptor kind. |
| What requirement is linked? | Medium; relation semantics implicit. | Explicit trace link `{ relation: requirement, target: RUNE-REQ-034 }`. |
| Is the descriptor profile-compatible? | High; not answerable from source alone. | `check` reports `status: ok` for `rune.neutral_descriptor_json@v0`. |
| What artifact would downstream tooling consume? | High; not answerable from source alone. | Generated artifact declares `output_artifact_kind: rune.descriptor.json`. |
| Is profile metadata visible? | High; not answerable from source alone. | Generated artifact includes profile id and version. |
| Is neutrality preserved? | Manual review required. | Evidence uses RUNE neutral vocabulary only. |

## Result

Complete. RUNE evidence improves this constrained scenario by making stable
identity, version, kind, trace link, profile compatibility, profile metadata, and
generated artifact shape explicit. The result remains fixture-backed and does not
validate arbitrary crate discovery or external profiles.

## Validation

```powershell
cargo test --workspace
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
git diff --check
```

# RUNE Specification Baseline

## Scope

This baseline groups the accepted RUNE v1 contract infrastructure requirements
into controlled specification surfaces. It preserves the neutral core boundary:
product-specific vocabularies belong in external profiles or downstream
adapters, not in `rune-core`.

## Specification Rows

| Spec ID | Source IDs | Surface | Baseline Rule | Verification Method | Status |
|---|---|---|---|---|---|
| SPEC-RUNE-001 | NEED-001 / RUNE-REQ-010 / RUNE-REQ-011 / RUNE-REQ-012 / RUNE-REQ-013 | Neutral descriptor model | Descriptors carry stable identity, version, kind, fields, invariants, and trace links without downstream product vocabulary. | `cargo test --workspace` and descriptor fixture review | accepted |
| SPEC-RUNE-002 | NEED-001 / RUNE-REQ-020 / RUNE-REQ-021 / RUNE-REQ-044 / RUNE-REQ-059 | Derive evidence boundary | Derive-generated descriptor evidence must be inspectable, deterministic, and fail closed when durable identity or version is missing. | derive tests, trybuild tests, retained fixture review | accepted |
| SPEC-RUNE-003 | NEED-002 / RUNE-REQ-030 / RUNE-REQ-033 / RUNE-REQ-039 / RUNE-REQ-054 / RUNE-REQ-068 | CLI inspection and compatibility | CLI inspection/check commands are read-only or fail-closed and keep diagnostics stable enough for review. | CLI tests and fixture commands | accepted |
| SPEC-RUNE-004 | NEED-002 / RUNE-REQ-052 / RUNE-REQ-053 / RUNE-REQ-055 / RUNE-REQ-056 / RUNE-REQ-057 / RUNE-REQ-063 | Retained collection evidence | Collection workflows preserve deterministic order, compatibility status, inventory, generated artifacts, and retained evidence bundles. | core and CLI collection tests | accepted |
| SPEC-RUNE-005 | NEED-003 / NEED-005 / RUNE-REQ-034 / RUNE-REQ-035 / RUNE-REQ-037 / RUNE-REQ-038 / RUNE-REQ-043 / RUNE-REQ-064 / RUNE-REQ-065 | Profile and generator boundary | Profiles are reviewed mappings over neutral descriptors and fail closed for unsupported versions, kinds, or concepts. | profile catalog tests and generation fixtures | accepted |
| SPEC-RUNE-006 | NEED-003 / RUNE-REQ-060 / RUNE-REQ-061 / RUNE-REQ-062 / RUNE-REQ-066 / RUNE-REQ-067 | Discovery and adapter boundary | Discovery is manifest-controlled and deterministic; adapters consume validated evidence or profile outputs outside the neutral core. | discovery tests, adapter tests, retained review packet fixture | accepted |
| SPEC-RUNE-007 | NEED-004 / NEED-005 / RUNE-REQ-040 / RUNE-REQ-041 / RUNE-REQ-042 / RUNE-REQ-045 / RUNE-REQ-070 / RUNE-REQ-071 / RUNE-REQ-072 | Validation, release, and adoption docs | v1 readiness depends on CI-ready validation, representative bakeoff evidence, adopter docs, and corpus update rules. | workspace validation, QUIVER bakeoff fixtures, docs package review | accepted |
| SPEC-RUNE-008 | NEED-005 / RUNE-REQ-075 / RUNE-REQ-076 | Design change control and field metadata | Field-level metadata must enter through a DCR, then preserve explicit author metadata through descriptors, derive output, and data-contract profile evidence. | DCR review, derive tests, retained shape fixtures, and workspace validation | accepted |

## Non-Goals

- RUNE core does not encode downstream product vocabularies.
- RUNE descriptors are not source-scraping or prompt-convention substitutes.
- Broad adoption claims require retained evidence or representative bakeoff
  proof.

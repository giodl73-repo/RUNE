# Wave: Adoption validation runbook

## Goal

Add an operator runbook that validates the complete v1 adopter evidence path
using existing retained fixtures and approved CLI commands.

## Engineering decision supported

The adoption docs package is not only explanatory; it includes a concrete,
ordered command sequence that reviewers can run to validate adopter evidence
without expanding the RUNE code surface.

## Scope

This wave may add `docs\runbooks`, update documentation indexes, link the runbook
from existing guides and traces, and update VTRACE records. It must not add new
CLI behavior, source scraping, Cargo traversal, executable hooks, or
product-specific vocabulary in `rune-core`.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Adopter evidence runbook | complete |
| 02 | Runbook indexing and VTRACE closure | complete |

## Decision

Wave 34 passes as a documentation-only adoption validation runbook.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -q -p rune-cli -- status
git diff --check
```

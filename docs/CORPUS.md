# RUNE documentation corpus

This file defines the documentation surfaces that must stay synchronized as RUNE
evolves.

## Surfaces

| Surface | Purpose | Update when |
|---|---|---|
| `README.md` | Public landing page, quick start, and validation commands. | A user-facing crate, CLI command, profile, adapter, or validation command changes. |
| `PRODUCT_PLAN.md` | Product thesis, wave sequence, and non-goals. | A new wave changes adoption posture, roadmap, or scope boundaries. |
| `docs\README.md` | Documentation map and recommended reading paths. | A docs surface is added, renamed, or retired. |
| `docs\concepts\` | Durable mental models. | A concept changes or a new adopter decision needs explanation before procedures. |
| `docs\how-to\` | Task-oriented procedures. | A supported workflow gains or changes commands. |
| `docs\runbooks\` | Ordered operator validation sequences. | A supported end-to-end command path changes or gains retained evidence. |
| `docs\tutorials\` | Progressive learning paths. | A new end-to-end adoption path is supported. |
| `docs\examples\` | Copyable example references and retained outputs. | Example crate files, fixtures, or expected outputs change. |
| `docs\traces\` | End-to-end evidence walkthroughs. | A scenario, bakeoff, or adopter workflow becomes evidence for a decision. |
| `docs\architecture\` | Interface-control and design boundaries. | A stable interface, boundary, or non-goal changes. |
| `docs\vtrace\` | Requirements, trace, verification, validation, review. | Any requirement, evidence claim, validation result, or review decision changes. |
| `context\waves\` | Repo-local execution history. | Every wave or pulse. |

## Synchronization rules

1. A new command needs a how-to update and VTRACE evidence.
2. A new profile needs a concept or architecture boundary, a how-to, retained
   fixtures, and VTRACE trace rows.
3. A new adapter needs an adapter concept update, retained output evidence, and a
   clear statement that adapter vocabulary stays outside `rune-core`.
4. A new example crate needs an examples page and a tutorial or how-to if it is
   intended for adopters.
5. A new bakeoff needs a trace walkthrough plus `docs\vtrace\BAKEOFF.md` and
   `docs\vtrace\VALIDATION.md` updates.
6. A new runbook needs concrete command inputs, retained evidence references,
   pass criteria, and validation under the current CLI surface.
7. A new semantic registry command needs how-to, runbook, example, trace,
   communications strategy, VTRACE, and release/readiness updates.
8. A new state graph command needs how-to, runbook, retained fixtures, VTRACE,
   and explicit live-state/runtime-host boundary language.
9. A new evidence packet command needs how-to, runbook, retained pass/failure
   fixtures, VTRACE, release/readiness updates, and explicit logging/runtime-host
   non-goals.
10. A new agent protocol command needs how-to, runbook, retained pass/failure
    fixtures, VTRACE, release/readiness updates, and explicit read-first,
    no-live-endpoint, no-mutation boundary language.

## Documentation validation

For documentation-only changes, run:

```powershell
git diff --check
```

If Rust examples, fixtures, or CLI outputs change, also run:

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
```

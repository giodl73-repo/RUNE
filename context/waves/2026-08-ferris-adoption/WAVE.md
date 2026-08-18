# Wave: FERRIS Procedural-Macro Consumer Contract

Status: Complete

## Product outcome

Give RUNE one consumer-owned compatibility proof for the exact FERRIS
`validation-plan` projection over a resolver-3 workspace with a procedural
macro and compile-test workload, without replacing RUNE validation.

## Frame

RUNE owns these required validation commands:

```console
cargo fmt --check
cargo test --workspace
cargo run -p rune-cli -- status
git diff --check
```

FERRIS owns read-only Cargo workspace discovery and package-closure planning.
The missing evidence is whether its PARLOR-tested projection generalizes to
RUNE's materially different topology. A `rune-derive` edit must retain the
procedural-macro anchor plus its two Cargo-declared adopters, while a
repository-level edit must retain all six workspace packages as fallback.

The proof does not claim that FERRIS understands macro expansion, `trybuild`,
features, targets, doctests, runtime status, or repository policy. RUNE keeps
those semantics and all execution authority.

The deletion target is manual reconstruction of this second-consumer control.
The thesis is disproved if the proof requires RUNE product-source, manifest, or
lockfile changes; copies a FERRIS schema; executes a selected plan; or narrows
RUNE validation.

## Product Value Governor

Disposition: `continue-within-budget`

Approved budget:

- exactly one consumer pulse and one implementation attempt;
- one exact public FERRIS commit pin;
- one RUNE-owned contract, checker, and Ubuntu workflow;
- one procedural-macro projection and one repository fallback;
- one all-eleven-role closeout; and
- no RUNE product-source, manifest, lockfile, or validation-command change.

Completion condition:

- build exact FERRIS commit
  `35f3518b6597acde34641a4f55e5111405334e70`;
- invoke its exact `cargo-ferris` adapter from a nested RUNE directory;
- bind `ferris.command-result/v2`, `ferris.validation-plan/v0`, command version
  `0.1.0`, and workspace ID `org.giodl73/rune`;
- prove the `rune-derive` anchor plus `rune-adopter` and
  `rune-shape-calculator` reverse dependencies;
- prove the six-package repository fallback;
- preserve RUNE's four owner validation commands; and
- exercise exact-pin migration and rollback.

Abandonment condition:

Stop `stop-value-exhausted` without a successor if RUNE only reproduces
PARLOR's topology, requires hidden state or copied schemas, or cannot preserve
procedural-macro and compile-test limitations explicitly.

## Compare

| Analogue | Classification | Use |
|---|---|---|
| RUNE README validation | reuse | remains the required owner contract |
| PARLOR FERRIS consumer proof | adapt | reuse exact-pin and alias-resistant mechanics, not PARLOR expectations |
| Full FERRIS JSON snapshot | avoid | bind only the consumer-owned projection |
| Copied FERRIS schema | avoid | preserve one schema owner |
| Selected-plan execution | avoid | FERRIS remains non-executable |

## Pulse table

| Pulse | Title | Status | Outcome |
|---:|---|---|---|
| 01 | Exact procedural-macro compatibility proof | Complete | Local, fetched, lifecycle, role, and Ubuntu CI gates passed |

## Closeout

The implementation evidence is recorded in
[`pulses/pulse-01.md`](pulses/pulse-01.md), and the all-eleven-role decision is
recorded in [`REVIEW.md`](REVIEW.md). RUNE PR
[#1](https://github.com/giodl73-repo/RUNE/pull/1) satisfied the consumer Ubuntu
workflow merge gate in run
[`32182071143`](https://github.com/giodl73-repo/RUNE/actions/runs/32182071143).

## Migration, rollback, and removal

To evaluate a later FERRIS revision, change only the exact pin and intentionally
adopted command/schema versions in `tools/ferris-contract/contract.json`, then
run:

```console
python tools/ferris-contract/check.py --ferris-source <EXACT_FERRIS_CHECKOUT>
```

Rollback restores the prior exact pin and reruns the proof. Complete removal
deletes `.github/workflows/ferris-contract.yml`,
`tools/ferris-contract/`, this wave, and the README compatibility paragraph.
No operation changes a RUNE manifest, lockfile, crate, example, semantic
contract, or owner validation command.

## Non-goals

- executing FERRIS-selected activities;
- understanding procedural-macro expansion or `trybuild` case semantics;
- replacing full-workspace tests or the RUNE status command;
- claiming FERRIS support, stability, feature completeness, or performance;
- copying a FERRIS schema or adding a FERRIS crate dependency; and
- automatically advancing the pin or creating a successor.

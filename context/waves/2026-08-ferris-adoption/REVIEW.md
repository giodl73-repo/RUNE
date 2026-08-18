# FERRIS Procedural-Macro Consumer Contract Role Review

Status: Accepted and complete
Scope: RUNE-owned exact FERRIS `validation-plan` compatibility proof

## Product Value Governor

Pass. RUNE is a public second consumer with a resolver-3, procedural-macro,
`trybuild`, and example-adopter topology that differs materially from PARLOR.
The slice adds new generality evidence without changing RUNE product code,
manifests, lockfiles, dependencies, or validation commands.

## Rust Safety Steward

Pass. No Rust source changes. The checker builds one exact safe-Rust FERRIS pin
in temporary custody and invokes only its read-only, non-executable planning
command.

## Compiler Performance Engineer

Pass with no performance claim. The proof adds one bounded FERRIS build and two
planning invocations. It does not infer compile-time savings or authorize
selected execution for procedural-macro or `trybuild` workloads.

## Interop Boundary Auditor

Pass. Cargo retains metadata and dependency authority. FERRIS retains command
and plan semantics. RUNE binds only the exact command/schema IDs, portable
input paths, package closure, activities, and fallback needed for this
consumer-owned projection.

## AI Assurance Skeptic

Pass. The checker fetches or verifies an exact immutable commit, rejects a
dirty FERRIS checkout, builds with the pinned lockfile, invokes the exact
binary rather than a Cargo alias, and checks accepted and fallback outcomes.
Lifecycle migration and rollback were exercised rather than asserted.

## Ecosystem Strategist

Pass. The second consumer proves a materially different topology: a
procedural-macro anchor with two example adopters, resolver 3, dev-dependency
compile tests, and a six-package fallback. No schema, crate, or validation
policy is copied from FERRIS.

## Rust Maintainer

Pass. RUNE crates, examples, manifests, and lockfile are untouched. The
contract surface is isolated under `tools/`, `.github/`, and this wave, and can
be removed without affecting normal Cargo behavior.

## Native Platform Adopter

Pass. The exact-pin proof, full RUNE tests, and status command passed on
Windows. The consumer workflow also passed on Ubuntu at the immutable
pull-request event revision.

## Scope Keeper

Pass. The proof does not execute selected work, replace full tests, interpret
macro expansion or `trybuild`, add dependencies, claim support, or authorize
automatic upgrades or a successor.

## Validation Checker

Pass. Local exact-checkout and fetched-pin modes passed,
including an injected Cargo alias. Python compilation, formatting, full
workspace tests, the RUNE status command, and diff hygiene passed. The checker
also binds both portable changed paths.

## Autonomy Supervisor

Pass. Product outcome, deletion target, one-pulse budget, completion condition,
and abandonment condition preceded implementation. Migration from FERRIS
`35f3518` to `5cd1aa9` and rollback to `35f3518` both passed with no retained
pin change. No successor authority is granted.

## Control record

- product outcome: second-consumer procedural-macro compatibility proof;
- value obtained: evidence that the same narrow FERRIS projection generalizes
  beyond PARLOR's sibling-crate topology;
- owner authority retained: all RUNE execution, macro, compile-test, runtime,
  feature, target, doctest, and policy semantics;
- budget consumed: one pulse and one implementation attempt;
- migration: update the exact pin and intentionally adopted IDs, then rerun
  local and consumer CI proof;
- rollback: restore the previous pin or delete the isolated proof surface;
- proposed next action: merge only after consumer CI, then reconcile FERRIS's
  public reuse posture with this exact second-consumer evidence; and
- disposition: `continue-within-budget`.

## Decision

All eleven roles accept the bounded contract. The Ubuntu consumer CI merge gate
passed in RUNE PR
[#1](https://github.com/giodl73-repo/RUNE/pull/1), workflow run
[`32182071143`](https://github.com/giodl73-repo/RUNE/actions/runs/32182071143).
No role grants execution, validation narrowing, support, general stability,
source modification, or successor authority.

Codex review was attempted three times with `codex review --uncommitted` and
was unavailable because of account capacity. Its required closeout function
was replaced by direct invariant review; no actionable finding remained after
binding the portable input paths explicitly.

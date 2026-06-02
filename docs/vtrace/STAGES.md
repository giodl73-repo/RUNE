# RUNE VTRACE Stages

## Engineering decision supported

RUNE will advance through explicit VTRACE stage gates before expanding its macro
surface, generator surface, or downstream adoption. Implementation may exist
early as scaffold, but no implementation surface is contract-stable until its
preceding VTRACE stage exits.

## Stage order

| Stage | Name | Purpose | Entry criteria | Exit criteria | Current status |
|---|---|---|---|---|---|
| 0 | Mission | Define why RUNE exists and what success means. | Repo exists and owner intent is captured. | Mission need, stakeholder needs, and intended outcome are reviewed. | pass |
| 1 | Stakeholder needs | Identify the users and tensions RUNE must satisfy. | Mission draft exists. | Needs cover maintainers, AI tooling, adapter authors, assurance reviewers, and platform contract authors. | pass |
| 2 | Requirements | Convert needs into stable, verifiable requirements. | Stakeholder needs are named. | Requirements have IDs, verification methods, and trace links. | pass |
| 3 | Concept and architecture | Define the neutral descriptor model and profile boundary. | Requirements exist. | Core concepts, non-goals, and adapter/profile boundary are reviewed. | pass |
| 4 | Interface and control specs | Define public macro, trait, descriptor, CLI, and generator interfaces. | Concept model is reviewed. | Interface specs exist before contract-stable implementation claims. | pass |
| 5 | Implementation slices | Implement the smallest complete crate and CLI surfaces. | Interface specs identify the allowed slice. | Code compiles, tests pass, and generated behavior is inspectable. | pass |
| 6 | Verification | Prove implementation satisfies requirements. | Implementation slice exists. | Formatting, tests, CLI checks, macro checks, and generated artifact checks pass. | pass |
| 7 | Validation | Prove RUNE solves the intended problem in representative Rust repos. | Verified slice exists. | Bakeoff scenarios show improved contract generation, AI comprehension, review, or handoff. | deferred with plan |
| 8 | Review and readiness | Decide whether the stage can broaden adoption or must loop back. | Verification and validation evidence exist. | `.roles` review findings are resolved, waived, or converted into follow-up pulses. | pass with limits |

## Stage gates

### Stage 0: Mission

Allowed work:

- clarify mission need,
- name intended outcome,
- reject out-of-scope product coupling.

Exit evidence:

- `docs/vtrace/MISSION.md`.

Current findings:

- The mission now captures neutral Rust contract derivation and IDL-style
  generated artifacts.
- Stage 0 role review passed on 2026-06-01. Later questions about descriptor
  detail, macro behavior, diagnostics, and generated examples belong to later
  stages.

### Stage 1: Stakeholder needs

Allowed work:

- add or refine stakeholder needs,
- verify each need represents a durable user or reviewer concern.

Exit evidence:

- stakeholder need table in `docs/vtrace/MISSION.md`.

Current findings:

- Needs cover Rust maintainers, AI tooling, downstream platforms, assurance
  reviewers, and contract/platform authors.
- The list is sufficient for foundation, but each later wave should add new
  needs only through this stage.
- Stage 1 review passed on 2026-06-01 for the foundation wave.

### Stage 2: Requirements

Allowed work:

- write verifiable requirements,
- assign stable IDs,
- map each requirement to validation or review evidence.

Exit evidence:

- `docs/vtrace/REQUIREMENTS.md`,
- `docs/vtrace/TRACE.md`.

Current findings:

- Stage 2 review passed on 2026-06-01 for the foundation wave.
- Requirements are now split across stage control, neutral descriptors, macros,
  CLI/generators, verification, and validation.
- Later interface work must still refine these requirements before declaring any
  implementation surface contract-stable.

### Stage 3: Concept and architecture

Allowed work:

- define core descriptor concepts,
- define profile and adapter boundaries,
- record non-goals.

Exit evidence:

- `docs/architecture/descriptor-model.md`,
- product-neutrality review in `docs/vtrace/REVIEW.md`.

Current findings:

- A first descriptor model exists.
- Stage 3 review passed on 2026-06-01 for the foundation wave.
- The descriptor model now names the foundation vocabulary, minimum neutral
  descriptor shape, vocabulary rules, and profile boundary.
- Exact trait, macro, CLI, generator, and diagnostic interfaces remain Stage 4
  work.

### Stage 4: Interface and control specs

Allowed work:

- specify derive attributes,
- specify trait and descriptor records,
- specify CLI command contracts,
- specify generator input/output contracts and diagnostics.

Exit evidence:

- interface specs under `docs/architecture/` or `docs/specs/`,
- trace rows from requirements to interfaces.

Current findings:

- Stage 4 review passed on 2026-06-01 for the foundation wave.
- `docs/architecture/interface-control.md` defines the approved Stage 5 slice:
  neutral descriptor records, `RuneContract`, `#[derive(RuneContract)]`,
  bounded `status` CLI behavior, generator/profile boundaries, and diagnostics
  concepts.
- Existing `rune-core`, `rune-derive`, and `rune-cli` code is still scaffold
  until Stage 5 aligns it with the interface-control spec.

### Stage 5: Implementation slices

Allowed work:

- implement only the interface slice approved by Stage 4,
- add focused tests,
- keep generated behavior inspectable.

Exit evidence:

- source code,
- tests,
- CLI output,
- generated artifact samples when applicable.

Current findings:

- Stage 5 review passed on 2026-06-01 for the foundation wave.
- `rune-core` exposes the approved descriptor records and `RuneContract` trait.
- `rune-derive` supports the approved `id`, `version`, and `kind` attributes for
  the first derive slice.
- `rune-cli status` is bounded and names deferred commands honestly.
- No generators, adapters, or broader CLI commands were added.

### Stage 6: Verification

Allowed work:

- run formatting, tests, CLI checks, macro compile checks, and generated artifact
  checks,
- record objective evidence.

Exit evidence:

- `docs/vtrace/VERIFICATION.md`,
- pulse evidence notes,
- validation command output.

Current findings:

- Stage 6 review passed on 2026-06-01 for the foundation wave.
- Formatting, workspace tests, CLI status, and whitespace checks pass.
- Proc-macro compile-pass and compile-fail fixtures exist for the approved
  derive slice.
- Generated artifact checks are not applicable until a generator surface is
  approved by a later stage.

### Stage 7: Validation

Allowed work:

- run representative repo bakeoffs,
- compare RUNE-generated contracts against source-only or prose-only baselines,
- record whether RUNE improves AI comprehension, review, and handoff.

Exit evidence:

- bakeoff scenarios,
- generated artifacts,
- findings and limitations.

Current findings:

- Stage 7 planning completed on 2026-06-01 for the foundation wave.
- Fixture-backed inspection, neutral generation, compatibility checks, and
  annotated-type evidence are now available.
- Wave 4 is open for constrained source/prose-only versus RUNE-evidence
  comparison on the existing annotated `Customer` scenario.
- Broad adoption and arbitrary crate discovery remain deferred.
- Consumer repos remain scenarios only; RUNE core must not encode their
  vocabulary.

### Stage 8: Review and readiness

Allowed work:

- run `.roles` review,
- decide pass, pass-with-risk, block, or loop-back,
- convert findings into follow-up pulses.

Exit evidence:

- `docs/vtrace/REVIEW.md`,
- issue or pulse links for unresolved findings.

Current findings:

- Role files exist.
- Stage 8 review passed with limits on 2026-06-01 for the foundation wave.
- Foundation is ready as a verified scaffold and VTRACE baseline.
- Foundation is not validated for broad adoption; bakeoff execution waits for an
  approved inspection or generator surface.

## Trace links expected

- Mission and stakeholder needs: `docs/vtrace/MISSION.md`.
- Requirements and trace matrix: `docs/vtrace/REQUIREMENTS.md`,
  `docs/vtrace/TRACE.md`.
- Concept and interface designs: `docs/architecture/`.
- Implementation evidence: `crates/`.
- Verification evidence: `docs/vtrace/VERIFICATION.md` and wave pulses.
- Review evidence: `.roles/` and `docs/vtrace/REVIEW.md`.

## Validation command

```powershell
git diff --check
```

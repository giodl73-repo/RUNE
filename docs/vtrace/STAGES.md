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
| 8 | Review and readiness | Decide whether the stage can broaden adoption or must loop back. | Verification and validation evidence exist. | `.roles` review findings are resolved, waived, or converted into follow-up pulses. | pass with role-review hardening |

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
- Mission 2.0 DCRs now add requirements for retained semantic registry evidence
  (`RUNE-REQ-085` through `RUNE-REQ-089`), hardened retained state graph
  validation (`RUNE-REQ-090`), and retained evidence runtime packets
  (`RUNE-REQ-091`), and retained read-first agent protocol requests
  (`RUNE-REQ-092`).
- Later Mission 2.0 lanes still require new requirements/DCRs before declaring
  compatibility, policy, or runtime host surfaces contract-stable.

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
- Mission 2.0 interface docs now exist for semantic registry, state graph,
  evidence runtime packets, agent protocol, compatibility negotiation,
  capability/sensitivity policy, and optional runtime host design.
- DCR-RUNE-003, DCR-RUNE-004, DCR-RUNE-005, and DCR-RUNE-006 have moved
  semantic registry, state graph, evidence runtime packets, and agent protocol
  from interface planning into implemented retained-evidence slices; all later
  lanes remain interface-only until their own implementation DCRs.

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
- V1 implementation later added reviewed fixture-backed inspection, generation,
  profile, adapter, discovery, evidence, registry, and state graph CLI commands.
- DCR-RUNE-003 implements read-only semantic registry validation/inspection.
- DCR-RUNE-004 implements hardened retained state graph validation with
  fail-closed retained evidence, ownership, duplicate graph-id, and live-state
  diagnostics.
- DCR-RUNE-005 implements retained evidence runtime packet validation with
  fail-closed packet identity, family, descriptor ref, severity/status, audit
  decision, registry ref, and evidence ref diagnostics.
- DCR-RUNE-006 implements retained read-first agent protocol validation with
  fail-closed operation, capability, input-ref, mutating-operation, and
  restricted-data diagnostics.
- Runtime host behavior, live state inspection, replay/mutation, Cargo traversal,
  source scraping, plugin discovery, automatic migration, and policy enforcement
  remain blocked.

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
- Verification now covers descriptor/collection/profile/adapter/discovery
  evidence, semantic registry checks/inspection, hardened state graph checks, and
  retained evidence packet checks, and retained agent protocol checks.
- State graph verification includes `RUNE-STATE-001` through `RUNE-STATE-009`
  fixtures and CLI tests.
- Evidence packet verification includes `RUNE-EVIDENCE-001` through
  `RUNE-EVIDENCE-007` validation paths and CLI tests.
- Agent protocol verification includes `RUNE-AGENT-001` through
  `RUNE-AGENT-005` validation paths and CLI tests.

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
- V1 release readiness passed with retained evidence and representative bakeoff
  evidence.
- Mission 2.0 role/panel review identified Wave 43 hardening needs; those are
  now resolved in DCR-RUNE-004 through retained evidence ref, ownership ref, and
  duplicate graph-id validation.
- Broad runtime adoption remains blocked until compatibility, policy, and
  optional runtime host lanes have their own approved DCRs and validation
  packages.

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

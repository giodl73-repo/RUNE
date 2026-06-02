# Games contract candidates

This trace records the games repo review used to steer RUNE beyond trait-only
contracts. The repos were reviewed as scenarios only; no game repo code was
changed.

## Findings

| Repo | High-value RUNE candidates | Contract lane |
|---|---|---|
| COURT | `CourtHost`, `CourtExperience`, `CourtAction`, `CourtSnapshot`, `CourtValidationPacket`, evidence and critique records | host trait plus data/state/action/evidence contracts |
| MUDDLE | `MuddleClient`, `MuddleHost`, `MuddleRoom`, `MuddleCommand`, `MuddleSession`, `MuddleClientSnapshot`, controls, panels, visual nodes | host/client traits plus data/command/state/UI contracts |
| RALLY | `SimulationRun`, `ActorTrace`, `SimulationMetric`, `ComparisonDelta`, `EventLogEntry`, `ValidationReport`, grid/area/vote primitives | simulation data, event, evidence, and validation contracts |
| RACKET | `RacketFramePlan`, `RacketRuntimeConfig`, `RacketRuntimeFrame`, `RacketRuntimeReport`, adapter diagnostics | adapter output and runtime report contracts |
| AMAZE / BANISH / QUEST / TIGRIS | MUDDLE host surfaces, room/exit/command/state structs, scenario harness states | scenario-specific data contracts and host implementations |

## Adoption conclusion

RUNE should not optimize only for trait extraction. The games repos are richer in
data, event, state, packet, report, and simulation contracts than in explicit
trait contracts. The first practical adoption path is:

1. Annotate stable structs/enums that cross repo, engine, AI, or review
   boundaries.
2. Retain descriptor collections and `rune.data_contract_json` profile output.
3. Add host/client trait contracts after the data snapshots and commands are
   stable.
4. Keep game-specific semantics in namespaced extensions or adapters.

## RALLY role review lens

The review followed the RALLY role guidance for shared games validation
infrastructure:

| Role | Result |
|---|---|
| Harness Boundary Engineer | pass: RUNE remains product-neutral; game language belongs in extensions/adapters. |
| Simulation Auditor | pass with follow-up: RALLY run, trace, metric, and report structs are strong contract candidates for deterministic evidence. |
| Consumer Advocate | pass: data-first contracts let AMAZE, QUEST, BANISH, and TIGRIS adopt without losing local workflows. |
| Evidence Packet Reviewer | pass: validation packets and reports map naturally to retained RUNE evidence. |
| Privacy Reviewer | pass: the survey names public contract surfaces only and does not copy private playtest or campaign content. |

## RUNE implications

The review supports `RUNE-REQ-074`: RUNE should expose metadata-driven data
contracts and a retained data-contract profile before broad games adoption. A
later wave can add field-level metadata and trait/function descriptors once the
data-contract path is validated.

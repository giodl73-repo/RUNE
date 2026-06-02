# RUNE documentation

RUNE documentation is organized by adoption intent. Start with the README for the
product overview, then use this map to choose the right depth.

| Surface | Audience | Purpose |
|---|---|---|
| [Concepts](concepts/) | Rust maintainers, AI tooling authors, reviewers | Explain the model: descriptors, evidence, discovery, profiles, and adapters. |
| [How-to guides](how-to/) | Adopters and operators | Complete one concrete task with commands and expected boundaries. |
| [Runbooks](runbooks/) | Operators and release reviewers | Execute ordered validation sequences over retained evidence. |
| [Tutorials](tutorials/) | Hands-on learners | Walk from a first annotated type to retained profile and adapter evidence. |
| [Examples](examples/) | Implementers | Copyable references for the adopter crate and retained JSON outputs. |
| [Traces](traces/) | Reviewers and assurance users | End-to-end walkthroughs that connect commands, evidence, and decisions. |
| [Architecture](architecture/) | Maintainers | Interface-control records and design boundaries. |
| [VTRACE](vtrace/) | Maintainers and reviewers | Requirements, traceability, verification, validation, and review evidence. |
| [Adopter guide](adopter-guide.md) | First adopters | Short v1 path through derive, registry, evidence, profile, and adapter output. |
| [Release readiness](release-readiness.md) | Maintainers | v1 surfaces, compatibility policy, and non-goals. |
| [CORPUS](CORPUS.md) | Documentation maintainers | Surface ownership and update obligations. |

## Recommended paths

| Goal | Read |
|---|---|
| Understand what RUNE adds to Rust | [Concepts: descriptors as contracts](concepts/descriptors-as-contracts.md) |
| Model metadata-rich data, command, event, and state contracts | [Concepts: data contracts](concepts/data-contracts.md) |
| Add RUNE to a crate | [Tutorials: adopter path](tutorials/adopter-path/) |
| Validate the full adopter evidence path | [Runbook: adopter evidence validation](runbooks/adopter-evidence-validation.md) |
| Generate reviewable evidence | [How-to: generate retained evidence](how-to/generate-retained-evidence.md) |
| Use discovery safely | [How-to: use a discovery manifest](how-to/use-discovery-manifest.md) |
| Build a downstream integration | [Concepts: profiles and adapters](concepts/profiles-and-adapters.md) |
| Evaluate game/simulation adoption | [Traces: games contract candidates](traces/games-contract-candidates.md) |

## Current boundary

The current docs describe RUNE v1. They do not promise arbitrary Rust source
scraping, Cargo metadata traversal, executable hooks, automatic publishing, or
product-specific vocabulary in `rune-core`.

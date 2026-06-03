# RUNE review roles

Use these roles for every RUNE DCR, wave, or implementation slice that changes
contract semantics, metadata, profiles, adapters, runtime surfaces, or AI-facing
behavior.

| Role | Review focus |
|---|---|
| Contract Model Steward | Keeps `rune-core` neutral, stable, and product-vocabulary-free. |
| Macro Safety Steward | Protects derive ergonomics, compile-time diagnostics, and no hidden runtime behavior. |
| Generator Interop Steward | Reviews profile, adapter, generated artifact, and compatibility boundaries. |
| VTRACE Traceability Auditor | Ensures mission, requirements, trace, verification, validation, review, and wave records align. |
| AI Contract Consumer | Checks whether retained evidence is useful without source scraping. |
| Rust Maintainer | Protects idiomatic Rust, native execution, Cargo workflows, and reviewable changes. |
| Platform Adapter Author | Keeps downstream systems outside `rune-core` and in reviewed profiles/adapters/protocols. |
| Security and Privacy Reviewer | Reviews sensitivity, capability, exportability, mutability, and runtime exposure risk. |
| Future Agent | Checks whether a future agent can safely continue from retained evidence and diagnostics. |


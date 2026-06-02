# RUNE Generator and Profile Interface

## Engineering decision supported

Define how RUNE descriptors may be transformed into generated artifacts without
letting any target format dictate the neutral core model.

## Boundary

Generators consume RUNE descriptors and emit profile-specific artifacts.
Profiles define the mapping, compatibility rules, unsupported-concept behavior,
and output evidence for a target artifact family.

```text
ContractDescriptor -> DescriptorDocument -> Profile -> GeneratedArtifact
DescriptorCollectionDocument -> Profile -> GeneratedCollectionArtifact
```

The first generator wave should start with a neutral descriptor-output profile,
not a third-party or product-specific target.

## Generator input contract

Generators may consume only reviewed RUNE descriptor data:

| Input | Requirement |
|---|---|
| `id` | Must be preserved in durable output. |
| `version` | Must be preserved in durable output. |
| `kind` | Must be supported or diagnosed. |
| `rust_type` | May be emitted for traceability. |
| `fields` | Must be supported or diagnosed. |
| `invariants` | Must be supported or diagnosed. |
| `trace_links` | Must be supported or diagnosed. |
| `extensions` | Must be namespaced and either supported or diagnosed. |

CLI fixture-backed generation must parse descriptor input through
`rune-core::DescriptorDraft` and validate it into
`rune-core::DescriptorDocument` before profile generation. The CLI must not
maintain a separate descriptor JSON model.

## Profile contract

Each profile must declare:

| Field | Meaning |
|---|---|
| `profile_id` | Stable profile identity. |
| `profile_version` | Profile compatibility boundary. |
| `input_descriptor_versions` | Descriptor versions the profile accepts. |
| `supported_kinds` | Descriptor kinds the profile can map. |
| `supported_concepts` | Descriptor concept families the profile can preserve. |
| `unsupported_policy` | Whether unsupported concepts are errors or warnings. |
| `output_artifact_kind` | Generated artifact family. |

The approved profile catalog is inspectable through:

```powershell
cargo run -p rune-cli -- profile list
```

The command is read-only and must not imply that unlisted external profiles are
supported.

Approved profile metadata is owned by `rune-core::ProfileCatalog` and
`rune-core::ProfileDescriptor`. CLI commands and future generator consumers must
use that shared profile catalog rather than duplicating profile ids, versions,
supported kinds, or output artifact kinds.

Descriptor/profile compatibility can be checked without emitting generated
artifacts through:

```powershell
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture <path>
```

The check command is read-only. It validates descriptor shape and selected-profile
compatibility, then emits a compact compatibility report.

Collection/profile compatibility can be checked without emitting generated
artifacts through:

```powershell
cargo run -p rune-cli -- check-collection --profile rune.neutral_descriptor_json --fixture <path>
```

The collection check command is read-only and fixture-backed. It validates the
collection document, validates each descriptor against the selected profile, and
then emits a compact collection compatibility report.

Collection/profile generation can emit a retained neutral artifact through:

```powershell
cargo run -p rune-cli -- generate-collection --profile rune.neutral_descriptor_json --fixture <path>
```

The collection generation command is fixture-backed. It validates the collection
document, validates each descriptor against the selected profile, and then emits
a generated collection artifact that preserves profile metadata and the validated
collection document.

## Diagnostics

Generator diagnostics must report unsupported or lossy mappings. Initial codes:

| Code | Severity | Meaning |
|---|---|---|
| `RUNE-GEN-001` | error | Descriptor identity is missing from generator input. |
| `RUNE-GEN-002` | error | Descriptor version is missing from generator input. |
| `RUNE-GEN-003` | error | Profile does not support descriptor kind. |
| `RUNE-GEN-004` | warning/error | Profile cannot represent an invariant. |
| `RUNE-GEN-005` | warning/error | Profile cannot represent a trace link. |
| `RUNE-GEN-006` | warning/error | Profile cannot represent a namespaced extension. |
| `RUNE-GEN-007` | error | Profile does not support descriptor version. |
| `RUNE-COLL-GEN-001` | error | Descriptor collection identity is missing from generator input. |
| `RUNE-COLL-GEN-002` | error | Descriptor collection version is missing from generator input. |
| `RUNE-COLL-GEN-003` | error | Collection generator profile or descriptor kind is unsupported. |
| `RUNE-COLL-GEN-004` | error | Collection generator profile cannot represent an invariant. |
| `RUNE-COLL-GEN-005` | error | Collection generator profile cannot represent a trace link. |
| `RUNE-COLL-GEN-006` | error | Collection generator profile cannot represent a namespaced extension. |
| `RUNE-COLL-GEN-007` | error | Collection generator profile does not support descriptor version. |
| `RUNE-COLL-GEN-008` | error | Descriptor collection contains duplicate descriptor ids. |
| `RUNE-COLL-GEN-900` | error | Collection generator input or artifact cannot be read, parsed, or serialized. |
| `RUNE-CHECK-001` | error | Descriptor identity is missing from check input. |
| `RUNE-CHECK-002` | error | Descriptor version is missing from check input. |
| `RUNE-CHECK-003` | error | Check profile or descriptor kind is unsupported. |
| `RUNE-CHECK-004` | error | Check profile cannot represent an invariant. |
| `RUNE-CHECK-005` | error | Check profile cannot represent a trace link. |
| `RUNE-CHECK-006` | error | Check profile cannot represent a namespaced extension. |
| `RUNE-CHECK-007` | error | Check profile does not support descriptor version. |
| `RUNE-CHECK-900` | error | Check input or report cannot be read, parsed, or serialized. |
| `RUNE-COLL-CHECK-001` | error | Descriptor collection identity is missing from check input. |
| `RUNE-COLL-CHECK-002` | error | Descriptor collection version is missing from check input. |
| `RUNE-COLL-CHECK-003` | error | Collection check profile or descriptor kind is unsupported. |
| `RUNE-COLL-CHECK-004` | error | Collection check profile cannot represent an invariant. |
| `RUNE-COLL-CHECK-005` | error | Collection check profile cannot represent a trace link. |
| `RUNE-COLL-CHECK-006` | error | Collection check profile cannot represent a namespaced extension. |
| `RUNE-COLL-CHECK-007` | error | Collection check profile does not support descriptor version. |
| `RUNE-COLL-CHECK-008` | error | Descriptor collection contains duplicate descriptor ids. |
| `RUNE-COLL-CHECK-900` | error | Collection check input or report cannot be read, parsed, or serialized. |
| `RUNE-PROF-900` | error | Profile catalog output cannot be serialized. |

## First profile candidate

The first implementation candidate should be a neutral descriptor-output profile:

```text
profile_id: rune.neutral_descriptor_json
profile_version: v0
input_descriptor_versions: v0
supported_kinds: entity, event, command, state, artifact, source, evidence
supported_concepts: fields, invariants, trace_links, extensions
unsupported_policy: error
output_artifact_kind: rune.descriptor.json
```

This profile is intentionally close to RUNE descriptors. Its purpose is to test
profile mechanics and retained generated artifacts before targeting external
formats.

The neutral descriptor-output profile accepts descriptor version `v0` only and
must fail closed for descriptor kinds that are valid RUNE escape hatches but not
listed in the profile catalog, including `other`.

The neutral descriptor-output profile declares support for all first-slice
descriptor concept families. Future profiles that cannot preserve invariants,
trace links, or extensions must fail closed with `RUNE-GEN-004`,
`RUNE-GEN-005`, or `RUNE-GEN-006` before generation; the read-only check command
uses corresponding `RUNE-CHECK-004`, `RUNE-CHECK-005`, and `RUNE-CHECK-006`
codes.

## Non-goals

- Do not implement JSON Schema, CSDL, OpenAPI, AgentMap, or product-specific
  adapters in the first generator slice.
- Do not treat generated artifacts as more authoritative than descriptors.
- Do not silently drop invariants, trace links, extensions, identity, or version.
- Do not discover or build collection inputs from crates; collection checks and
  collection generation remain fixture-backed until a discovery interface is
  reviewed.
- Do not run broad adoption bakeoffs before generated artifact verification
  exists.

## Validation command

```powershell
git diff --check
cargo run -p rune-cli -- profile list
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- check-collection --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.json
cargo run -p rune-cli -- generate-collection --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\known_contract_descriptor_collection.json
```

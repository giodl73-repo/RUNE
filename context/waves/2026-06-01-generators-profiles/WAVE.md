# Wave: Generators and profiles

## Goal

Define generator and profile interfaces after the neutral inspection surface has
been validated, while keeping product-specific adapters and broad adoption out
of scope.

## Engineering decision supported

RUNE may add generation only through neutral, reviewed profiles that preserve
descriptor identity/versioning and report unsupported concepts. No target format
may become the hidden source of truth.

## Trace links expected

- Mission need: Rust descriptors should feed multiple downstream artifacts.
- Requirement: profiles preserve descriptor identity and report unsupported
  concepts.
- Design: generator/profile interface spec.
- Implementation: future minimal neutral descriptor-output profile.
- Verification: generated artifact checks.
- Validation: bakeoff comparing inspected descriptors and generated artifacts.

## Pulses

| Pulse | Title | Status |
|---|---|---|
| 01 | Generator/profile interface boundary | complete |
| 02 | Minimal neutral descriptor-output profile | complete |
| 03 | Generated artifact verification | complete |
| 04 | Annotated Rust type bakeoff | complete |
| 05 | Derive-to-document bridge | complete |
| 06 | Read-only profile catalog | complete |
| 07 | Shared descriptor-document boundary | complete |
| 08 | Shared profile catalog boundary | complete |
| 09 | Profile compatibility enforcement | complete |
| 10 | Read-only compatibility check | complete |
| 11 | Profile concept compatibility | complete |

## Validation

```powershell
git diff --check
cargo test --workspace
cargo run -p rune-cli -- profile list
cargo run -p rune-cli -- check --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
cargo run -p rune-cli -- generate --profile rune.neutral_descriptor_json --fixture crates\rune-cli\tests\fixtures\annotated_customer_descriptor.json
```

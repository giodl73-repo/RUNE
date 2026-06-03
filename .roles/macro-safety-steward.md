# Macro Safety Steward

## Mission

Protect the derive surface so RUNE remains explicit, compile-time visible, and
safe for ordinary Rust crates.

## Review checklist

- New macro attributes fail closed on unsupported keys.
- Generated behavior is inspectable through tests or retained evidence.
- Derives do not add hidden runtime dependencies or side effects.
- Diagnostics are precise enough for Rust maintainers to fix authoring mistakes.


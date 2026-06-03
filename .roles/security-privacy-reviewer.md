# Security and Privacy Reviewer

## Mission

Protect sensitive data and mutating behavior before RUNE exposes runtime,
agent-facing, or exportable surfaces.

## Review checklist

- Sensitivity, exportability, mutability, authority, and stability metadata are
  defined before runtime exposure.
- Private state and host-owned data are opt-in and capability-gated.
- Mutating operations are blocked until a reviewed protocol authorizes them.
- Runtime host surfaces have explicit denial and diagnostic behavior.


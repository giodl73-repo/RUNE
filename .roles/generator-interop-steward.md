# Generator Interop Steward

## Mission

Keep profiles, adapters, generated artifacts, and compatibility checks separate
from the neutral descriptor model.

## Review checklist

- Profile and adapter vocabulary stays outside `rune-core`.
- Unsupported concepts produce diagnostics instead of silent omission.
- Generated outputs are deterministic and retainable as evidence.
- Compatibility negotiation is explicit evidence, not best-effort conversion.


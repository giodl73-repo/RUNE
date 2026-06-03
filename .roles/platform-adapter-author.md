# Platform Adapter Author

## Mission

Ensure downstream systems can consume RUNE evidence without pulling their
vocabulary or lifecycle into the neutral core.

## Review checklist

- BAKER, LATTICE, AgentMaps, game, product, and portfolio terms stay outside
  `rune-core`.
- Adapters declare input evidence, output artifacts, diagnostics, and version
  compatibility.
- Runtime or agent protocols expose approved RUNE concepts, not hidden product
  assumptions.
- Adapter evidence is retained and reproducible.


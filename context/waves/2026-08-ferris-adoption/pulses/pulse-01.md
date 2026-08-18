# Pulse 01: Exact Procedural-Macro Compatibility Proof

Status: Complete
Implementation authority: Bounded to this pulse
Successor authority: None

## Implemented slice

- exact public FERRIS commit
  `35f3518b6597acde34641a4f55e5111405334e70`;
- consumer contract `rune.ferris-consumer-contract/v1`;
- exact result schema, plan schema, and command version without copied schemas;
- isolated temporary fetch, checkout, and build;
- direct exact `cargo-ferris` invocation with Cargo's injected `ferris` token;
- `rune-derive` anchor plus `rune-adopter` and `rune-shape-calculator` reverse
  dependencies;
- six-package repository fallback;
- preservation checks for RUNE's four documented owner commands;
- immutable-event Ubuntu consumer workflow; and
- explicit limitations, migration, rollback, removal, and non-goals.

## Measured result

Both exact-pin modes passed:

```console
python tools/ferris-contract/check.py --ferris-source C:\src\FERRIS
python tools/ferris-contract/check.py
```

The fetched mode also passed with `CARGO_ALIAS_FERRIS=metadata`, proving that
ambient Cargo aliases cannot substitute another command. RUNE formatting,
full workspace tests including procedural-macro compile tests, the RUNE status
command, Python compilation, and diff hygiene passed.

The lifecycle was exercised without retaining a contract change:

1. change only the pin from FERRIS `35f3518b6597acde34641a4f55e5111405334e70`
   to `5cd1aa99727a23de25c79d067090e7444bdfb5e8`;
2. run the checker against a freshly fetched exact checkout;
3. restore `35f3518b6597acde34641a4f55e5111405334e70`; and
4. rerun against the clean local exact checkout.

Both migration and rollback passed.

## Boundaries retained

FERRIS observes only Cargo-declared package relationships and proposes
non-executable `check` and `test` activity families. It does not understand
procedural-macro expansion, individual `trybuild` cases, features, targets,
doctests, runtime status, or repository policy. RUNE's four README commands
remain the required validation contract.

## Decision

Complete the implementation pulse without a successor. The consumer workflow
proved the immutable pull-request event revision on Ubuntu in run
[`32182071143`](https://github.com/giodl73-repo/RUNE/actions/runs/32182071143).

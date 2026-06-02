# Adopter workflow example

The primary example crate is:

```text
examples\rune-adopter
```

It demonstrates:

1. annotated Rust contracts,
2. a crate-owned `RUNE_CONTRACTS` registry,
3. deterministic descriptor collection evidence,
4. duplicate descriptor id diagnostics,
5. documentation packet profile evidence,
6. review packet adapter evidence.

Run it with:

```powershell
cargo test -p rune-adopter
```

Use the example as a template for adopter-owned tests. Do not make adopter tests
mutate retained fixtures during ordinary validation.

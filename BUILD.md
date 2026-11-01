# Build the RUNE contract workbench

Use Rust 1.95.0 with the wasm32-unknown-unknown target, wasm-bindgen-cli 0.2.127,
Python 3.12 and Node 22. The consumer has its own committed lockfile and does not
change the workspace dependency graph. Its path dependencies are inside this repo.

```
cargo +1.95.0 test --locked
cargo +1.95.0 test --locked --manifest-path web/adapter/Cargo.toml --all-features
npm ci
npx playwright install chromium
python -B tools/build-pages.py
npm run test:pages
python -B tools/serve-pages.py
```

Open http://127.0.0.1:4177/RUNE/. The builder invokes native Rust for every parity
reference, creates a full source ZIP, dependency/license closure, standalone source
views, provenance, and a bounded static site. Dirty previews are labeled; release
artifacts must come from a clean revision. Deployment serves the tested artifact.
Set RUNE_BASE_URL to test an existing deployment without a local server.

Native CLI: `rune-replay REQUEST.json` accepts an envelope with kind, profile and
input_json (the original contract JSON string); `--examples` emits authored neutral
examples. Input is capped at 128 KiB, envelope at 256 KiB and collections at 128
members. Core serde recursion limits remain active. Browser worker startup,
computation and file reads have separate deadlines and stale-result fences.

Metadata checks do not execute constraints, invariant text, derive macros or a
runtime host. Documentation/review summaries omit information by design; exact
input and recognized normalized metadata remain separately inspectable. Unknown
fields are listed by serde paths and are not validated. Duplicate known keys are
rejected; unknown duplicate keys can be ignored once. Files must be valid UTF-8;
original imports retain CRLF and large numeric lexemes as text until edited.

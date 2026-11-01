# Pulse 01: existing contracts in WebAssembly

Status: implementation authorized by stages 5–6; readiness pending evidence.

1. Preserve public main and prior tracked PITFALL doctrine.
2. Add an isolated, unpublished web consumer of rune-core and rune-adapters.
3. Route supplied JSON through existing native draft validation and profile
   compatibility; revalidate collection members individually before generation.
4. Keep exact input, disclose unknown fields, bound input and collection size,
   and render all submitted strings as text.
5. Publish inspectable source, WASM, dependency notices and provenance.

Acceptance IDs: WEB-01 own descriptor/collection task; WEB-02 exact input and
native artifact exports; WEB-03 fail closed on invalid/unsupported metadata;
WEB-04 unknown-field disclosure and resource limits; WEB-05 usable keyboard,
mobile, download-failure recovery; WEB-06 clean source/artifact custody.

Validation: workspace fmt/test/build/doc, isolated consumer fmt/test/Clippy on
host and wasm32, native/WASM exact-response parity, real Chromium task/download
and recovery checks, source/license inspection, AI-simulated role review, and
exact-main hosted/live evidence before TRACKER snapshot.

## Local verification outcome

Implemented the existing native APIs in an isolated consumer. Seven consumer
tests, 56 exact native/WASM responses and 21 Chromium cases pass locally, with
desktop/mobile screenshots inspected. Existing workspace tests/build/doc/fmt and
consumer host/wasm Clippy passed. Core/derive/adapters, workspace manifest/lock and
MIT license are unchanged from integrated baseline. See VTRACE verification and
review records. Public exact-main deployment qualification follows in TRACKER.

Workspace Clippy under Rust 1.95 requires scoped allowances for two existing
`collapsible_if` findings in core and repeated `namespace` macro metadata in the
shape example (`duplicated_attributes`). Consumer host/wasm linting remains
`-D warnings` without allowances. No core/example source changed for these lints.

# Phase 08 Research: API, FFI, and Migration Readiness

## Research Question

What minimum concrete outputs are required to close `API-01`, `API-02`, and
`API-03` after Phase 7's Rust API absorption?

## Inputs Reviewed

- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md`
- `.planning/phases/07-capability-absorption-pipeline/07-CAPABILITY-DESIGN.md`
- `.planning/phases/07-capability-absorption-pipeline/07-VERIFICATION.md`
- `.planning/codebase/INTEGRATIONS.md`
- `README.md`
- `CHANGELOG.md`
- `cavalier_contours_ffi/README.md`

## Findings

### Phase 7 Delta Is Rust-API Only

Phase 7 introduced `rect_clip` and `rect_clip_opt` on `PlineSource` and did not
change C ABI. Phase 8 therefore needs explicit compatibility notes, not forced
FFI growth.

### API-02 Is Conditional and Must Be Explicit

`API-02` requires ABI tests and header regeneration only when FFI changes exist.
For this phase, "no FFI change" must be proven and recorded, including that
`cavalier_contours_ffi.h` remains untouched.

### Migration Notes Are Currently Missing

Repository docs explain the rewrite and capabilities, but do not provide a
direct migration path for old C++ users. A focused migration document should
cover:

- C++ concept to Rust API mapping.
- C FFI use cases and constraints.
- behavior/limitations that matter during migration.

## Recommended Plan Shape

- `08-01`: compatibility audit artifact.
- `08-02`: compatibility notes in release/docs surfaces; verify no FFI drift.
- `08-03`: migration notes + full verification + phase completion.

## Verification Architecture

- Targeted API test: `cargo test -p cavalier_contours --test test_pline_boolean rect_clip -- --nocapture`
- Workspace gates: `cargo test --workspace`, `cargo fmt --all --check`,
  `cargo clippy --all-targets -- -D warnings`.
- Drift and health: `git diff --check`, `gsd-sdk query state.validate`,
  `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`.

## RESEARCH COMPLETE

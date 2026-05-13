# Phase 92: C-API Logic Alignment Map

This map captures next steps after self-intersect/contains null-result contract
symmetry coverage hardening.

## Deepening Outcome

- Self-intersect null-input API contracts now include explicit default-options
  path assertions proving return behavior and output-flag stability match
  explicit-options expectations.
- Contains explicit-options invalid-input contracts now include null-result
  pointer symmetry assertions for both null-`pline1` and null-`pline2`.
- Null-result and invalid-input behavior now has stronger regression resistance
  against asymmetry across equivalent boundary paths.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI APIs with `Specific Error Codes` for missing failure-path output stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Extend default/options-path invariance checks to remaining invalid-input branches that currently validate only one path | `cavalier_contours_ffi/tests/test_pline.rs` | Keep deterministic API-level assertions only. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`

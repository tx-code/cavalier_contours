# Phase 71: C-API Logic Alignment Map

This map captures next steps after default coincident matrix source-mapping
guard convergence.

## Deepening Outcome

- Explicit default-path parity for `coincident_case1` and `coincident_case2`
  now fails fast on source-backed case-count/name/operation drift.
- Shared coincident matrix helper and explicit default matrices reuse one
  mapping guard path and consistent diagnostics.
- Existing output-property assertions remain unchanged; this phase is guard
  hardening only.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Expand explicit default-path source-backed matrices with additional old C++ behavior groups | `cavalier_contours_ffi/tests/test_pline.rs` | Add only cases with deterministic expected-output provenance from old C++ tests. |
| P1 | Keep mapping guard messages stable during further matrix deepening | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve case-count/name/operation drift diagnostics as canonical guard surface. |
| P2 | Trigger drift triage template when first real source update mismatch appears | `.planning/phases/*` | Instantiate drift report only on real source delta, not synthetic guard failures. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`

# Phase 25: C-API Logic Alignment Map

This map defines next deep C-API parity targets after function-surface matrix
closure.

## Alignment Completed in Phase 25

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| generated circle function matrix (metrics/winding) | direct C-API matrix parity test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| generated half-circle function matrix (metrics/winding) | direct C-API matrix parity test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| closest-point matrix expectations | C-API surface gap classification | `.planning/phases/25-capi-function-surface-matrix-parity/*.md` + `cavalier_contours_ffi/src/lib.rs` | complete (not-comparable classification) |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API optioned-boolean/offset parity edge cases | `cavalier_contours_ffi/tests/test_pline.rs` | Use source-backed options behavior only; avoid option combinations without old C++ expectation anchors. |
| P1 | C-API broadened coincident no-modify matrices | `cavalier_contours_ffi/tests/test_pline.rs` | Extend no-modify checks to heavier coincident matrices after options-path edge behavior is stable. |
| P2 | C-API closest-point parity bridge | `cavalier_contours_ffi/src/lib.rs` + `cavalier_contours_ffi/tests/test_pline.rs` | Requires explicit C closest-point API design/approval before parity import. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`

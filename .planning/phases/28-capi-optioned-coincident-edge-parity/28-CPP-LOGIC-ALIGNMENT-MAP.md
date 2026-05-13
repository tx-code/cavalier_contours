# Phase 28: C-API Logic Alignment Map

This map defines next deep C-API parity targets after optioned coincident edge
parity closure.

## Alignment Completed in Phase 28

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| coincident case1 intersect collapsed-area edge | optioned C-API collapsed filter parity check | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| coincident case1/case2 options-path no-modify matrices | options-path no-modify matrix parity checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| options-path exclusion direction invariants (`A-B`,`B-A`) | explicit options-path no-modify checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API closest-point parity bridge decision | `cavalier_contours_ffi/src/lib.rs` + `cavalier_contours_ffi/tests/test_pline.rs` | Requires explicit decision to introduce closest-point C-API before importing parity cases. |
| P1 | C-API broadened function-surface options-path parity | `cavalier_contours_ffi/tests/test_pline.rs` | Extend only for function surfaces that already have stable source-backed default-path parity. |
| P2 | C-API coincident matrix stress slices | `cavalier_contours_ffi/tests/test_pline.rs` | Keep source-backed inputs and avoid synthetic stress fixtures unless gap evidence demands it. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`

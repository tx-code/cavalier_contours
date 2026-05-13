# Phase 29: C-API Logic Alignment Map

This map defines next deep C-API parity targets after optioned coincident
output parity closure.

## Alignment Completed in Phase 29

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| coincident case1/case2 output parity under options path | default-vs-options output parity matrix check | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| exclusion direction output parity (`A-B`,`B-A`) | explicit options output parity checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API closest-point parity bridge decision | `cavalier_contours_ffi/src/lib.rs` + `cavalier_contours_ffi/tests/test_pline.rs` | Requires explicit decision to introduce closest-point C-API before importing parity matrices. |
| P1 | C-API function-surface parity gap closure report | `.planning/phases/*` + `cavalier_contours_ffi/src/lib.rs` | Keep not-comparable classification explicit until closest-point surface exists. |
| P2 | C-API coincident matrix stress-slice follow-ups | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-backed stress variants or explicitly justified derived variants. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`

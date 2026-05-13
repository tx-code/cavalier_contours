# Phase 27: C-API Logic Alignment Map

This map defines next deep C-API parity targets after coincident no-modify
matrix expansion.

## Alignment Completed in Phase 27

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| coincident case1 no-modify matrix | direct C-API no-modify matrix parity test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| coincident case2 no-modify matrix | direct C-API no-modify matrix parity test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| exclusion direction no-modify variants (`A-B`,`B-A`) | explicit operation matrix no-modify checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API optioned coincident collapsed-area edge behavior | `cavalier_contours_ffi/tests/test_pline.rs` | Keep source-backed coincident case1 intersect anchors and avoid synthetic thresholds without C++ traceability. |
| P1 | C-API coincident options-path no-modify checks | `cavalier_contours_ffi/tests/test_pline.rs` | Extend options-path no-modify checks only on already stabilized coincident matrices. |
| P2 | C-API closest-point parity bridge | `cavalier_contours_ffi/src/lib.rs` + `cavalier_contours_ffi/tests/test_pline.rs` | Requires explicit closest-point C-API introduction and approval before parity import. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`

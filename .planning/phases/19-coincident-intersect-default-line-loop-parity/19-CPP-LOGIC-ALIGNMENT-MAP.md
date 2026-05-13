# Phase 19: C++ Logic Alignment Map

This map defines next deep parity targets after default-path coincident
intersect closure.

## Alignment Completed in Phase 19

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| `coincident_case1_intersect` default empty expectation | line-only 2-vertex loop pruning in stitch stage | `cavalier_contours/src/polyline/internal/pline_boolean.rs` | complete |
| default-path classification update | remove intentional-divergence branch in executable parity test | `cavalier_contours/tests/test_cpp_combine_parity.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Additional C++ C-API pline parity mapping | `cavalier_contours_ffi/tests/test_pline.rs` and parity companions | Expand no-Clipper parity coverage across C-API surfaces now that coincident default-path gap is closed. |
| P1 | Degenerate boolean case expansion around line/arc mixed loops | `cavalier_contours/tests/test_pline_boolean.rs` | Add focused cases only when they improve source-traceable parity coverage and avoid synthetic noise. |
| P2 | Coincident/open-boundary parity probes from old C++ combine suites | `cavalier_contours/tests/test_cpp_combine_parity.rs` | Keep scope on old C++ reproducible cases; defer Clipper-backed comparisons. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust core:
  - `cavalier_contours/src/polyline/internal/pline_boolean.rs`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_combine_parity.rs`
  - `cavalier_contours/tests/test_pline_boolean.rs`

# Phase 17: C++ Logic Alignment Map

This map defines next deep parity targets after coincident combine matrix
closure.

## Alignment Completed in Phase 17

| C++ source | Rust target | Module/File | Status |
|------------|-------------|-------------|--------|
| `TEST_cavc_combine_plines.cpp::createCoincidentCases` | executable coincident combine matrix parity tests | `cavalier_contours/tests/test_cpp_combine_parity.rs` | complete |
| `coincident_case1_intersect` empty expectation | explicit divergence classification with bounded sliver behavior check | `cavalier_contours/tests/test_cpp_combine_parity.rs`, `17-CPP-COINCIDENT-COMBINE-PARITY.md` | complete |

## Next Alignment Targets (No Clipper)

| Priority | C++ source target | Rust parity file/module | Why next |
|----------|-------------------|--------------------------|----------|
| P1 | `coincident_case1_intersect` sliver behavior root-cause and elimination feasibility | `cavalier_contours/src/polyline/internal/pline_boolean.rs`, `cavalier_contours/src/polyline/internal/pline_intersects.rs`, `cavalier_contours/tests/test_cpp_combine_parity.rs` | Determine whether divergence can be safely reduced from intentional to parity by suppressing zero-area slivers. |
| P1 | Deferred intersection standalone expectation import path from Phase 09 | `cavalier_contours/tests/test_cpp_offset_parity.rs`, `cavalier_contours/src/polyline/internal/pline_intersects.rs` | Close the remaining direct intersection expectation gap with executable case-to-expected checks when source expectations are available. |
| P2 | Additional C++ C-API pline suite parity candidates (`TEST_cavc_pline.cpp`) | `cavalier_contours_ffi/tests/test_pline.rs`, `cavalier_contours/tests/*` | Extend deep parity over low-level C API semantics where API mapping is direct. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_combine_parity.rs`
  - `cavalier_contours/tests/test_cpp_offset_parity.rs`
- Rust geometry core:
  - `cavalier_contours/src/polyline/internal/pline_boolean.rs`
  - `cavalier_contours/src/polyline/internal/pline_intersects.rs`


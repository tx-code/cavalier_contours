# Phase 15: C++ Logic Alignment Map

This map defines the next deep parity targets after generated half-circle
offset/collapse matrix closure.

## Alignment Completed in Phase 15

| C++ source | Rust target | Module/File | Status |
|------------|-------------|-------------|--------|
| `TEST_cavc_pline_function.cpp::addHalfCircleCases` (`parallel_offset` + `collapsedOffsetDeltas`) | full generated matrix parity tests for offset and collapse | `cavalier_contours/tests/test_cpp_pline_function_parity.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | C++ source target | Rust parity file/module | Why next |
|----------|-------------------|--------------------------|----------|
| P1 | Additional closest-point tie/index cases for generated non-axis-aligned and reverse variants | `cavalier_contours/tests/test_cpp_pline_function_parity.rs`, `cavalier_contours/src/polyline/traits.rs` | Stress-tests tie-break determinism after strict-index fix. |
| P1 | Remaining C++ function-level suites adjacent to `pline_function` with executable parity potential | `cavalier_contours/tests/*`, mapped Rust modules per suite | Extends parity from current generated matrices into adjacent behavior surfaces. |
| P2 | Selected lower-level segment/offset edge cases not yet represented in generated matrix corpus | `cavalier_contours/src/polyline/pline_seg.rs`, `cavalier_contours/src/polyline/internal/pline_offset.rs` + targeted tests | Captures algorithm edge behavior beyond matrix formulas. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- Rust geometry core:
  - `cavalier_contours/src/polyline/traits.rs`
  - `cavalier_contours/src/polyline/internal/pline_offset.rs`
  - `cavalier_contours/src/polyline/pline_seg.rs`

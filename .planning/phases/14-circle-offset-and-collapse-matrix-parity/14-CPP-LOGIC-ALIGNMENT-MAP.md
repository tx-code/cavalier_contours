# Phase 14: C++ Logic Alignment Map

This map defines the next deep parity targets after generated circle
offset/collapse matrix closure.

## Alignment Completed in Phase 14

| C++ source | Rust target | Module/File | Status |
|------------|-------------|-------------|--------|
| `TEST_cavc_pline_function.cpp::addCircleCases` (`parallel_offset` + `collapsedOffsetDeltas`) | full generated matrix parity tests for offset and collapse | `cavalier_contours/tests/test_cpp_pline_function_parity.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | C++ source target | Rust parity file/module | Why next |
|----------|-------------------|--------------------------|----------|
| P1 | `TEST_cavc_pline_function.cpp::addHalfCircleCases` expected offset vertex matrices (open/closed, x/y, cw/ccw, centers) | `cavalier_contours/tests/test_cpp_pline_function_parity.rs`, `cavalier_contours/tests/test_utils/*` | Extends parity from scalar/function-level to detailed topology on non-full-circle cases. |
| P1 | `TEST_cavc_pline_function.cpp::addHalfCircleCases` collapsed offset thresholds | `cavalier_contours/tests/test_cpp_pline_function_parity.rs` | Captures collapse behavior parity under open/closed and direction variants. |
| P2 | Additional closest-point tie and index behavior around non-axis-aligned generated cases | `cavalier_contours/src/polyline/traits.rs`, targeted parity tests | Confirms no regressions after strict-index tie-break introduction. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- Rust geometry core:
  - `cavalier_contours/src/polyline/traits.rs`
  - `cavalier_contours/src/polyline/internal/pline_offset.rs`
- Shared test utilities:
  - `cavalier_contours/tests/test_utils/pline_test_properties.rs`

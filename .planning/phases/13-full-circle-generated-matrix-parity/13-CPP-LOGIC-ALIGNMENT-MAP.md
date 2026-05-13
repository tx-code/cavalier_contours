# Phase 13: C++ Logic Alignment Map

This map defines the next deep parity targets after full generated circle
matrix closure.

## Alignment Completed in Phase 13

| C++ source | Rust target | Module/File | Status |
|------------|-------------|-------------|--------|
| `TEST_cavc_pline_function.cpp::addCircleCases` (generated matrix: metrics/winding/closest) | full generated matrix parity tests | `cavalier_contours/tests/test_cpp_pline_function_parity.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | C++ source target | Rust parity file/module | Why next |
|----------|-------------------|--------------------------|----------|
| P1 | `TEST_cavc_pline_function.cpp::cavc_parallel_offset` expected offset vertex matrix for circle/half-circle generated cases | `cavalier_contours/tests/test_cpp_pline_function_parity.rs`, `cavalier_contours/tests/test_utils/*` | Deepens parity into shape topology and bulge-level output, beyond scalar properties. |
| P1 | `TEST_cavc_pline_function.cpp::collapsedOffsetDeltas` matrix-wide collapse thresholds | `cavalier_contours/tests/test_cpp_pline_function_parity.rs` | Verifies collapse behavior parity under direction/closure variants. |
| P2 | Additional closest-point tie intersections on not-axis-aligned variants | `cavalier_contours/src/polyline/traits.rs`, targeted parity tests | Confirms tie-break robustness under broader generated geometry. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- Rust geometry core:
  - `cavalier_contours/src/polyline/traits.rs`
  - `cavalier_contours/src/polyline/pline_seg.rs`
- Shared test utilities:
  - `cavalier_contours/tests/test_utils/pline_test_properties.rs`

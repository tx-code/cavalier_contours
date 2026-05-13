# Phase 12: C++ Logic Alignment Map

This map lists the next high-value C++ function-level alignment targets after
Phase 12.

## Alignment Completed in Phase 12

| C++ source | Rust target | Module/File | Status |
|------------|-------------|-------------|--------|
| `TEST_cavc_pline_function.cpp::addHalfCircleCases` (generated matrix: area/path/extents/winding/closest) | generated matrix parity + strict closest index checks | `cavalier_contours/tests/test_cpp_pline_function_parity.rs` | complete |
| `cavc_get_closest_point` tie behavior from half-circle endpoint cases | deterministic tie-break for closest-point segment index | `cavalier_contours/src/polyline/traits.rs` (`PlineSource::closest_point`) | fixed |

## Next Alignment Targets (No Clipper)

| Priority | C++ source target | Rust parity file/module | Why next |
|----------|-------------------|--------------------------|----------|
| P1 | `TEST_cavc_pline_function.cpp::addCircleCases` full generated matrix (all centers, alignments, reverse variants) | `cavalier_contours/tests/test_cpp_pline_function_parity.rs` | Completes function-level generated matrix parity beyond bounded subsets. |
| P1 | `TEST_cavc_pline_function.cpp::cavc_parallel_offset` full expected vertex checks for half-circle/circle matrices | `cavalier_contours/tests/test_cpp_pline_function_parity.rs`, `cavalier_contours/tests/test_utils/*` | Deepens parity into offset topology and bulge-specific expectations. |
| P2 | `TEST_cavc_pline_function.cpp::collapsedOffsetDeltas` matrix-wide collapse behavior | `cavalier_contours/tests/test_cpp_pline_function_parity.rs` | Captures collapse thresholds and directional behavior against old C++. |
| P2 | closest-point tie behavior on other generated vertex intersections | `cavalier_contours/src/polyline/traits.rs`, targeted parity tests | Verifies tie-break stability after Phase 12 fix. |

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

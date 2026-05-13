# Phase 16: C++ Logic Alignment Map

This map defines next deep parity targets after expanded C++ offset matrix
closure.

## Alignment Completed in Phase 16

| C++ source | Rust target | Module/File | Status |
|------------|-------------|-------------|--------|
| `TEST_cavc_parallel_offset.cpp::createSimpleCases/createSpecificCases` | executable offset property parity matrix tests | `cavalier_contours/tests/test_cpp_offset_parity.rs` | complete |
| `TEST_cavc_parallel_offset.cpp::reversed_parallel_offset_test` | reverse + negated-delta parity checks with area sign inversion | `cavalier_contours/tests/test_cpp_offset_parity.rs` | complete |
| `TEST_cavc_parallel_offset.cpp::parallel_offset_does_not_modify_input_test` | input immutability parity check | `cavalier_contours/tests/test_cpp_offset_parity.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | C++ source target | Rust parity file/module | Why next |
|----------|-------------------|--------------------------|----------|
| P1 | Standalone C++ intersection expected tables currently deferred in Phase 09 | `cavalier_contours/tests/test_cpp_offset_parity.rs`, `cavalier_contours/tests/test_pline_seg_intersect.rs`, `cavalier_contours/src/polyline/internal/pline_intersects.rs` | Close the remaining not-comparable intersection parity gap with executable expectations. |
| P1 | Additional closest-point tie/index stress probes for generated circle reverse/diagonal variants | `cavalier_contours/tests/test_cpp_pline_function_parity.rs`, `cavalier_contours/src/polyline/traits.rs` | Further pressure-test deterministic closest-point index behavior after tie-break fix. |
| P2 | Remaining C++ C-API pline operation suite parity candidates | `cavalier_contours_ffi/tests/test_pline.rs`, `cavalier_contours/tests/*` | Extend deep parity into C-API surface behavior where Rust mappings are direct. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_offset_parity.rs`
  - `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- Rust geometry core:
  - `cavalier_contours/src/polyline/internal/pline_offset.rs`
  - `cavalier_contours/src/polyline/internal/pline_intersects.rs`
  - `cavalier_contours/src/polyline/traits.rs`


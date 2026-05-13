# Phase 20: C-API Logic Alignment Map

This map defines next deep parity targets after the coincident intersect C-API
bridge closure.

## Alignment Completed in Phase 20

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| `coincident_case1_intersect` C-API empty expectation | direct FFI boolean parity test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| Intersect mode mapping verification | operation `1` mapped to `BooleanOp::And` | `cavalier_contours_ffi/src/lib.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API combine matrix expansion (circle/rectangle + coincident case2) | `cavalier_contours_ffi/tests/test_pline.rs` | Add source-traceable expected counts/properties from old C++ before widening case count. |
| P1 | C-API parallel-offset parity bridge | `cavalier_contours_ffi/tests/test_pline.rs`, `cavalier_contours/tests/test_cpp_offset_parity.rs` | Start with already-stable no-Clipper cases and keep option/tolerance assumptions explicit. |
| P2 | C-API function-surface parity probes (path/area/extents/winding/closest) | `cavalier_contours_ffi/tests/test_pline.rs` | Extend only where old C++ has explicit comparable expectations. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`

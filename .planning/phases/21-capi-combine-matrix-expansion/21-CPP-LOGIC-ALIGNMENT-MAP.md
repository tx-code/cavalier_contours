# Phase 21: C-API Logic Alignment Map

This map defines next deep C-API parity targets after combine matrix expansion.

## Alignment Completed in Phase 21

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| `circle_rectangle` combine matrix | direct FFI boolean matrix parity test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| `coincident_case2` combine matrix | direct FFI boolean matrix parity test (including both exclude directions) | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API parallel-offset matrix parity bridge | `cavalier_contours_ffi/tests/test_pline.rs` + `TEST_cavc_parallel_offset.cpp` mapping | Start with simple + specific old C++ cases already green in Rust core parity, keep option/tolerance assumptions explicit. |
| P1 | C-API combine self-invariants parity bridge | `cavalier_contours_ffi/tests/test_pline.rs` | Mirror old C++ combine-with-self invariants (all modes) with explicit expected counts/properties. |
| P2 | C-API function-surface parity probes | `cavalier_contours_ffi/tests/test_pline.rs` + `TEST_cavc_pline_function.cpp` | Expand only where old C++ has explicit, comparable expectations and stable tie-break policy. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`

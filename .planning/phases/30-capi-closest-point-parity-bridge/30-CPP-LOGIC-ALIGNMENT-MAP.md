# Phase 30: C-API Logic Alignment Map

This map defines next deep C-API parity targets after closest-point C-API
bridge closure.

## Alignment Completed in Phase 30

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| closest-point function surface | new C-API closest-point evaluator | `cavalier_contours_ffi/src/lib.rs` + `cavalier_contours_ffi.h` | complete |
| circle closest-point probes (vertex + axis + 45-degree) | C-API closest-point parity matrix tests | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| closest-point null/empty behavior | explicit C-API error-path checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API half-circle closest-point strict-index parity | `cavalier_contours_ffi/tests/test_pline.rs` | Reuse source-backed half-circle case matrix with explicit index expectations. |
| P1 | C-API function-surface completion pass | `cavalier_contours_ffi/tests/test_pline.rs` + `.planning/phases/*` | Ensure function-surface parity coverage map explicitly closes remaining uncovered probes. |
| P2 | C-API closest-point options/epsilon sensitivity matrix | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-traceable epsilon/tie-break probes tied to old C++ expectations. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`

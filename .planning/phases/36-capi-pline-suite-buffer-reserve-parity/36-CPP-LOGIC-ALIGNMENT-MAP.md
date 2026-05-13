# Phase 36: C-API Logic Alignment Map

This map defines next deep C-API parity targets after pline-suite buffer/reserve
parity closure.

## Alignment Completed in Phase 36

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| empty pline vertex_data buffer no-write behavior | sentinel-buffer no-write assertion | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| reserve operation on populated pline | vertex persistence assertion across reserve calls | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| pline-suite edge semantics hardening | explicit source-backed edge tests | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Cross-suite closure audit (old C++ C-API tests -> FFI evidence) | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Produce explicit mapping for `TEST_cavc_pline`, `TEST_cavc_pline_function`, `TEST_cavc_parallel_offset`, `TEST_cavc_combine_plines` and close only source-explicit gaps. |
| P1 | Residual source-explicit edge catalog completion | `cavalier_contours_ffi/tests/test_pline.rs` + `.planning/phases/*` | Add only direct old C++ expectations not yet represented; avoid synthetic speculative cases. |
| P2 | Options-path deep edge catalog | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Keep additions driven by demonstrated parity gaps rather than speculative synthetic cases. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi/src/lib.rs`

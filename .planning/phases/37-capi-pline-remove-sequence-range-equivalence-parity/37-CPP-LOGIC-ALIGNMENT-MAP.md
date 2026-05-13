# Phase 37: C-API Logic Alignment Map

This map defines next deep C-API parity targets after remove-sequence
range-equivalence parity closure.

## Alignment Completed in Phase 37

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| pline remove_range scenario | ordered remove-sequence equivalence test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| intermediate vertex transition behavior | vertex-level step assertions | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| final empty-state removal closure | vertex count closure assertion | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Final cross-suite closure audit (old C++ -> FFI evidence) | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Build explicit checklist per old test block and close only source-explicit residual gaps. |
| P1 | Residual source-explicit edge catalog completion | `cavalier_contours_ffi/tests/test_pline.rs` + `.planning/phases/*` | Add only direct old C++ expectations not yet represented; avoid synthetic assumptions. |
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

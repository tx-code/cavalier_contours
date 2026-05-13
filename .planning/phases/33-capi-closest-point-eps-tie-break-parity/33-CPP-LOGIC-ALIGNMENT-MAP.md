# Phase 33: C-API Logic Alignment Map

This map defines next deep C-API parity targets after closest-point
epsilon/tie-break matrix parity closure.

## Alignment Completed in Phase 33

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| closest-point explicit index probes under source tolerance | epsilon matrix index/point/distance checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| circle shared-vertex tie-break behavior | explicit index checks across epsilon matrix | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| half-circle strict index closest-point behavior under epsilon variation | explicit index matrix checks across epsilon matrix | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API function-surface parallel-offset matrix parity completion | `cavalier_contours_ffi/tests/test_pline.rs` + `.planning/phases/*` | Port remaining function-surface offset expectations from old C++ cases with explicit vertex/properties evidence. |
| P1 | C-API closest-point edge-case catalog (source explicit only) | `cavalier_contours_ffi/tests/test_pline.rs` + `.planning/phases/*` | Add only explicit old C++ closest-point expectations not yet represented; no synthetic tie-break assumptions. |
| P2 | C-API coincident function-surface edge catalog | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Keep additions driven by demonstrated parity gaps rather than speculative synthetic cases. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi/src/lib.rs`

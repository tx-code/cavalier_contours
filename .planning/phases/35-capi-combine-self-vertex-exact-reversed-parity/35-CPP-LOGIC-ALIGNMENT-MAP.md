# Phase 35: C-API Logic Alignment Map

This map defines next deep C-API parity targets after combine-self vertex-exact
reversed parity closure.

## Alignment Completed in Phase 35

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| nontrivial sample self-combine invariants | vertex-exact self-combine C-API checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| reversed self-combine output invariants | exact reversed vertex output checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| reversed/forward cross emptiness invariants | explicit empty-result checks for exclude/xor | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API source-explicit edge catalog completion | `cavalier_contours_ffi/tests/test_pline.rs` + `.planning/phases/*` | Extract remaining explicit old C++ expectations (if any) and add only direct, source-traceable tests. |
| P1 | C-API parity closure audit against old C++ test suites | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Build checklist mapping each old C++ C-API test block to concrete FFI evidence before declaring alignment closure. |
| P2 | C-API options-path deep edge catalog | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Keep additions driven by demonstrated parity gaps rather than speculative synthetic cases. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi/src/lib.rs`

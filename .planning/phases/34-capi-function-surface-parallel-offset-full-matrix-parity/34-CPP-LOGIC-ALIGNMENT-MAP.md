# Phase 34: C-API Logic Alignment Map

This map defines next deep C-API parity targets after function-surface
full-matrix parallel-offset parity closure.

## Alignment Completed in Phase 34

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| function-surface generated circle matrix parallel offsets | full-matrix outward/inward C-API offset checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| function-surface generated half-circle matrix parallel offsets | full-matrix outward/inward C-API offset checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| function-surface collapsed offset expectations | matrix collapsed-delta empty-result checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API closest/coincident source-explicit edge catalog | `cavalier_contours_ffi/tests/test_pline.rs` + `.planning/phases/*` | Add only explicit old C++ expectations not yet represented at C-API boundary. |
| P1 | C-API function-surface consolidated matrix closure audit | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Verify every C++ function-surface matrix category has matching C-API evidence without relying on inferred coverage. |
| P2 | C-API options-path deep edge catalog | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Keep additions driven by demonstrated parity gaps rather than speculative synthetic cases. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi/src/lib.rs`

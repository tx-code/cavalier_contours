# Phase 24: C-API Logic Alignment Map

This map defines next deep C-API parity targets after combine no-modify bridge
closure.

## Alignment Completed in Phase 24

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| combine no-modify operation matrix | direct C-API boolean no-modify test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| subject+clip immutability verification | pre/post vertex buffer comparisons for each mode | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API function-surface parity probes (path/area/extents/winding/closest) | `cavalier_contours_ffi/tests/test_pline.rs` + `TEST_cavc_pline_function.cpp` | Expand only where old C++ expectations are explicit and comparable. |
| P1 | C-API optioned-boolean/offset parity edge cases | `cavalier_contours_ffi/tests/test_pline.rs` | Use default-path parity anchors first; add options-path only for source-backed edge behavior. |
| P2 | C-API coincident-case broadened no-modify checks | `cavalier_contours_ffi/tests/test_pline.rs` | Extend no-modify coverage to heavier coincident matrices once function-surface probes stabilize. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`

# Phase 26: C-API Logic Alignment Map

This map defines next deep C-API parity targets after options-path parity
bridge closure.

## Alignment Completed in Phase 26

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| circle/rectangle boolean matrix options path | default-vs-options parity bridge via `cavc_pline_boolean_o` | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| imported offset matrices options path | default-vs-options parity bridge via `cavc_pline_parallel_offset_o` | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API broadened coincident no-modify matrices | `cavalier_contours_ffi/tests/test_pline.rs` | Expand to heavier coincident matrices only where old C++ operation expectations remain explicit. |
| P1 | C-API optioned coincident collapsed-area edge behavior | `cavalier_contours_ffi/tests/test_pline.rs` | Keep source-backed coincident cases and avoid synthetic thresholds not present in C++ evidence. |
| P2 | C-API closest-point parity bridge | `cavalier_contours_ffi/src/lib.rs` + `cavalier_contours_ffi/tests/test_pline.rs` | Requires explicit closest-point C-API introduction and approval before parity import. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`

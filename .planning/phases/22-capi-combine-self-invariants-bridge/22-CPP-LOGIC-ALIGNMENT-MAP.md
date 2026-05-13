# Phase 22: C-API Logic Alignment Map

This map defines next deep C-API parity targets after combine self-invariants
bridge closure.

## Alignment Completed in Phase 22

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| combine-with-self invariants | direct C-API boolean invariants parity test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| reversed + mixed-orientation empty-result invariants | explicit Not/Xor mixed-orientation cases | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API parallel-offset matrix parity bridge | `cavalier_contours_ffi/tests/test_pline.rs` + `TEST_cavc_parallel_offset.cpp` mapping | Start with simple + specific cases that already have stable Rust-core parity anchors. |
| P1 | C-API combine input-immutability parity | `cavalier_contours_ffi/tests/test_pline.rs` | Mirror old C++ no-modify input checks for representative combine matrices. |
| P2 | C-API function-surface parity probes | `cavalier_contours_ffi/tests/test_pline.rs` + `TEST_cavc_pline_function.cpp` | Expand only when old C++ has explicit comparable expectations and tie-break policy remains stable. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`

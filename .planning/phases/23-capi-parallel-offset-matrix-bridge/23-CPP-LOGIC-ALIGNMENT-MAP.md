# Phase 23: C-API Logic Alignment Map

This map defines next deep C-API parity targets after parallel-offset matrix
bridge closure.

## Alignment Completed in Phase 23

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| `parallel_offset` simple matrix | direct C-API offset matrix parity test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| `parallel_offset` specific matrix | direct C-API offset matrix parity test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| reversed/no-modify invariants | explicit C-API reversed and no-modify tests | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API combine input-immutability matrix parity | `cavalier_contours_ffi/tests/test_pline.rs` + old C++ combine no-modify suite | Start with existing combine matrix anchors and add no-modify checks per mode/case. |
| P1 | C-API function-surface parity probes | `cavalier_contours_ffi/tests/test_pline.rs` + `TEST_cavc_pline_function.cpp` | Expand only where old C++ expectations are explicit and stable. |
| P2 | C-API optioned-offset parity edge cases | `cavalier_contours_ffi/tests/test_pline.rs` | Add options-path parity only after default-path matrix parity is fully stabilized. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`

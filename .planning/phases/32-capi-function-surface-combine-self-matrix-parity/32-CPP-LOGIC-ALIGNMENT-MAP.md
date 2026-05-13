# Phase 32: C-API Logic Alignment Map

This map defines next deep C-API parity targets after function-surface
combine-with-self matrix parity closure.

## Alignment Completed in Phase 32

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| function-surface combine_with_self invariants (circle matrix) | matrix self-boolean invariant test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| function-surface combine_with_self invariants (closed half-circle matrix) | matrix self-boolean invariant test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| output vertex and no-modify checks | boolean output vertex + input persistence assertions | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API closest-point epsilon/tie-break sensitivity probes | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-traceable tie-break/epsilon probes where old C++ expectations are explicit. |
| P1 | C-API function-surface parallel-offset matrix parity completion | `cavalier_contours_ffi/tests/test_pline.rs` + `.planning/phases/*` | Port remaining function-surface offset expectations from old C++ cases with explicit vertex/properties evidence. |
| P2 | C-API coincident function-surface edge catalog | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Keep additions driven by demonstrated parity gaps rather than speculative synthetic cases. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi/src/lib.rs`

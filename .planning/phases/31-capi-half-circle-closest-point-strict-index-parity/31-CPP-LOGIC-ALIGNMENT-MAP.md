# Phase 31: C-API Logic Alignment Map

This map defines next deep C-API parity targets after half-circle closest-point
strict index parity closure.

## Alignment Completed in Phase 31

| C++ source target | Rust target | Module/File | Status |
|-------------------|-------------|-------------|--------|
| half-circle closest-point generated matrix strict index probes | C-API closest-point strict index matrix parity test | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| open/closed endpoint index behavior | explicit strict index assertions per case | `cavalier_contours_ffi/tests/test_pline.rs` | complete |
| alignment/direction variant closest-point behavior | generated matrix parity checks | `cavalier_contours_ffi/tests/test_pline.rs` | complete |

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | C-API function-surface matrix completion pass | `cavalier_contours_ffi/tests/test_pline.rs` + `.planning/phases/*` | Reconcile remaining function-level probes from old C++ list against current C-API coverage map. |
| P1 | C-API closest-point epsilon/tie-break sensitivity probes | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-traceable tie-break/epsilon probes where old C++ expectations are explicit. |
| P2 | C-API coincident function-surface edge catalog | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Keep additions driven by demonstrated parity gaps rather than speculative synthetic cases. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi/src/lib.rs`

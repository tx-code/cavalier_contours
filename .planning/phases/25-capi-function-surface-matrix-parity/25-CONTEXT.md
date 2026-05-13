# Phase 25: capi-function-surface-matrix-parity - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 25 bridges old C++ `TEST_cavc_pline_function.cpp` function-surface matrix
expectations through Rust FFI C-API calls for area/path/extents/winding.

## Decisions

- **D-01:** Import circle and half-circle generated matrix expectations into
  C-API parity tests with explicit source-traceable formulas.
- **D-02:** Classify closest-point parity as not-comparable at C-API boundary
  because `cavalier_contours_ffi` does not expose a closest-point API.
- **D-03:** Keep options-path parity work for next phase after default-path
  function-surface matrix stabilization.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

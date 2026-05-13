# Phase 30: capi-closest-point-parity-bridge - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 30 introduces closest-point C-API surface and bridges source-backed
closest-point parity checks through FFI.

## Decisions

- **D-01:** Expose `cavc_pline_eval_closest_point` with explicit null and empty
  polyline behavior codes.
- **D-02:** Reuse source-backed generated circle matrix probes for closest-point
  parity at C-API boundary.
- **D-03:** Regenerate root C header after ABI surface change.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`
- `cavalier_contours_ffi.h`

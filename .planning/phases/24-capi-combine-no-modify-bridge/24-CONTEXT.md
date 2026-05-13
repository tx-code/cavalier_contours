# Phase 24: capi-combine-no-modify-bridge - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 24 bridges old C++ combine no-modify input invariants through Rust FFI
`cavc_pline_boolean` operation matrix checks.

## Decisions

- **D-01:** Reuse old C++ representative circle/rectangle combine matrix
  geometry for no-modify checks.
- **D-02:** Validate both subject and clip vertex buffers before/after each
  operation call.
- **D-03:** Keep this phase focused on immutability parity and defer additional
  C-API function-surface expansion.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

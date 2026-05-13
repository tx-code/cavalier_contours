# Phase 21: capi-combine-matrix-expansion - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 21 extends C-API parity from the single Phase 20 bridge case into full
source-traceable combine matrices for:

- `circle_rectangle`
- `coincident_case2`

using `cavc_pline_boolean` in `cavalier_contours_ffi`.

## Decisions

- **D-01:** Reuse old C++ expected property sets (vertex count, area,
  path-length, extents) as parity ground truth.
- **D-02:** Match property sets unordered and ignore area sign, consistent with
  historical C++ `EqIgnoreSignOfArea` behavior.
- **D-03:** Keep this phase focused on combine matrix coverage; defer offset and
  function-surface C-API expansions to follow-up phases.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

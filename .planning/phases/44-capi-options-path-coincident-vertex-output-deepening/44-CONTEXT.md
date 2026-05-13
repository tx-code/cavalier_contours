# Phase 44: capi-options-path-coincident-vertex-output-deepening - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 44 deepens options-path coincident parity by comparing default-path and
options-path outputs at vertex level across source-backed coincident matrices.

## Decisions

- **D-01:** Use source-backed coincident case1/case2 matrices as canonical
  options-path deepening surface.
- **D-02:** Compare both remaining and subtracted output sets at vertex level.
- **D-03:** Keep this phase in FFI tests only; no core geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

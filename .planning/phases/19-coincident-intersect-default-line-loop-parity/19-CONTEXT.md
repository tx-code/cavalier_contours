# Phase 19: coincident-intersect-default-line-loop-parity - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 19 targets the remaining default-path gap from Phase 18:
`coincident_case1_intersect` could still emit a tiny line-only loop unless
`collapsed_area_eps` was explicitly set.

## Decisions

- **D-01:** Do not change default tolerance values.
- **D-02:** Fix only the structural degenerate case: two-vertex closed loops
  where both segments are line segments.
- **D-03:** Preserve valid two-vertex arc loops used by existing boolean tests.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours/src/polyline/internal/pline_boolean.rs`
- `cavalier_contours/tests/test_cpp_combine_parity.rs`

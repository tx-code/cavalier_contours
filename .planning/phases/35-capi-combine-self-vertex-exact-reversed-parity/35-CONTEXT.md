# Phase 35: capi-combine-self-vertex-exact-reversed-parity - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 35 closes C-API combine-with-self invariants at vertex-exact level for
the source-backed nontrivial sample polyline, including reversed-direction
combinations.

## Decisions

- **D-01:** Keep existing property-level self-invariant checks, and add explicit
  vertex-level checks matching old C++ combine tests.
- **D-02:** Reuse `run_boolean_vertexes` helper to validate exact output
  vertices for union/intersect and empty results for exclude/xor.
- **D-03:** Include reversed self and reversed/forward cross-combinations where
  old C++ source explicitly checks them.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

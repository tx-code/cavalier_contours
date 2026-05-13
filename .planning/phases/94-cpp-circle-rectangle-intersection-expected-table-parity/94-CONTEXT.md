# Phase 94: cpp-circle-rectangle-intersection-expected-table-parity - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 94 deepens old C++ intersection parity by replacing a count-only
circle/rectangle snapshot with executable expected-table assertions:

- exact basic-intersect cardinality
- segment-index attribution
- coordinate-level point checks
- explicit empty-overlapping assertion

## Decisions

- **D-01:** Keep this phase scoped to Rust parity tests and planning artifacts.
- **D-02:** Reuse historical C++ circle/rectangle geometry already in
  `TEST_cavc_combine_plines.cpp`.
- **D-03:** Treat this as a parity-evidence deepening slice, not a geometry
  kernel rewrite.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours/tests/test_cpp_offset_parity.rs`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`


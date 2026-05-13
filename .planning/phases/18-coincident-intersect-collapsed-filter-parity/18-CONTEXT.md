# Phase 18: coincident-intersect-collapsed-filter-parity - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 18 targets the bounded Phase 17 divergence at
`coincident_case1_intersect` by validating an explicit collapsed-area filtered
parity path (`collapsed_area_eps`) against old C++ empty-output expectation.

## Decisions

- **D-01:** Keep default boolean behavior unchanged in this phase.
- **D-02:** Add executable parity evidence for the filtered path to prove
  alignment without broad default-behavior churn.
- **D-03:** Record explicit decision boundary for future default-threshold
  adoption work.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours/tests/test_cpp_combine_parity.rs`


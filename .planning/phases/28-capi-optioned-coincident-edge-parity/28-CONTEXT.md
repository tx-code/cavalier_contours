# Phase 28: capi-optioned-coincident-edge-parity - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 28 validates optioned coincident edge behavior through C-API
`collapsed_area_eps` and options-path no-modify invariants.

## Decisions

- **D-01:** Reuse source-backed coincident case1 intersect as collapsed-area
  edge anchor.
- **D-02:** Use the same source-backed coincident case1/case2 matrix inputs for
  options-path no-modify checks.
- **D-03:** Keep options-path invariants scoped to stabilized matrix cases and
  defer new synthetic edge geometry.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours/tests/test_cpp_combine_parity.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

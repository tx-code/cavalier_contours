# Phase 29: capi-optioned-coincident-output-parity - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 29 validates output parity between default-path and options-path for
source-backed coincident case matrices in C-API boolean operations.

## Decisions

- **D-01:** Reuse stabilized coincident case1/case2 matrix inputs from earlier
  parity phases.
- **D-02:** Compare options-path output property sets against default-path
  output property sets for each operation/case.
- **D-03:** Keep this phase focused on output parity equivalence and defer C-API
  closest-point surface decision.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

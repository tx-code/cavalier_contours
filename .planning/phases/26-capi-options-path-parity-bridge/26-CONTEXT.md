# Phase 26: capi-options-path-parity-bridge - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 26 validates options-path behavior parity against default-path behavior for
source-backed C-API boolean and offset matrix cases.

## Decisions

- **D-01:** Reuse stabilized C++-anchored matrix cases (circle/rectangle boolean
  and imported offset matrices) as options-path parity anchors.
- **D-02:** Compare options-path output property sets against default-path output
  property sets for the same inputs and operations.
- **D-03:** Keep the phase focused on options-path equivalence and defer broader
  coincident optioned matrices to next scope.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

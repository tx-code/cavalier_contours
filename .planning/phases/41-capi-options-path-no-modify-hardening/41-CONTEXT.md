# Phase 41: capi-options-path-no-modify-hardening - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 41 extends source-backed no-modify invariants into options-path calls for
boolean circle/rectangle and parallel-offset matrix paths at the C-API layer.

## Decisions

- **D-01:** Reuse source-backed simple/specific offset and circle/rectangle
  boolean matrices as canonical options-path no-modify surfaces.
- **D-02:** Validate no-modify invariants at vertex-level equality for input
  polylines.
- **D-03:** Keep this phase test-only and avoid core geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

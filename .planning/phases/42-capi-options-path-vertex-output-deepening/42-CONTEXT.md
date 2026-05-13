# Phase 42: capi-options-path-vertex-output-deepening - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 42 deepens options-path parity by comparing default-path and options-path
outputs at vertex level for source-backed boolean and offset matrices.

## Decisions

- **D-01:** Keep deepening scope to source-backed matrix surfaces:
  circle/rectangle boolean and simple/specific offset cases.
- **D-02:** Compare output as unordered polyline sets with fuzzy vertex matching
  and closed-curve rotation tolerance.
- **D-03:** Keep this phase in FFI tests only; no core geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

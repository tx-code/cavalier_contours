# Phase 45: capi-options-path-tolerance-matrix-deepening - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 45 deepens options-path parity by checking bounded tolerance/epsilon
matrix stability against default-path outputs on source-backed boolean and
offset matrix surfaces.

## Decisions

- **D-01:** Keep tolerance matrix bounded (`0.5x`, `1.0x`, `2.0x`) around
  default option values.
- **D-02:** Validate both property-level and vertex-level output equivalence.
- **D-03:** Keep this phase in FFI tests only; no core geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

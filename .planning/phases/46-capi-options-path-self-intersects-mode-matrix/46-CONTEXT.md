# Phase 46: capi-options-path-self-intersects-mode-matrix - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 46 deepens options-path parity by validating self-intersects include mode
matrix stability on source-backed non-self-intersecting offset cases.

## Decisions

- **D-01:** Use source-backed simple offset cases as mode-matrix surface.
- **D-02:** Validate both property-level and vertex-level output equivalence.
- **D-03:** Keep this phase in FFI tests only; no core geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

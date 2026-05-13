# Phase 47: capi-self-intersects-mode-no-modify-matrix - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 47 deepens self-intersects mode coverage by adding no-modify invariants
across source-backed simple and specific offset matrices.

## Decisions

- **D-01:** Reuse source-backed simple/specific offset matrices.
- **D-02:** Validate input stability for each self-intersects include mode.
- **D-03:** Keep this phase in FFI tests only; no core geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

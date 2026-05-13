# Phase 22: capi-combine-self-invariants-bridge - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 22 extends C-API combine parity to include old C++ self-invariant
semantics through `cavc_pline_boolean`, including reversed and mixed-orientation
empty-result cases.

## Decisions

- **D-01:** Reuse the old C++ invariant polyline geometry from
  `TEST_cavc_combine_plines.cpp`.
- **D-02:** Validate invariants at property-set level through FFI output lists.
- **D-03:** Keep this phase narrow to self-invariants; defer C-API parallel
  offset parity matrix to the next phase.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

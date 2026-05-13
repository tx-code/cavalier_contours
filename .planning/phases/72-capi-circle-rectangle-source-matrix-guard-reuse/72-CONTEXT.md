# Phase 72: capi-circle-rectangle-source-matrix-guard-reuse - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 72 hardens source-backed circle-rectangle C-API boolean parity by:

- enforcing canonical `name+operation` mapping guard on the explicit default
  matrix, and
- reusing one canonical operation-sequence constant across
  default/options/no-modify matrix surfaces.

## Decisions

- **D-01:** Keep this phase scoped to FFI parity tests and planning artifacts.
- **D-02:** Reuse existing shared mapping guard helper for circle-rectangle
  default matrix checks.
- **D-03:** Preserve output-property assertions; this phase adds mapping/order
  drift protection only.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/71-capi-coincident-default-matrix-source-map-guard/71-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

# Phase 82: capi-plinelist-failure-path-output-stability-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 82 hardens direct failure-path output-stability contract coverage for
plinelist surfaces:

- `cavc_plinelist_get_count`
- `cavc_plinelist_get_pline`
- `cavc_plinelist_pop`
- `cavc_plinelist_take`

with explicit null/OOB/empty return-code assertions and sentinel stability
checks on out parameters.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Prioritize direct API-level failure-path assertions over implicit behavior.
- **D-03:** Treat output-sentinel stability as a first-class C-API contract check.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

# Phase 85: capi-pline-core-accessor-output-stability-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 85 hardens direct failure-path output-stability contract coverage for
pline core accessor surfaces:

- `cavc_pline_clone`
- `cavc_pline_get_is_closed`
- `cavc_pline_get_vertex_count`
- `cavc_pline_get_vertex_data`
- `cavc_pline_get_vertex`
- `cavc_pline_get_userdata_count`
- `cavc_pline_get_userdata_values`

with explicit null/OOB return-code assertions and out-parameter sentinel
stability checks.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Prioritize direct API-level failure-path assertions over implicit behavior.
- **D-03:** Treat output-sentinel stability as first-class pline core accessor contract evidence.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

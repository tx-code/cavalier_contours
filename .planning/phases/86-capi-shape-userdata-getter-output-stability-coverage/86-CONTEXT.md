# Phase 86: capi-shape-userdata-getter-output-stability-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 86 hardens direct failure-path output-stability contract coverage for
shape userdata getter surfaces:

- `cavc_shape_get_ccw_pline_userdata_count`
- `cavc_shape_get_ccw_pline_userdata_values`
- `cavc_shape_get_cw_pline_userdata_count`
- `cavc_shape_get_cw_pline_userdata_values`

with explicit null/OOB return-code assertions and out-parameter sentinel
stability checks.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Prioritize direct API-level failure-path assertions over implicit behavior.
- **D-03:** Treat output-sentinel stability as first-class shape userdata getter contract evidence.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

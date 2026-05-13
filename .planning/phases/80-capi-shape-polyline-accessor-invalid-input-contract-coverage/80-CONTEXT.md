# Phase 80: capi-shape-polyline-accessor-invalid-input-contract-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 80 hardens direct invalid-input contract coverage for shape polyline
accessor surfaces:

- `cavc_shape_get_ccw_polyline_count`
- `cavc_shape_get_ccw_polyline_is_closed`
- `cavc_shape_get_ccw_polyline_vertex_data`
- `cavc_shape_get_cw_polyline_count`
- `cavc_shape_get_cw_polyline_is_closed`
- `cavc_shape_get_cw_polyline_vertex_data`

including failure-path output stability checks and FFI doc wording alignment.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests/docs/planning artifacts.
- **D-02:** Prioritize direct API-level null/OOB error-code assertions.
- **D-03:** Assert failure-path output sentinel stability for caller-provided out buffers.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi.h`
- `cavalier_contours_ffi/tests/test_pline.rs`

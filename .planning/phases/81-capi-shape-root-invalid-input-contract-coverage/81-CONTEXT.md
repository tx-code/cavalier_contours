# Phase 81: capi-shape-root-invalid-input-contract-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 81 hardens direct invalid-input contract coverage for shape-root surfaces:

- `cavc_shape_create`
- `cavc_shape_parallel_offset`
- `cavc_shape_get_ccw_count`
- `cavc_shape_get_cw_count`

including failure-path output sentinel stability checks and shape-surface doc
reference alignment.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests/docs/planning artifacts.
- **D-02:** Prioritize direct API-level null-input error-code assertions.
- **D-03:** Assert failure-path output sentinel stability for root out-pointer and count outputs.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi.h`
- `cavalier_contours_ffi/tests/test_pline.rs`

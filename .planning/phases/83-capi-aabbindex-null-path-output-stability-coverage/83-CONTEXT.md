# Phase 83: capi-aabbindex-null-path-output-stability-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 83 hardens direct null-path output-stability contract coverage for
aabbindex surfaces:

- `cavc_pline_create_approx_aabbindex`
- `cavc_pline_create_aabbindex`
- `cavc_aabbindex_get_extents`

with explicit null-input return-code assertions and sentinel stability checks
for out parameters.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Prioritize direct API-level failure-path assertions over implicit behavior.
- **D-03:** Treat out-parameter sentinel stability as first-class aabbindex contract evidence.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

# Phase 89: capi-shape-offset-null-path-output-stability-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 89 hardens direct failure-path output-stability contract coverage for:

- `cavc_shape_parallel_offset` (`result` shape output)

with explicit null-input return-code assertions and out-parameter sentinel
stability checks across default-options and explicit-options calls.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Prioritize direct API-level failure-path assertions over implicit behavior.
- **D-03:** Treat output-sentinel stability as first-class shape-offset contract evidence.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

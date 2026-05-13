# Phase 84: capi-pline-eval-failure-path-output-stability-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 84 hardens direct failure-path output-stability contract coverage for
pline-eval surfaces:

- `cavc_pline_eval_path_length`
- `cavc_pline_eval_area`
- `cavc_pline_eval_wn`
- `cavc_pline_eval_extents`
- `cavc_pline_eval_closest_point`

with explicit null/empty-path return-code assertions and out-parameter sentinel
stability checks.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Prioritize direct API-level failure-path assertions over implicit behavior.
- **D-03:** Treat output-sentinel stability as first-class pline-eval contract evidence.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

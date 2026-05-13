# Phase 87: capi-boolean-self-intersect-output-stability-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 87 hardens direct failure-path output-stability contract coverage for:

- `cavc_pline_boolean` (`pos_plines`, `neg_plines` outputs)
- `cavc_pline_scan_for_self_intersect` (`is_self_intersecting` output)

with explicit invalid-operation/invalid-options/null return-code assertions and
out-parameter sentinel stability checks.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Prioritize direct API-level failure-path assertions over implicit behavior.
- **D-03:** Treat output-sentinel stability as first-class boolean/self-intersect contract evidence.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

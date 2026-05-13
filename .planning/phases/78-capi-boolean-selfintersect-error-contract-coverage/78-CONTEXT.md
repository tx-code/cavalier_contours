# Phase 78: capi-boolean-selfintersect-error-contract-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 78 hardens direct error-contract coverage for:

- boolean operation dispatch (`cavc_pline_boolean`), and
- self-intersect options validation (`cavc_pline_scan_for_self_intersect`),

including alignment of API docs with actual parameter naming.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests/docs/planning artifacts.
- **D-02:** Prioritize direct API-level error code assertions over indirect
  behavior.
- **D-03:** Keep doc wording aligned to runtime contract (`pline` naming).

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi.h`
- `cavalier_contours_ffi/tests/test_pline.rs`

# Phase 79: capi-contains-extents-invalid-input-contract-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 79 hardens direct invalid-input contract coverage for:

- contains invalid-input result semantics (`cavc_pline_contains`), and
- extents degenerate-input semantics (`cavc_pline_eval_extents`).

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Prioritize direct API-level error/result assertions over indirect behavior.
- **D-03:** Preserve no-Clipper boundary and avoid geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

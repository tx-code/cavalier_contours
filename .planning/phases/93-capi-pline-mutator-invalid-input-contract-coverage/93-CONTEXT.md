# Phase 93: capi-pline-mutator-invalid-input-contract-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 93 hardens invalid-input contract coverage for pline mutator surfaces:

- `cavc_pline_set_vertex_data`
- `cavc_pline_set_is_closed`
- `cavc_pline_clear`
- `cavc_pline_set_vertex`
- `cavc_pline_remove`

with direct null/OOB return-code assertions.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Prioritize direct API-level invalid-input return-code assertions.
- **D-03:** Keep OOB assertions explicit on mutators that document index-bound errors.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

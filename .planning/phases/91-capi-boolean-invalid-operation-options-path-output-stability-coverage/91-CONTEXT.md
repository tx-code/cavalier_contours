# Phase 91: capi-boolean-invalid-operation-options-path-output-stability-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 91 hardens explicit-options invalid-input contract coverage for:

- `cavc_pline_boolean` invalid-operation output stability
- `cavc_pline_contains` null-result-pointer invalid-input behavior

with direct return-code and output-behavior assertions.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Validate invalid-operation behavior remains invariant between default/options paths.
- **D-03:** Keep null-result-pointer invalid-input checks explicit on options path.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

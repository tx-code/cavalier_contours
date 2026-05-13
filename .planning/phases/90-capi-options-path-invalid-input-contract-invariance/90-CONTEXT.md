# Phase 90: capi-options-path-invalid-input-contract-invariance - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 90 hardens explicit-options invalid-input contract invariance for:

- `cavc_pline_boolean` (null-input error path and output-pointer stability)
- `cavc_pline_contains` (null-input error path and deterministic invalid-result writes)

with direct return-code and output-behavior assertions.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Validate options-path behavior remains invariant with default-path invalid-input contracts.
- **D-03:** Treat deterministic invalid-result writes as first-class contains failure-path evidence.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

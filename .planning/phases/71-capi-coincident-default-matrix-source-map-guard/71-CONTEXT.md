# Phase 71: capi-coincident-default-matrix-source-map-guard - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 71 hardens explicit source-backed C-API boolean parity by adding shared
`name+operation` mapping guards to both default-path matrix suites:
`coincident_case1` and `coincident_case2`.

## Decisions

- **D-01:** Keep this phase scoped to FFI test guards and planning artifacts.
- **D-02:** Reuse one helper for mapping diagnostics instead of duplicating
  case-count/name/operation assertions.
- **D-03:** Preserve existing expected output-property assertions and only add
  preflight source-mapping drift protection.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/70-capi-coincident-case1-matrix-parity-expansion/70-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

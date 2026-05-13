# Phase 92: capi-self-intersect-contains-null-result-contract-symmetry - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 92 hardens null-result/invalid-input contract symmetry for:

- `cavc_pline_scan_for_self_intersect` null-input behavior across default/options paths
- `cavc_pline_contains` explicit-options null-result-pointer behavior symmetry

with direct return-code and output-stability assertions.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Validate default/options-path symmetry for null-input behavior.
- **D-03:** Keep null-result-pointer invalid-input checks explicit and deterministic.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

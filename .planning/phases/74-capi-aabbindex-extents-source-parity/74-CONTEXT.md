# Phase 74: capi-aabbindex-extents-source-parity - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 74 hardens C-API aabbindex parity by adding source-backed extents checks
aligned to old `TEST_staticspatialindex.cpp` extents assertions and by covering
exact/approx/extents edge behavior in FFI tests.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Map source-backed extents behavior to C-API aabbindex constructors
  (`create_approx_aabbindex` and `create_aabbindex`) with explicit expected
  extents.
- **D-03:** Preserve existing parity coverage and add null/empty extents checks
  as bounded hardening.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/73-capi-pline-core-suite-source-coverage-parity/73-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_staticspatialindex.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

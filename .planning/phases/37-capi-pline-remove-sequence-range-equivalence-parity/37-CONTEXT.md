# Phase 37: capi-pline-remove-sequence-range-equivalence-parity - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 37 closes source-backed `remove_range` behavior equivalence from old
pline suite by validating equivalent remove-sequence behavior on current C-API
surface.

## Decisions

- **D-01:** Use old `TEST_cavc_pline.cpp` remove-range scenario as source of
  truth.
- **D-02:** Validate equivalent behavior via ordered `cavc_pline_remove` calls
  because current API exposes single-element removal.
- **D-03:** Keep assertions at vertex-level after each step and final count
  closure.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`

# Phase 33: capi-closest-point-eps-tie-break-parity - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 33 closes C-API closest-point epsilon/tie-break sensitivity parity for
source-backed explicit index expectations.

## Decisions

- **D-01:** Restrict epsilon/tie-break probes to old C++ points with explicit
  segment index expectations.
- **D-02:** Reuse existing source-backed circle and half-circle matrix case
  builders.
- **D-03:** Cover multiple `pos_equal_eps` values while keeping expected
  point/distance/index stable.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`

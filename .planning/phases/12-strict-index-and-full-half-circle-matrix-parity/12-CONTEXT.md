# Phase 12: strict-index-and-full-half-circle-matrix-parity - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 12 deepens old C++ vs Rust parity (no Clipper) by expanding generated
`pline_function` half-circle cases from bounded subset to full matrix coverage
with strict closest-point index checks.

## Decisions

- **D-01:** Keep tolerance management centralized in parity tests using
  `test_utils::PlineProperties` constants.
- **D-02:** Treat strict closest-point index mismatch as logic-level parity gap,
  not as non-comparable, when old C++ index expectations are explicit.
- **D-03:** Keep Phase 12 scoped to `closest_point` plus generated half-circle
  function matrix; defer full circle matrix expansion to follow-up.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/src/polyline/traits.rs`
- `cavalier_contours/tests/test_cpp_pline_function_parity.rs`

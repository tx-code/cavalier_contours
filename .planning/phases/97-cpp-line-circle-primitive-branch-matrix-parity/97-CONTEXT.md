# Phase 97: cpp-line-circle-primitive-branch-matrix-parity - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 97 deepens old C++ standalone primitive parity by adding source-traceable
line-circle expected-table coverage for `intrLineSeg2Circle2` branch families:

- degenerate-point on/off-circle
- tangent
- no-intersect
- two-intersect (inside and outside segment)

## Decisions

- **D-01:** Keep this phase scoped to Rust parity tests and planning artifacts.
- **D-02:** Use bounded branch-matrix assertions with explicit parametric
  outputs (`t0`, `t1`).
- **D-03:** Treat this as parity-evidence deepening only; do not edit geometry
  kernel without a confirmed mismatch.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours/tests/test_cpp_line_circle_parity.rs`
- `E:/Coding/CavalierContours/include/cavc/intrlineseg2circle2.hpp`


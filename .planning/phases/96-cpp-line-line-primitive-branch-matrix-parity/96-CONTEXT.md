# Phase 96: cpp-line-line-primitive-branch-matrix-parity - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 96 deepens old C++ standalone primitive parity by adding source-traceable
line-line expected-table coverage for `intrLineSeg2LineSeg2` branch families:

- non-parallel `True` and `False`
- parallel non-collinear `None`
- collinear overlap `Coincident`
- degenerate point branches (`point-point`, `point-segment`)

## Decisions

- **D-01:** Keep this phase scoped to Rust parity tests and planning artifacts.
- **D-02:** Use bounded branch-matrix assertions rather than broad randomized
  geometry generation.
- **D-03:** Treat this as parity-evidence deepening only; do not edit geometry
  kernel without a confirmed mismatch.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours/tests/test_cpp_line_line_parity.rs`
- `E:/Coding/CavalierContours/include/cavc/intrlineseg2lineseg2.hpp`


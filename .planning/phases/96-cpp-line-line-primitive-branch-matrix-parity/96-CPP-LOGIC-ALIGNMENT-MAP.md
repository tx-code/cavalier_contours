# Phase 96: C++ Logic Alignment Map

This map captures next deep parity targets after line-line primitive
branch-matrix closure.

## Deepening Outcome

- Added executable expected-table parity for old C++ line-line primitive branch
  families:
  - `True` and `False` non-parallel outcomes
  - `None` parallel non-collinear outcome
  - `Coincident` overlap outcome
  - degenerate `point-point` and `point-segment` paths
- Parametric outputs are now explicitly asserted as parity evidence.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Build standalone line-circle primitive branch expected-table parity for `0/1/2` intersections and degenerate point branch | `cavalier_contours/tests/test_line_circle_intersect.rs` | Add parity tests first; only touch core if a confirmed mismatch appears. |
| P1 | Build standalone circle-circle primitive branch expected-table parity for `None`, tangent, two-intersects, and coincident families | `cavalier_contours/tests/test_circle_circle_intersect.rs` | Keep source-traceable expected values and bounded matrix size. |
| P2 | Expand polyline segment intersection parity only for uncovered endpoint-stickiness and overlap-order branches | `cavalier_contours/tests/test_pline_seg_intersect.rs` | No broad rewrite; target uncovered branches only. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/include/cavc/intrlineseg2lineseg2.hpp`
  - `E:/Coding/CavalierContours/include/cavc/intrlineseg2circle2.hpp`
  - `E:/Coding/CavalierContours/include/cavc/intrcircle2circle2.hpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_line_line_parity.rs`
  - `cavalier_contours/tests/test_line_circle_intersect.rs`
  - `cavalier_contours/tests/test_circle_circle_intersect.rs`


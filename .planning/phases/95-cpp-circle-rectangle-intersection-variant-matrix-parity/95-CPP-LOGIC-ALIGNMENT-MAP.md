# Phase 95: C++ Logic Alignment Map

This map captures next deep parity targets after circle/rectangle variant
matrix closure.

## Deepening Outcome

- Circle/rectangle intersection parity now includes:
  - baseline expected-table assertions (from Phase 94)
  - swapped-operand expected-table assertions (Phase 95)
  - operand-order and direction-variant matrix point-set invariance checks
- This reduces direction/order-specific blind spots on the same historical C++
  geometry.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Build standalone line-line primitive C++ branch expected-table parity cases (`None`, `True`, `Coincident`, `False`) | `cavalier_contours/tests/test_line_line_intersect.rs` | Add parity tests first; only touch core if a confirmed mismatch appears. |
| P1 | Build standalone line-circle and circle-circle primitive C++ branch expected-table parity cases (`0/1/2` intersects and coincident/tangent boundaries) | `cavalier_contours/tests/test_line_circle_intersect.rs`, `cavalier_contours/tests/test_circle_circle_intersect.rs` | Keep source-traceable expected values and bounded case matrices. |
| P2 | Expand polyline segment-intersection C++ parity matrix only for uncovered overlap/end-point stickiness branches | `cavalier_contours/tests/test_pline_seg_intersect.rs` | No broad rewrite; target uncovered branches only. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/include/cavc/intrlineseg2lineseg2.hpp`
  - `E:/Coding/CavalierContours/include/cavc/intrlineseg2circle2.hpp`
  - `E:/Coding/CavalierContours/include/cavc/intrcircle2circle2.hpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_offset_parity.rs`
  - `cavalier_contours/tests/test_line_line_intersect.rs`
  - `cavalier_contours/tests/test_line_circle_intersect.rs`
  - `cavalier_contours/tests/test_circle_circle_intersect.rs`


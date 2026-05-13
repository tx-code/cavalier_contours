# Phase 97: C++ Logic Alignment Map

This map captures next deep parity targets after line-circle primitive
branch-matrix closure.

## Deepening Outcome

- Added executable expected-table parity for old C++ line-circle primitive
  branch families:
  - degenerate-point on/off-circle
  - tangent
  - no-intersect
  - two-intersect including outside-segment solutions
- Parametric outputs (`t0`, `t1`) are now explicitly asserted as parity
  evidence.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Build standalone circle-circle primitive branch expected-table parity for `NoIntersect`, tangent, two-intersects, and overlapping/coincident families | `cavalier_contours/tests/test_circle_circle_intersect.rs` | Add parity tests first; only touch core if a confirmed mismatch appears. |
| P1 | Expand polyline segment-intersection parity for uncovered overlap-order and endpoint-stickiness branches | `cavalier_contours/tests/test_pline_seg_intersect.rs` | Keep scope bounded to uncovered branches; no broad rewrite. |
| P2 | Add bounded find-intersects collection-level parity probes only if primitive parity remains green | `cavalier_contours/src/polyline/internal/pline_intersects.rs`, `cavalier_contours/tests/*` | Trigger on real drift, not speculative churn. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/include/cavc/intrlineseg2circle2.hpp`
  - `E:/Coding/CavalierContours/include/cavc/intrcircle2circle2.hpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_line_circle_parity.rs`
  - `cavalier_contours/tests/test_circle_circle_intersect.rs`
  - `cavalier_contours/tests/test_pline_seg_intersect.rs`


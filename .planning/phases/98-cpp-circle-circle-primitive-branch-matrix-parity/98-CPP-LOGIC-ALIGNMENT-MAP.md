# Phase 98: C++ Logic Alignment Map

This map captures next deep parity targets after circle-circle primitive
branch-matrix closure.

## Deepening Outcome

- Added executable expected-table parity for old C++ circle-circle primitive
  branch families:
  - coincident
  - no-intersect (outside and inside)
  - tangent
  - two-intersects
  - near-tangent midpoint behavior
- Intersect point outputs are now explicitly asserted as parity evidence.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Expand polyline segment-intersection parity for uncovered overlap-order and endpoint-stickiness branches across line-line/line-arc/arc-arc cases | `cavalier_contours/tests/test_pline_seg_intersect.rs` | Add bounded parity tests first; edit core only if a confirmed mismatch appears. |
| P1 | Add bounded `find_intersects` collection-level parity probes around duplicate-filter and start/end-point attribution behavior | `cavalier_contours/src/polyline/internal/pline_intersects.rs`, `cavalier_contours/tests/*` | Trigger on concrete uncovered behavior, avoid speculative churn. |
| P2 | Extend historical C++ combine/offset-derived intersection fixtures only when direct source mapping exists | `cavalier_contours/tests/test_cpp_offset_parity.rs`, `cavalier_contours/tests/test_cpp_combine_parity.rs` | Keep source-traceable evidence and no-Clipper scope. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/include/cavc/intrcircle2circle2.hpp`
  - `E:/Coding/CavalierContours/include/cavc/plinesegment.hpp`
  - `E:/Coding/CavalierContours/include/cavc/polylineintersects.hpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_circle_circle_parity.rs`
  - `cavalier_contours/tests/test_pline_seg_intersect.rs`
  - `cavalier_contours/src/polyline/internal/pline_intersects.rs`


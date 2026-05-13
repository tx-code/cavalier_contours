# Phase 99: C++ Logic Alignment Map

This map captures next deep parity targets after pline segment overlap-order and
endpoint-stickiness branch closure.

## Deepening Outcome

- Added executable expected-case parity for old C++ `intrPlineSegs` branch
  families:
  - line-line overlap-order according to second segment direction
  - line-arc endpoint-stickiness path
  - arc-line endpoint-stickiness path
  - two-intersect ordering according to second segment direction in line-arc
    and arc-line paths
- Intersect point outputs are explicitly asserted as parity evidence.
- Added bounded collection-level parity for overlap-adjacent duplicate-filter
  behavior in `find_intersects`:
  - `overlap_endpoint_basic_intersect_deduplication`
- Added bounded collection-level parity for `skip_intr_at_end` endpoint-elision
  symmetry in open vs closed paths:
  - `skip_intr_at_end_open_pline1_uses_next_segment_index`
  - `skip_intr_at_end_closed_pline1_uses_next_segment_index`
  - `skip_intr_at_end_open_pline2_uses_next_segment_index`
  - `skip_intr_at_end_closed_pline2_uses_next_segment_index`
- Added bounded mixed line/arc overlap-adjacent collection-level parity probe:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication`
- Added closed/open asymmetry probe for mixed line/arc overlap-adjacent dedup:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_closed_pline1`
- Added complementary closed/open asymmetry probe where `pline2` is closed:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_closed_pline2`
- Added opposing-direction arc-overlap-adjacent collection-level parity probe:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication`
- Added opposing-direction arc-overlap-adjacent closed/open variant probes:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_closed_pline1`
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_closed_pline2`
- Added non-circle arc/arc-overlap-adjacent collection-level parity probe:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication`
- Added non-circle arc/arc-overlap-adjacent closed/open variant probes:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_closed_pline1`
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_closed_pline2`
- Added non-circle reversed-overlap-endpoint-order probe with adjacent-line flip:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip`
- Added non-circle reversed-overlap-endpoint-order closed/open variants that
  also pin expected closure-edge basic intersections:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect`
- Clarified bounded parity behavior for these variants: closure edges can
  produce additional real basic intersections that are independent of
  overlap-adjacent duplicate filtering.
- Added bounded wrap-around-adjacency endpoint-dedup probes that exercise
  `next_wrapping_index(last) == 0` without introducing independent closure-edge
  crossings:
  - `wrap_around_overlap_endpoint_deduplication_closed_pline1`
  - `wrap_around_overlap_endpoint_deduplication_closed_pline2`

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Extend wrap-around-adjacency endpoint-dedup probes from line-line to mixed line/arc and arc/arc non-circle closed shapes | `cavalier_contours/src/polyline/internal/pline_intersects.rs` | Keep source-traceable cases and isolate wrap-around dedup from independent closure crossings. |
| P2 | Extend collection-level parity to mixed arc/arc-overlap adjacency in non-circle closed shapes where direct old C++ mapping is available | `cavalier_contours/src/polyline/internal/pline_intersects.rs`, `cavalier_contours/tests/test_pline_seg_intersect.rs` | Keep source mapping explicit and bounded. |
| P2 | Extend combine/offset-derived intersection fixture parity only when direct C++ source mapping exists | `cavalier_contours/tests/test_cpp_combine_parity.rs`, `cavalier_contours/tests/test_cpp_offset_parity.rs` | Keep provenance explicit and no-Clipper. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/include/cavc/plinesegment.hpp`
  - `E:/Coding/CavalierContours/include/cavc/polylineintersects.hpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_pline_seg_intersect.rs`
  - `cavalier_contours/src/polyline/internal/pline_intersects.rs`

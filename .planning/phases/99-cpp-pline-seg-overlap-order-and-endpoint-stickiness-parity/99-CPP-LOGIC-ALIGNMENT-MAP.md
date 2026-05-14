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
- Added non-circle arc/arc-overlap-adjacent both-closed variant probe:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed`
- Added parameter-role flipped counterpart for the same bounded
  both-closed adjacent dedup probe:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_flipped_roles`
- Added non-circle reversed-overlap-endpoint-order probe with adjacent-line flip:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip`
- Added complementary open-path `arc1_reverse_dir` collection-level probe:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_with_adjacent_line_flip`
- This probe pins a bounded asymmetry: unlike the paired `arc2_reverse_dir`
  open-path case, an endpoint basic at `(3, 1)` can remain as an independent
  adjacent-line intersect.
- Added open-path counterpart where both arcs are reversed:
  - `non_circle_partial_arc_overlap_both_reverse_dir_with_adjacent_line_flip`
- In this bounded geometry, overlap ordering remains `(3, 1) -> (2, 0)` with
  no surviving basic intersects.
- Added non-circle reversed-overlap-endpoint-order both-closed variant with
  explicit independent closure-edge basics:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed`
- Added parameter-role flipped counterpart for the same bounded reversed
  endpoint-order + both-closed probe:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_flipped_roles`
- Added source-aligned primitive parity probes for non-circle partial-overlap
  where both arcs are reversed:
  - `arc_arc_partial_overlap_both_reverse_dir`
  - `arc_arc_partial_overlap_both_reverse_dir_flipped`
- These probes lock an ordering nuance from old C++ behavior: in this bounded
  geometry, swapping parameter order does not flip overlap endpoint ordering
  when both arcs are reversed.
- Added bounded closed-shape collection-level counterpart for `arc1_reverse_dir`
  partial overlap:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed`
- Added parameter-role flipped counterpart for the same bounded
  `arc1_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_flipped_roles`
- Added bounded closed-shape collection-level counterpart for `both_reverse_dir`
  partial overlap:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed`
- Added parameter-role flipped counterpart for the same bounded
  `both_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_flipped_roles`
- Added bounded closed-shape collection-level counterpart for `arc2_reverse_dir`
  partial overlap:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed`
- Added parameter-role flipped counterpart for the same bounded
  `arc2_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_flipped_roles`
- This closed-shape probe also pins a wrap-index nuance: a basic intersect at
  `(3, 1)` on `start_index2 = 2` remains independent from overlap-adjacent
  duplicate filtering for overlap `start_index2 = 0`.
- Together these closed-shape non-circle variants now pin a side-specific
  behavior split: `arc2_reverse_dir` can remain pure overlap (no basic
  intersects) in bounded both-closed geometry where `arc1_reverse_dir` and
  `both_reverse_dir` still surface an additional basic at overlap endpoint
  adjacency.
- Added non-circle reversed-overlap-endpoint-order closed/open variants that
  also pin expected closure-edge basic intersections:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect`
- Added parameter-role flipped counterpart for the closed-`pline1`
  closure-basic reversed-endpoint-order probe:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect_flipped_roles`
- Added parameter-role flipped counterpart for the closed-`pline2`
  closure-basic reversed-endpoint-order probe:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect_flipped_roles`
- Clarified bounded parity behavior for these variants: closure edges can
  produce additional real basic intersections that are independent of
  overlap-adjacent duplicate filtering.
- Added bounded wrap-around-adjacency endpoint-dedup probes that exercise
  `next_wrapping_index(last) == 0` without introducing independent closure-edge
  crossings:
  - `wrap_around_overlap_endpoint_deduplication_closed_pline1`
  - `wrap_around_overlap_endpoint_deduplication_closed_pline2`
- Added bounded mixed line/arc wrap-around-adjacency endpoint-dedup probes:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline1`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline2`
- Added bounded non-circle arc/arc wrap-around-adjacency endpoint-dedup probes
  for same and reversed endpoint ordering:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline1`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline1`
- Added exact parameter-role flipped counterpart for the bounded same-order
  closed-`pline1` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline1_flipped_roles`
- Added exact parameter-role flipped counterpart for the bounded reversed-order
  closed-`pline1` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline1_flipped_roles`
- Added complementary non-circle arc/arc wrap-around-adjacency probes where
  `pline2` is closed:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline2`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline2`
- Added exact parameter-role flipped counterpart for the bounded same-order
  closed-`pline2` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline2_flipped_roles`
- Added exact parameter-role flipped counterpart for the bounded reversed-order
  closed-`pline2` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline2_flipped_roles`
- Added non-circle arc/arc wrap-around closure-edge variants where additional
  real basic intersects are expected and asserted:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline1_with_closure_basic_intersect`
  - `wrap_around_non_circle_arc_overlap_reversed_order_closed_pline1_with_closure_basic_intersect`
- Added complementary closure-edge variants where `pline2` is closed and
  independent basics are explicitly asserted:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect`
- Added closed-`pline2` closure-edge counterpart that isolates open-side
  reversal while keeping closed-side arc orientation non-reversed:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect`
- Added closed-side reversed closure-edge counterpart on the closed-`pline1`
  surface:
  - `wrap_around_non_circle_arc_overlap_closed_side_reversed_closed_pline1_with_closure_basic_intersect`
- This additional probe pins side-specific ordering behavior: in this bounded
  configuration, open-side reversal alone does not flip overlap endpoint
  ordering when closed-side orientation remains non-reversed.
- Added role-flip symmetry probe on the closed-side reversed closure-edge
  geometry to assert start-index role swapping under parameter inversion while
  preserving bounded overlap endpoint ordering:
  - `wrap_around_non_circle_arc_overlap_closed_side_reversed_closure_basic_role_flip_symmetry`
- Added role-flip symmetry probe on the open-side reversed + normal closed-side
  closure-edge geometry; this bounded case keeps index-role inversion but
  swaps overlap endpoint ordering under parameter inversion:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_normal_closed_side_role_flip_symmetry`
- Added role-flip symmetry probe on the open-side-reversed + closed-side-reversed
  closure-edge geometry:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_side_reversed_role_flip_symmetry`
  - in this bounded case, role inversion keeps overlap endpoint ordering while
    swapping index roles.
- Together, these closure-edge probes now cover bounded both-side reversed
  ordering symmetry with explicit source-traceable geometry.
- Confirmed closed-`pline2` closure-edge counterpart with reversed overlap
  endpoint ordering (`point1 = (3, 1)`, `point2 = (2, 0)`) while still
  asserting one independent basic intersect.
- Added offset-derived circle/rectangle intersection role-flip symmetry matrix
  parity across orientation reversals:
  - `cpp_circle_rectangle_intersection_role_flip_symmetry_matrix_parity`
  - verifies AB/BA role inversion (`start_index1 <-> start_index2`) while
    preserving the expected four C++-aligned intersection points.
- Added combine-derived circle/rectangle commutative role-flip symmetry matrix
  parity across orientation reversals for C++-mapped operations:
  - `cpp_circle_rectangle_commutative_role_flip_matrix_parity`
  - verifies AB and BA both match C++-mapped geometry for `OR`/`AND`/`XOR`
    and match each other under role inversion.
- Added source-aligned input immutability parity for circle/rectangle combine
  modes (mirrors old C++ no-modify suite coverage):
  - `cpp_circle_rectangle_combine_does_not_modify_input`
  - verifies `OR`/`NOT`/`AND`/`XOR` do not mutate either input polyline.
- Added source-aligned self-combine reverse/mixed-direction invariants from old
  C++ combine test coverage:
  - `cpp_combine_with_self_reverse_mix_invariants`
  - verifies self `UNION`/`INTERSECT` returns self for forward and reversed
    orientation while keeping `neg_plines` empty, and self `EXCLUDE`/`XOR`
    keeps both `pos_plines` and `neg_plines` empty for
    `fwd/fwd`, `rev/rev`, `rev/fwd`, and `fwd/rev`.
- Added source-aligned `subtracted`-surface parity for combine fixtures:
  - `cpp_combine_expected_subtracted_empty_parity`
  - verifies Rust `neg_plines` remains empty across the C++-mapped
  circle/rectangle and coincident combine matrices where old
  `expectedSubtracted` is empty.
- Added source-aligned primitive parity probes for coincident-arc single-endpoint
  touch branches in old C++ `intrPlineSegs`:
  - `arc_arc_coincident_touch_only_at_arc1_start`
  - `arc_arc_coincident_touch_only_at_arc2_start`
  - verifies one-intersect endpoint behavior when only one coincident-arc end
    touches (no overlap span).
- Added source-aligned primitive parity probe for coincident-arc disjoint-sweep
  branch in old C++ `intrPlineSegs`:
  - `arc_arc_coincident_disjoint_sweeps_no_intersect`
  - verifies no-intersect behavior when coincident arcs have non-overlapping
    sweeps (including parameter inversion and reversed second-arc direction).
- Added collection-level no-intersect parity guard for the same coincident-arc
  disjoint-sweep branch:
  - `coincident_arc_disjoint_sweeps_no_intersects_collection_level`
  - verifies `find_intersects` does not surface basic/overlap entries for this
    branch (including reversed second-arc direction).

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Extend collection-level parity to mixed arc/arc-overlap adjacency in non-circle closed shapes where direct old C++ mapping is available | `cavalier_contours/src/polyline/internal/pline_intersects.rs`, `cavalier_contours/tests/test_pline_seg_intersect.rs` | Keep source mapping explicit and bounded. |
| P2 | Extend combine/offset-derived intersection fixture parity only when direct C++ source mapping exists | `cavalier_contours/tests/test_cpp_combine_parity.rs`, `cavalier_contours/tests/test_cpp_offset_parity.rs` | Keep provenance explicit and no-Clipper. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/include/cavc/plinesegment.hpp`
  - `E:/Coding/CavalierContours/include/cavc/polylineintersects.hpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_pline_seg_intersect.rs`
  - `cavalier_contours/src/polyline/internal/pline_intersects.rs`

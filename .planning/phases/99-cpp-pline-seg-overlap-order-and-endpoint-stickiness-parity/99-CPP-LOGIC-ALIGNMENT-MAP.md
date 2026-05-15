# Phase 99: C++ Logic Alignment Map

This map captures next deep parity targets after pline segment overlap-order and
endpoint-stickiness branch closure.

## Deepening Outcome

- Added segment-level parity probes for circle-circle two-intersection paths
  where arc sweep filtering retains exactly one point:
  - `arc_arc_two_circle_intersections_only_one_in_sweeps`
  - `arc_arc_two_circle_intersections_only_one_in_sweeps_flipped_roles`
  - `arc_arc_two_circle_intersections_only_one_in_sweeps_reversed_dirs`
  - `arc_arc_two_circle_intersections_only_one_in_sweeps_reversed_dirs_flipped_roles`
  - verifies old C++ `intrPlineSegs` `Circle2Circle2IntrType::TwoIntersects`
    branch emits `OneIntersect` when only one of the two geometric circle
    intersections lies within both arc sweeps, including role inversion and
    complementary arc-direction orientation.
- Added collection-level explicit closure-basic expected-case probes for
  open-side-reversed + closed-side-reversed wrap-around geometry on closed
  `pline2`:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closed_side_reversed_closure_basic_intersect`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closed_side_reversed_closure_basic_intersect_flipped_roles`
  - verifies one overlap with reversed endpoint ordering `(3, 1) -> (2, 0)` and
    one independent basic intersect at `(2, 2)` with explicit segment indexes
    under parameter-role inversion.
- Added collection-level role-flip symmetry guards for bounded mixed arc +
  adjacent-line overlap families in non-circle open paths:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_with_adjacent_line_flip_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_with_adjacent_line_flip_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_both_reverse_dir_with_adjacent_line_flip_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_both_reverse_dir_with_adjacent_line_flip_zero_length_lead_role_flip_symmetry`
  - verifies AB/BA role inversion preserves overlap/basic cardinality, preserves
    role-swapped start-index attribution, and keeps overlap endpoint sets
    stable (allowing direction-order reversal) in source-traceable
    `reversed endpoint order`, `arc1 reverse`, and `both reverse` branch
    geometries, including zero-length-lead non-zero-index counterparts for
    `reversed endpoint order`, `arc1 reverse`, and `both reverse`.
- Added collection-level role-flip symmetry guard for bounded both-closed
  reversed-endpoint-order mixed arc + adjacent-line overlap with zero-length
  lead index shift:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - verifies AB/BA role inversion preserves overlap/basic structure and role-swapped
  start-index mapping after non-zero index shift, while keeping overlap endpoint
  ordering behavior (`AB point1/point2 == BA point2/point1`) source-aligned in
  non-rotated and closed-pline1/2-rotated + zero-length-lead geometries.
- Added collection-level role-flip symmetry guard for bounded both-closed
  `arc1 reverse` mixed arc + adjacent-line overlap with zero-length lead index
  shift:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - verifies AB/BA role inversion preserves one-basic + one-overlap structure,
    preserves role-swapped basic/overlap start-index mapping with non-zero
    indexes, and preserves source-aligned overlap endpoint-order reversal
    (`AB point1/point2 == BA point2/point1`) in non-rotated,
    start-index-rotated, and closed-pline1/2-rotated + zero-length-lead
    geometries.
- Added collection-level role-flip symmetry guard for bounded both-closed
  `both reverse` mixed arc + adjacent-line overlap with zero-length lead index
  shift:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - verifies AB/BA role inversion preserves one-basic + one-overlap structure,
    preserves role-swapped basic/overlap start-index mapping with non-zero
    indexes, and preserves source-aligned overlap endpoint-order stability
    (`AB point1/point2 == BA point1/point2`) in non-rotated,
    start-index-rotated, and closed-pline1/2-rotated + zero-length-lead
    geometries.
- Added collection-level role-flip symmetry guard for bounded both-closed
  `arc2 reverse` mixed arc + adjacent-line overlap with zero-length lead index
  shift:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - verifies AB/BA role inversion preserves no-basic + one-overlap structure,
  preserves role-swapped overlap start-index mapping with non-zero indexes,
    and preserves source-aligned overlap endpoint-order reversal
    (`AB point1/point2 == BA point2/point1`) in non-rotated, start-index-rotated,
    and closed-pline1/2-rotated + zero-length-lead geometries.
- Added segment-level parity probes for old C++ `intrPlineSegs`
  `Circle2Circle2IntrType::TwoIntersects` sweep-classification boundaries:
  - `arc_arc_two_circle_intersections_both_in_sweeps`
  - `arc_arc_two_circle_intersections_both_in_sweeps_flipped_roles`
  - `arc_arc_two_circle_intersections_both_outside_sweeps_no_intersect`
  - `arc_arc_two_circle_intersections_both_outside_sweeps_no_intersect_flipped_roles`
  - verifies explicit `TwoIntersects` output when both circle intersection
    points lie within both arc sweeps, and explicit `NoIntersect` output when
    neither point lies within both sweeps, including parameter-role inversion.
- Added segment-level parity probes for old C++ `intrPlineSegs`
  `Circle2Circle2IntrType::OneIntersect` (tangent) sweep filtering boundaries:
  - `arc_arc_circle_tangent_in_sweeps`
  - `arc_arc_circle_tangent_in_sweeps_flipped_roles`
  - `arc_arc_circle_tangent_outside_sweeps_no_intersect`
  - `arc_arc_circle_tangent_outside_sweeps_no_intersect_flipped_roles`
  - verifies explicit tangent intersection retention when the tangent point is
    inside both arc sweeps, and explicit filtering to `NoIntersect` when the
    tangent point falls outside sweep coverage, including parameter-role
    inversion.
- Added segment-level parity probes for old C++ `intrPlineSegs`
  `processLineArcIntr` `numIntersects == 1` (line-circle tangent) boundaries:
  - `line_arc_tangent_in_sweep`
  - `line_arc_tangent_outside_sweep_no_intersect`
  - `arc_line_tangent_in_sweep`
  - `arc_line_tangent_outside_sweep_no_intersect`
  - verifies tangent retention when line-circle tangent point lies in arc
    sweep, and filtering to `NoIntersect` when tangent lies outside sweep,
    across both line-arc and arc-line dispatch paths.
- Added segment-level parity probes for old C++ `intrPlineSegs`
  `processLineArcIntr` `numIntersects == 2` sweep-classification boundaries
  without endpoint-stickiness substitution:
  - `line_arc_two_intersections_only_one_in_sweep_non_sticky`
  - `arc_line_two_intersections_only_one_in_sweep_non_sticky`
  - `line_arc_two_intersections_both_outside_sweep_no_intersect`
  - `arc_line_two_intersections_both_outside_sweep_no_intersect`
  - verifies one-in-sweep filtering to `OneIntersect` and both-outside
    filtering to `NoIntersect` across both line-arc and arc-line dispatch
    paths when line endpoints are not arc endpoints.
- Added segment-level parity probes for old C++ `intrPlineSegs` line-line
  non-overlap branch split:
  - `line_line_true_intersect`
  - `line_line_false_intersect_outside_segments_no_intersect`
  - `line_line_none_parallel_no_intersect`
  - verifies explicit `LineSeg2LineSeg2IntrType::True -> OneIntersect` mapping
    and explicit `LineSeg2LineSeg2IntrType::False/None -> NoIntersect`
    mappings.
- Added collection-level guards for the same line-line non-overlap split in
  `find_intersects` with overlapping AABB candidates:
  - `line_line_false_intersection_no_intersects_collection_level`
  - `line_line_none_parallel_no_intersects_collection_level`
  - `line_line_false_intersection_no_intersects_collection_level_nonzero_indexes`
  - `line_line_false_intersection_no_intersects_collection_level_nonzero_indexes_flipped_roles`
  - `line_line_none_parallel_no_intersects_collection_level_nonzero_indexes`
  - `line_line_none_parallel_no_intersects_collection_level_nonzero_indexes_flipped_roles`
  - `line_line_true_intersection_collection_level`
  - `line_line_true_intersection_collection_level_flipped_roles`
  - `line_line_true_intersection_collection_level_nonzero_indexes`
  - `line_line_true_intersection_collection_level_nonzero_indexes_flipped_roles`
  - verifies no basic/overlap emission when segment-level line-line result is
    `False` or `None` (including non-zero index shifts and role inversion), and
    verifies one basic/no-overlap emission when segment-level line-line result
    is `True`, including stable non-zero segment start-index attribution under
    parameter-role inversion.
- Replaced the remaining index-0-shift canonical-name wrapper aliases with
  explicit assertion-backed probes, eliminating wrapper-style alias indirection
  in `find_intersects_tests`:
  - `all_self_intersects_basic_include_overlapping_coincident_arc_overlap_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - `all_self_intersects_basic_include_overlapping_coincident_arc_reversed_second_segment_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - `all_self_intersects_basic_single_intersect_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - `all_self_intersects_basic_overlap_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - `all_self_intersects_basic_two_intersects_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - `non_local_single_intersect_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - `non_local_overlap_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - `non_local_coincident_arc_overlap_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - `non_local_two_intersects_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - `non_local_coincident_arc_overlap_reversed_second_segment_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
- Replaced canonical-name closed-side-rotation wrapper aliases with explicit
  assertion-backed probes in wrap-around overlap dedup families:
  - `wrap_around_overlap_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
- Replaced nested canonical-name wrapper aliases with explicit assertion-backed
  probes for open-side-reversed + normal-closed-side closure-basic role-flip
  families:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_role_flip_symmetry`
- Replaced canonical-name wrapper aliases with explicit assertion-backed probes
  for closure-basic `intersect` naming parity in bounded role-flip families:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect_start_index_rotation_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect_start_index_rotation_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect_start_index_rotation_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect_start_index_rotation_role_flip_symmetry`
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
- Added source-aligned wrap-join branch parity for overlap-slice stitching:
  - `empty_overlapping_intersects_returns_no_slices`
  - `wrap_join_at_pline2_zero_vertex_increments_end_index_offset`
  - `wrap_join_away_from_pline2_zero_vertex_keeps_end_index_offset`
  - `non_connected_overlapping_slices_remain_separate`
  - verifies `sort_and_join_overlapping_intersects` joins last+first slices
    when they connect at wrap boundary, including both:
    - empty overlap input returns no slices,
    - connection exactly at `pline2[0]` (increments `end_index_offset`)
    - connection away from `pline2[0]` (keeps `end_index_offset` unchanged),
    - and preserves separate slices when overlaps are not end/start connected.
- Strengthened full-loop overlap-slice parity probes for coincident circles:
  - `overlapping_circles_same_dir`
  - `overlapping_circles_same_dir_flipped_index`
  - `overlapping_circles_opposing_dir`
  - `unsorted_same_segment_overlaps_are_sorted_and_joined`
  - `unsorted_multi_segment_overlaps_are_sorted_and_joined`
  - `single_segment_overlap_opposing_direction_marks_opposing`
  - `same_segment_join_opposing_direction_marks_opposing`
  - `multi_segment_join_opposing_direction_marks_opposing`
  - `find_intersects_overlapping_points_follow_second_segment_direction`
  - `full_closed_line_overlap_loop_uses_vertex_count_minus_one_offset`
  - verifies `sort_and_join_overlapping_intersects` marks full overlap as
    `is_loop=true` with stable `start_index` / `end_index_offset` view data in
    both same-direction and opposing-direction paths, and still sorts+joins
    out-of-order overlaps on the same segment and across multiple segments;
    plus non-loop single-segment overlap and same-segment joined overlap set
    `opposing_directions=true` when segment directions are opposite, including
    both joined-on-same-segment and joined-across-multiple-segments branches;
    and confirms `find_intersects` overlap endpoints respect second-segment
    direction ordering invariant (`point1` closest to segment start), including
    loop offset behavior on closed multi-segment line overlaps (`vc - 1`).
- Added bounded collection-level parity for `skip_intr_at_end` endpoint-elision
  symmetry in open vs closed paths:
  - `skip_intr_at_end_open_pline1_uses_next_segment_index`
  - `skip_intr_at_end_closed_pline1_uses_next_segment_index`
  - `skip_intr_at_end_open_pline2_uses_next_segment_index`
  - `skip_intr_at_end_closed_pline2_uses_next_segment_index`
- Added source-aligned local-self singularity parity probe for adjacent
  duplicate vertexes:
  - `adjacent_duplicate_vertex_reports_overlapping_singularity`
  - verifies local self-intersect reports an overlapping intersect at the
    duplicate point instead of emitting a basic intersect.
- Added source-aligned local-self `vc == 2 && is_closed` overlap probe for
  zero-bulge two-vertex line geometry:
  - `closed_two_vertex_line_reports_self_overlap`
  - verifies the early local-self overlap branch emits one overlapping
    intersect spanning the closed two-vertex segment.
- Added source-aligned local-self singularity parity probe on closed
  wrap-around adjacency (`last -> first` local pass):
  - `closed_wraparound_duplicate_vertex_reports_overlapping_singularity`
  - verifies no basic intersects and two overlapping intersects: one from the
    adjacent collinear-reverse segment pair and one from the wrap-around
    singularity pass, with stable start-index mapping.
- Added source-aligned global-self non-local overlap parity probe:
  - `non_local_collinear_overlap_reports_overlapping_intersect`
  - verifies `visit_global_self_intersects` emits an overlapping intersect for
    non-adjacent collinear segments (`start_index` pair `0/6`) with overlap
    endpoints `[1, 0]` and `[2, 0]`.
- Added source-aligned global-self non-local coincident-arc overlap parity
  probe:
  - `non_local_coincident_arc_overlap_reports_overlapping_intersect`
  - verifies `visit_global_self_intersects` emits an overlapping intersect for
  non-adjacent coincident arcs (`start_index` pair `0/4`) with overlap
  endpoints `[2, 0]` and `[3, 1]`.
- Added re-parameterization counterpart for the same global-self
  coincident-arc overlap branch:
  - `non_local_coincident_arc_overlap_reports_overlapping_intersect_with_zero_length_lead_segment`
  - verifies prepending a zero-length lead segment (index shift) preserves
    overlap detection and endpoint output for shifted pair (`start_index` pair
    `1/5`).
- Added non-zero-index counterpart for the same global-self coincident-arc
  overlap branch:
  - `non_local_coincident_arc_overlap_nonzero_index_reports_overlapping_intersect`
  - verifies overlap detection and endpoint output remain stable with non-zero
    segment indexes (`start_index` pair `1/5`).
- Added re-parameterization counterpart for the same non-zero-index
  coincident-arc overlap branch:
  - `non_local_coincident_arc_overlap_nonzero_index_reports_overlapping_intersect_with_zero_length_lead_segment`
  - verifies prepending a zero-length lead segment (index shift) preserves
    overlap detection and endpoint output for shifted pair (`start_index` pair
    `3/7`).
- Added reversed-second-segment counterpart for the same global-self
  coincident-arc overlap branch:
  - `non_local_coincident_arc_overlap_reversed_second_segment_ordering`
  - verifies overlap endpoints follow second-segment direction (`[3, 1] -> [2,
    0]`) with stable non-local pair mapping.
- Added index-0 re-parameterization counterpart for the same reversed-
  second-segment overlap-order branch:
  - `non_local_coincident_arc_overlap_reversed_second_segment_ordering_with_zero_length_lead_segment_index0_shift`
  - verifies prepending a zero-length lead segment (index shift) preserves
    reversed-second-segment overlap endpoint ordering for shifted pair
    (`start_index` pair `1/5`).
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  non-local coincident-arc overlap branch:
  - `all_self_intersects_basic_include_overlapping_coincident_arc_overlap_ordering`
  - `all_self_intersects_basic_include_overlapping_coincident_arc_overlap_ordering_nonzero_indexes`
  - `all_self_intersects_basic_include_overlapping_coincident_arc_overlap_ordering_with_zero_length_lead_segment`
  - `all_self_intersects_basic_include_overlapping_coincident_arc_overlap_ordering_with_zero_length_lead_segment_index0_shift`
  - verifies `include_overlapping=false` emits no basics for the overlap pair,
  while `include_overlapping=true` emits exactly two overlap endpoints for
    the targeted pair across index-0, non-zero, and zero-length-lead
    re-parameterization paths (including index-0 zero-length shift).
- Added explicit global-self shared-endpoint skip probe:
  - `non_local_shared_end_point_pair_is_skipped`
  - `non_local_shared_end_point_pair_is_skipped_with_zero_length_lead_segment`
  - verifies non-local segment pairs that intersect at a common end point are
    filtered by `skip_intr_at_end` for global-self basic output, including a
    zero-length-lead re-parameterization shift.
- Added index-0 re-parameterization counterpart for the same global-self
  shared-endpoint skip probe:
  - `non_local_shared_end_point_pair_is_skipped_with_zero_length_lead_segment_index0_shift`
  - verifies prepending a zero-length lead segment (index shift) keeps shifted
    pair (`start_index` pair `1/5`) filtered.
- Added global-self `TwoIntersects` + shared-endpoint skip counterpart:
  - `non_local_two_intersects_shared_end_point_filters_one_point`
  - verifies when one of two intersection points is the shared segment end,
    only the non-shared point is emitted in global-self basic output.
- Added non-zero-index counterparts for the same global-self skip branches:
  - `non_local_shared_end_point_pair_is_skipped_nonzero_indexes`
  - `non_local_two_intersects_shared_end_point_filters_one_point_nonzero_indexes`
  - verifies shared-endpoint skip behavior remains stable away from index-0
    segment pairs.
- Added re-parameterization counterpart for the same global-self
  `TwoIntersects` shared-end filter path using a zero-length lead segment:
  - `non_local_two_intersects_shared_end_filters_one_point_with_zero_length_lead_segment`
  - verifies shared-end filtering still retains only `(-1,0)` after index shift
    caused by a leading degenerate segment.
- Added index-0 re-parameterization counterpart for the same global-self
  `TwoIntersects` shared-end filter path:
  - `non_local_two_intersects_shared_end_filters_one_point_with_zero_length_lead_segment_index0_shift`
  - verifies prepending a zero-length lead segment (index shift) still retains
    only `(-1,0)` for shifted pair (`start_index` pair `1/5`).
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  shared-endpoint skip path:
  - `all_self_intersects_basic_shared_end_point_pair_is_skipped`
  - `all_self_intersects_basic_shared_end_point_pair_is_skipped_nonzero_indexes`
  - `all_self_intersects_basic_shared_end_point_pair_is_skipped_with_zero_length_lead_segment`
  - `all_self_intersects_basic_shared_end_point_pair_is_skipped_with_zero_length_lead_segment_index0_shift`
  - verifies both `include_overlapping=false/true` keep shared-end pair output
  filtered in API-level basics across index-0/non-zero index pairs,
    including the zero-length-lead re-parameterization shift (including
    index-0 zero-length shift).
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  `TwoIntersects` shared-end filter path:
  - `all_self_intersects_basic_two_intersects_shared_end_filters_one_point`
  - `all_self_intersects_basic_two_intersects_shared_end_filters_one_point_nonzero_indexes`
  - `all_self_intersects_basic_two_intersects_shared_end_filters_one_point_with_zero_length_lead_segment`
  - `all_self_intersects_basic_two_intersects_shared_end_filters_one_point_with_zero_length_lead_segment_index0_shift`
  - verifies both `include_overlapping=false/true` retain only the non-shared
  point for the targeted pair and do not re-introduce the filtered shared-end
    point across index-0/non-zero index pairs, including the zero-length-lead
    re-parameterization shift (including index-0 zero-length shift).
- Added single-intersect branch counterparts where the intersect is at one
  segment end but not both segment ends:
  - `non_local_one_intersect_at_single_segment_end_is_kept`
  - `non_local_one_intersect_at_single_segment_end_is_kept_nonzero_indexes`
  - `non_local_one_intersect_at_single_segment_end_is_kept_with_zero_length_lead_segment`
  - verifies `skip_intr_at_end` does not filter one-intersect points unless the
    endpoint condition is simultaneously true for both tested segments,
    including a zero-length-lead re-parameterization shift.
- Added index-0 re-parameterization counterpart for the same single-intersect
  branch:
  - `non_local_one_intersect_at_single_segment_end_is_kept_with_zero_length_lead_segment_index0_shift`
  - verifies prepending a zero-length lead segment (index shift) still keeps
    one retained point for shifted pair (`start_index` pair `1/5`).
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  single-intersect-at-one-end branch:
  - `all_self_intersects_basic_one_intersect_at_single_segment_end_is_kept`
  - `all_self_intersects_basic_one_intersect_at_single_segment_end_is_kept_nonzero_indexes`
  - `all_self_intersects_basic_one_intersect_at_single_segment_end_is_kept_with_zero_length_lead_segment`
  - `all_self_intersects_basic_one_intersect_at_single_segment_end_is_kept_with_zero_length_lead_segment_index0_shift`
  - verifies both `include_overlapping=false/true` keep one retained basic
    point for the targeted pair across index-0/non-zero and zero-length-lead
    re-parameterization variants (including index-0 zero-length shift).
- Added overlap-branch shared-end boundary counterparts where the shared end
  appears on overlap `point2`:
  - `non_local_overlap_with_shared_end_on_point2_is_kept`
  - `non_local_overlap_with_shared_end_on_point2_is_kept_nonzero_indexes`
  - `non_local_overlap_with_shared_end_on_point2_is_kept_with_zero_length_lead_segment`
  - verifies these overlaps remain in output with ordering `[3,0] -> [4,0]`,
  i.e. the overlap branch filters on `point1` only for this boundary shape,
  including a zero-length-lead re-parameterization shift.
- Added index-0 re-parameterization counterpart for the same overlap
  shared-end-on-point2 boundary:
  - `non_local_overlap_with_shared_end_on_point2_is_kept_with_zero_length_lead_segment_index0_shift`
  - verifies prepending a zero-length lead segment (index shift) keeps overlap
    ordering `[3,0] -> [4,0]` for shifted pair (`start_index` pair `1/6`).
- Added overlap-branch shared-end boundary counterparts where the shared end
  appears on overlap `point1` but is not both segment ends:
  - `non_local_overlap_with_shared_end_on_point1_but_not_both_ends_is_kept`
  - `non_local_overlap_with_shared_end_on_point1_but_not_both_ends_is_kept_nonzero_indexes`
  - `non_local_overlap_with_shared_end_on_point1_but_not_both_ends_is_kept_with_zero_length_lead_segment`
  - verifies these overlaps remain in output with ordering `[4,0] -> [3,0]`
    when the shared endpoint condition is not simultaneously true for both
    segment ends, including a zero-length-lead re-parameterization shift.
- Added index-0 re-parameterization counterpart for the same overlap
  shared-end-on-point1 boundary:
  - `non_local_overlap_with_shared_end_on_point1_but_not_both_ends_is_kept_with_zero_length_lead_segment_index0_shift`
  - verifies prepending a zero-length lead segment (index shift) keeps overlap
    ordering `[4,0] -> [3,0]` for shifted pair (`start_index` pair `1/6`).
- Added zero-length shared-end boundary counterparts for global-self
  non-local pairs:
  - `non_local_zero_length_shared_end_pair_is_skipped`
  - `non_local_zero_length_shared_end_pair_is_skipped_nonzero_indexes`
  - `non_local_zero_length_shared_end_pair_is_skipped_with_zero_length_lead_segment`
  - verifies when both segment ends coincide at the same point and one segment
  is zero-length at that point, the pair is filtered (no basic and no overlap
  output for the targeted pair), including a zero-length-lead
    re-parameterization shift.
- Added index-0 re-parameterization counterpart for the same zero-length
  shared-end boundary:
  - `non_local_zero_length_shared_end_pair_is_skipped_with_zero_length_lead_segment_index0_shift`
  - verifies prepending a zero-length lead segment (index shift) keeps shifted
    pair (`start_index` pair `1/6`) filtered in both basic and overlap output.
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  overlap/shared-end boundaries:
  - `all_self_intersects_basic_include_overlapping_keeps_point2_shared_end_overlap_pair`
  - `all_self_intersects_basic_include_overlapping_keeps_point2_shared_end_overlap_pair_with_zero_length_lead_segment`
  - `all_self_intersects_basic_include_overlapping_keeps_point2_shared_end_overlap_pair_with_zero_length_lead_segment_index0_shift`
  - `all_self_intersects_basic_include_overlapping_keeps_point1_shared_end_overlap_pair`
  - `all_self_intersects_basic_include_overlapping_keeps_point1_shared_end_overlap_pair_with_zero_length_lead_segment`
  - `all_self_intersects_basic_include_overlapping_keeps_point1_shared_end_overlap_pair_with_zero_length_lead_segment_index0_shift`
  - `all_self_intersects_basic_include_overlapping_skips_zero_length_shared_end_pair`
  - `all_self_intersects_basic_include_overlapping_skips_zero_length_shared_end_pair_nonzero_indexes`
  - `all_self_intersects_basic_include_overlapping_skips_zero_length_shared_end_pair_with_zero_length_lead_segment`
  - `all_self_intersects_basic_include_overlapping_skips_zero_length_shared_end_pair_with_zero_length_lead_segment_index0_shift`
  - verifies `include_overlapping=true` emits both overlap endpoints only when
  the global-self overlap pair survives filtering, and does not re-introduce
  endpoints for filtered zero-length shared-end pairs, including a
    zero-length-lead re-parameterization shift (including index-0 zero-length
    shifts for shared-end-on-point2, shared-end-on-point1, and zero-length
    shared-end branches).
- Added global-self `TwoIntersects` positive-path counterpart:
  - `non_local_two_intersects_keeps_both_points_when_not_shared_end`
  - verifies both intersection points are emitted when neither point satisfies
    the shared-endpoint skip condition.
- Added index-0 re-parameterization counterpart for the same global-self
  `TwoIntersects` positive path:
  - `non_local_two_intersects_keeps_both_points_when_not_shared_end_with_zero_length_lead_segment_index0_shift`
  - verifies prepending a zero-length lead segment (index shift) still retains
    both points for shifted pair (`start_index` pair `1/5`).
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  `TwoIntersects` positive path:
  - `all_self_intersects_basic_two_intersects_keeps_both_points_when_not_shared_end`
  - `all_self_intersects_basic_two_intersects_keeps_both_points_when_not_shared_end_nonzero_indexes`
  - `all_self_intersects_basic_two_intersects_keeps_both_points_when_not_shared_end_with_zero_length_lead_segment`
  - `all_self_intersects_basic_two_intersects_keeps_both_points_when_not_shared_end_with_zero_length_lead_segment_index0_shift`
  - verifies both `include_overlapping=false/true` retain both intersection
    points for targeted pairs across index-0/non-zero and zero-length-lead
    re-parameterization variants (including index-0 zero-length shift).
- Added explicit global-self visited-pair dedup probe:
  - `non_local_single_intersect_pair_is_not_duplicated`
  - verifies a non-local crossing pair contributes one basic intersect even
    when reverse pair traversal is reachable via the spatial index.
- Added overlap-branch counterpart for the same global-self visited-pair dedup:
  - `non_local_overlap_pair_is_not_duplicated`
  - verifies a non-local overlapping pair contributes one overlap entry even
    when reverse pair traversal is reachable.
- Added non-zero-index counterparts for the same global-self visited-pair dedup
  probes:
  - `non_local_single_intersect_pair_is_not_duplicated_nonzero_indexes`
  - `non_local_overlap_pair_is_not_duplicated_nonzero_indexes`
  - verifies visited-pair dedup behavior remains stable away from index-0
    segment pairs for both basic and overlap output paths.
- Added re-parameterization counterparts for the same global-self visited-pair
  dedup paths:
  - `non_local_single_intersect_pair_is_not_duplicated_with_zero_length_lead_segment`
  - `non_local_overlap_pair_is_not_duplicated_with_zero_length_lead_segment`
  - verifies prepending a zero-length lead segment (index shift) preserves one
    basic entry for shifted single-intersect pairs and one overlap entry for
    shifted overlap pairs.
- Added canonical-name alias counterpart for direct index-0-shift tracing of
  the same single-intersect dedup path:
  - `non_local_single_intersect_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - verifies parity evidence is explicitly discoverable under index-0-shift
    naming while reusing identical shifted geometry.
- Added canonical-name alias counterpart for direct index-0-shift tracing of
  the same overlap dedup path:
  - `non_local_overlap_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - verifies parity evidence is explicitly discoverable under index-0-shift
    naming while reusing identical shifted geometry.
- Added coincident-arc overlap counterparts for the same global-self
  visited-pair dedup overlap path:
  - `non_local_coincident_arc_overlap_pair_is_not_duplicated`
  - `non_local_coincident_arc_overlap_pair_is_not_duplicated_nonzero_indexes`
  - `non_local_coincident_arc_overlap_pair_is_not_duplicated_with_zero_length_lead_segment`
  - verifies coincident-arc overlap pairs also remain deduplicated (single
    overlap entry) across index-0, non-zero, and zero-length-lead
    re-parameterization variants.
- Added canonical-name alias counterpart for direct index-0-shift tracing of
  the same coincident-arc overlap dedup path:
  - `non_local_coincident_arc_overlap_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - verifies parity evidence is explicitly discoverable under index-0-shift
    naming while reusing identical shifted geometry.
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  visited-pair dedup paths:
  - `all_self_intersects_basic_single_intersect_pair_is_not_duplicated`
  - `all_self_intersects_basic_single_intersect_pair_is_not_duplicated_nonzero_indexes`
  - `all_self_intersects_basic_single_intersect_pair_is_not_duplicated_with_zero_length_lead_segment`
  - `all_self_intersects_basic_single_intersect_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - `all_self_intersects_basic_overlap_pair_is_not_duplicated`
  - `all_self_intersects_basic_overlap_pair_is_not_duplicated_nonzero_indexes`
  - `all_self_intersects_basic_overlap_pair_is_not_duplicated_with_zero_length_lead_segment`
  - `all_self_intersects_basic_overlap_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - verifies single-crossing pairs remain one basic entry under
  `include_overlapping=false/true`, and overlap pairs emit no basics when
  `include_overlapping=false` and exactly two overlap endpoints (no duplicate
  endpoint expansion) when `include_overlapping=true`, across index-0,
    non-zero, and zero-length-lead re-parameterization variants.
- Added canonical-name API-level alias counterpart for direct index-0-shift
  tracing of the same coincident-arc overlap dedup path:
  - `all_self_intersects_basic_include_overlapping_coincident_arc_overlap_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - verifies parity evidence is explicitly discoverable under index-0-shift
    naming while reusing identical shifted geometry.
- Added explicit `TwoIntersects` visited-pair dedup probe:
  - `non_local_two_intersects_pair_is_not_duplicated`
  - verifies a two-intersection pair contributes exactly two basic points (not
    duplicated via reverse pair traversal).
- Added non-zero-index counterpart for the same `TwoIntersects` visited-pair
  dedup probe:
  - `non_local_two_intersects_pair_is_not_duplicated_nonzero_indexes`
  - verifies two-point dedup behavior remains stable away from index-0 segment
    pairs.
- Added re-parameterization counterpart for the same global-self
  `TwoIntersects` visited-pair dedup path:
  - `non_local_two_intersects_pair_is_not_duplicated_with_zero_length_lead_segment`
  - verifies prepending a zero-length lead segment (index shift) preserves
    exactly two basic points for the shifted pair.
- Added canonical-name alias counterpart for direct index-0-shift tracing of
  the same global-self `TwoIntersects` dedup path:
  - `non_local_two_intersects_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - verifies parity evidence is explicitly discoverable under index-0-shift
    naming while reusing identical shifted geometry.
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  `TwoIntersects` visited-pair dedup path:
  - `all_self_intersects_basic_two_intersects_pair_is_not_duplicated`
  - `all_self_intersects_basic_two_intersects_pair_is_not_duplicated_nonzero_indexes`
  - `all_self_intersects_basic_two_intersects_pair_is_not_duplicated_with_zero_length_lead_segment`
  - `all_self_intersects_basic_two_intersects_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - verifies both `include_overlapping=false/true` preserve exactly two basic
  points for the targeted pair (no reverse-traversal duplication) across
    index-0, non-zero, and zero-length-lead re-parameterization variants.
- Added non-zero-index counterpart for global-self reversed-second-segment
  coincident-arc overlap ordering:
  - `non_local_coincident_arc_overlap_reversed_second_segment_ordering_nonzero_index`
  - verifies overlap endpoint ordering remains aligned to the reversed second
    segment direction on non-zero segment indexes.
- Added re-parameterization counterpart for the same non-zero-index
  reversed-second-segment overlap ordering:
  - `non_local_coincident_arc_overlap_reversed_second_segment_ordering_with_zero_length_lead_segment`
  - verifies prepending a zero-length lead segment (index shift) keeps overlap
    endpoint ordering aligned to reversed second-segment direction.
- Added pair-dedup counterpart for index-0 reversed-segment coincident-arc
  overlap branch:
  - `non_local_coincident_arc_overlap_reversed_second_segment_pair_is_not_duplicated`
  - verifies the index-0 reversed-overlap pair contributes one overlap entry
    (no reverse-traversal duplication).
- Added re-parameterization counterpart for index-0 reversed-segment overlap
  dedup branch:
  - `non_local_coincident_arc_overlap_reversed_second_segment_pair_is_not_duplicated_with_zero_length_lead_segment`
  - verifies prepending a zero-length lead segment (index shift) keeps the
    shifted reversed-overlap pair deduplicated as one entry.
- Added canonical-name alias counterpart for direct index-0-shift tracing of
  the same index-0 reversed-segment overlap dedup branch:
  - `non_local_coincident_arc_overlap_reversed_second_segment_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - verifies parity evidence is explicitly discoverable under index-0-shift
    naming while reusing identical shifted geometry.
- Added pair-dedup counterpart for the same non-zero-index reversed-segment
  coincident-arc overlap branch:
  - `non_local_coincident_arc_overlap_reversed_second_segment_nonzero_not_duplicated`
  - verifies the non-zero-index reversed-overlap pair contributes one overlap
    entry (no reverse-traversal duplication).
- Added re-parameterization counterpart for the same non-zero-index reversed
  overlap dedup branch:
  - `non_local_coincident_arc_overlap_reversed_second_segment_nonzero_not_duplicated_with_zero_length_lead_segment`
  - verifies prepending a zero-length lead segment (index shift) still keeps
    the shifted overlap pair deduplicated as one entry.
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  reversed-second-segment coincident-arc overlap path:
  - `all_self_intersects_basic_include_overlapping_coincident_arc_reversed_second_segment_ordering`
  - `all_self_intersects_basic_include_overlapping_coincident_arc_reversed_second_segment_ordering_nonzero_indexes`
  - `all_self_intersects_basic_include_overlapping_coincident_arc_reversed_second_segment_ordering_with_zero_length_lead_segment`
  - `all_self_intersects_basic_include_overlapping_coincident_arc_reversed_second_segment_ordering_with_zero_length_lead_segment_index0_shift`
  - verifies `include_overlapping=false` emits no basics for the overlap pair,
  while `include_overlapping=true` emits exactly two overlap endpoints for the
    targeted pair (no endpoint duplication) on index-0, non-zero, and
    zero-length-lead re-parameterization paths (including index-0 zero-length
    shift).
- Added canonical-name API-level alias counterpart for direct index-0-shift
  tracing of the same reversed-second-segment coincident-arc overlap dedup
  path:
  - `all_self_intersects_basic_include_overlapping_coincident_arc_reversed_second_segment_pair_is_not_duplicated_with_zero_length_lead_segment_index0_shift`
  - verifies parity evidence is explicitly discoverable under index-0-shift
    naming while reusing identical shifted geometry.
- Added index-0 + reversed + shared-end-anchor counterpart for global-self
  `TwoIntersects`:
  - `non_local_two_intersects_reversed_second_segment_shared_end_keeps_both`
  - verifies both points are retained when the shared intersection point is not
    simultaneously the end point of both segments under global-self skip logic.
- Added non-zero + reversed + shared-end-anchor counterpart for global-self
  `TwoIntersects`:
  - `non_local_two_intersects_reversed_second_segment_shared_end_keeps_both_nonzero`
  - verifies both points are retained when the shared intersection point is not
    simultaneously the end point of both segments under global-self skip logic.
- Added re-parameterization counterpart for the same non-zero + reversed
  shared-end-anchor `TwoIntersects` branch:
  - `non_local_two_intersects_reversed_second_segment_shared_end_keeps_both_with_zero_length_lead_segment`
  - verifies prepending a zero-length lead segment (index shift) still retains
    both points for the shifted target pair.
- Added index-0 re-parameterization counterpart for the same reversed
  shared-end-anchor `TwoIntersects` branch:
  - `non_local_two_intersects_reversed_second_segment_shared_end_keeps_both_with_zero_length_lead_segment_index0_shift`
  - verifies prepending a zero-length lead segment (index shift) still retains
    both points for shifted pair (`start_index` pair `1/5`).
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  reversed shared-end-anchor `TwoIntersects` path:
  - `all_self_intersects_basic_two_intersects_reversed_second_segment_shared_end_keeps_both`
  - `all_self_intersects_basic_two_intersects_reversed_second_segment_shared_end_keeps_both_nonzero`
  - `all_self_intersects_basic_two_intersects_reversed_second_segment_shared_end_keeps_both_with_zero_length_lead_segment`
  - `all_self_intersects_basic_two_intersects_reversed_second_segment_shared_end_keeps_both_with_zero_length_lead_segment_index0_shift`
  - verifies both `include_overlapping=false/true` retain both points for the
  targeted pair across index-0/non-zero paths, including the zero-length-lead
    re-parameterization shift (including index-0 zero-length shift).
- Added complementary index-0/non-zero + reversed + no-shared-end counterparts
  for global-self `TwoIntersects`:
  - `non_local_two_intersects_reversed_second_segment_keeps_both`
  - `non_local_two_intersects_reversed_second_segment_keeps_both_nonzero`
  - verifies both points are retained when the reversed second-segment path has
    no shared-endpoint skip condition across index-0/non-zero pair indexes.
- Added re-parameterization counterpart for the same non-zero + reversed
  `TwoIntersects` positive path:
  - `non_local_two_intersects_reversed_second_segment_keeps_both_with_zero_length_lead_segment`
  - verifies prepending a zero-length lead segment (index shift) still retains
    both points for the shifted target pair.
- Added index-0 re-parameterization counterpart for the same reversed
  + no-shared-end `TwoIntersects` path:
  - `non_local_two_intersects_reversed_second_segment_keeps_both_with_zero_length_lead_segment_index0_shift`
  - verifies prepending a zero-length lead segment (index shift) still retains
    both points for shifted pair (`start_index` pair `1/5`).
- Added API-level `all_self_intersects_as_basic` counterparts for the same
  non-zero + reversed + no-shared-end `TwoIntersects` path:
  - `all_self_intersects_basic_two_intersects_reversed_second_segment_keeps_both`
  - `all_self_intersects_basic_two_intersects_reversed_second_segment_keeps_both_nonzero`
  - `all_self_intersects_basic_two_intersects_reversed_second_segment_keeps_both_with_zero_length_lead_segment`
  - `all_self_intersects_basic_two_intersects_reversed_second_segment_keeps_both_with_zero_length_lead_segment_index0_shift`
  - verifies both `include_overlapping=false/true` retain both points for the
  targeted pair across index-0/non-zero paths, including the zero-length-lead
    re-parameterization shift (including index-0 zero-length shift).
- Added bounded mixed line/arc overlap-adjacent collection-level parity probe:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication`
- Added closed/open asymmetry probe for mixed line/arc overlap-adjacent dedup:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_closed_pline1`
- Added complementary closed/open asymmetry probe where `pline2` is closed:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_closed_pline2`
- Added both-closed counterpart for the same mixed line/arc
  overlap-adjacent dedup branch:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed`
- Added role-flip symmetry counterpart for the same both-closed mixed line/arc
  overlap-adjacent dedup branch:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_role_flip_symmetry`
  - verifies AB/BA index-role inversion and stable overlap endpoint ordering in
    bounded both-closed geometry.
- Added start-index-rotated role-flip symmetry counterpart for the same
  both-closed mixed line/arc overlap-adjacent dedup branch:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation keeps dedup behavior intact while
    producing non-zero overlap segment indexing with stable AB/BA role symmetry.
- Added canonical-name alias counterpart for direct closed-pline2 rotated
  counterpart tracing of the same branch:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-pline2
    naming while reusing identical rotated geometry.
- Added complementary closed-side start-index-rotated role-flip symmetry
  counterpart where the non-zero overlap segment index is carried by `pline1`:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - verifies overlap-adjacent endpoint dedup behavior and AB/BA role inversion
    remain stable when closed-side rotation is applied on the first input
    polyline instead of the second.
- Added zero-length-lead counterparts for the explicit closed-pline1/2 rotated
  role-flip probes in the same mixed line/arc both-closed branch:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA role-inversion start-index mapping,
    and stable overlap endpoint ordering under non-zero index shift.
- Added opposing-direction arc-overlap-adjacent collection-level parity probe:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication`
- Added opposing-direction arc-overlap-adjacent closed/open variant probes:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_closed_pline1`
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_closed_pline2`
- Added both-closed counterpart for the same opposing-direction
  arc-overlap-adjacent dedup branch:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed`
- Added role-flip symmetry counterpart for the same both-closed
  opposing-direction arc-overlap-adjacent dedup branch:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_role_flip_symmetry`
  - verifies AB/BA index-role inversion and overlap-endpoint set equivalence in
    bounded both-closed geometry.
- Added both-closed start-index-rotation role-flip symmetry counterpart for the
  same opposing-direction arc-overlap-adjacent dedup branch:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation yields non-zero overlap segment
    indexing, preserves overlap-adjacent endpoint basic dedup behavior, and
    keeps AB/BA overlap endpoint sets equivalent under role inversion.
- Added canonical-name alias counterpart for direct closed-pline2 rotated
  counterpart tracing of the same branch:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-pline2
    naming while reusing identical rotated geometry.
- Added complementary closed-side start-index-rotated role-flip symmetry
  counterpart where the non-zero overlap segment index is carried by `pline1`:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - verifies overlap-adjacent endpoint dedup behavior and AB/BA role inversion
  remain stable when closed-side rotation is applied on the first input
  polyline instead of the second.
- Added zero-length-lead counterparts for the explicit closed-pline1/2 rotated
  role-flip probes in the same opposing-direction branch:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - verifies pure-overlap dedup behavior (no basic intersects), AB/BA
    role-inversion start-index mapping, and stable overlap endpoint-set
    equivalence under non-zero index shift.
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
- Added role-flip symmetry counterpart for the same bounded both-closed
  adjacent dedup probe:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_role_flip_symmetry`
  - verifies AB/BA index-role inversion and overlap endpoint-order stability in
    bounded both-closed geometry.
- Added start-index-rotated role-flip symmetry counterpart for the same bounded
  both-closed adjacent dedup probe:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation (non-zero overlap segment index)
    preserves bounded overlap behavior and AB/BA index-role inversion semantics.
- Added canonical-name alias counterpart for direct closed-pline2 rotated
  counterpart tracing of the same branch:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-pline2
    naming while reusing identical rotated geometry.
- Added complementary closed-side start-index-rotated role-flip symmetry
  counterpart where the non-zero overlap segment index is carried by `pline1`:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - verifies bounded overlap-adjacent dedup behavior and AB/BA index-role
  inversion remain stable when closed-side rotation is applied on the first
  input polyline instead of the second.
- Added zero-length-lead counterparts for the explicit closed-pline1/2 rotated
  role-flip probes in the same adjacent-dedup branch:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - verifies no-basic overlap-adjacent dedup behavior, AB/BA role-inversion
    start-index mapping, and branch-aligned overlap endpoint ordering under
    non-zero index shift.
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
- Added role-flip symmetry counterpart for the same bounded reversed
  endpoint-order + both-closed probe:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_role_flip_symmetry`
  - verifies stable three-basic-intersect behavior, AB/BA basic-index
    role inversion, and overlap endpoint-order swap semantics.
- Added closed-side start-index rotation counterpart for the same bounded
  reversed endpoint-order + both-closed adjacent-line-flip probe:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies non-zero overlap segment indexing under closed-side start-vertex
    rotation, stable three-basic-intersect behavior, and AB/BA index-role
    inversion semantics.
- Added canonical-name alias counterpart for direct closed-pline2 rotated
  counterpart tracing of the same branch:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-pline2
    naming while reusing identical rotated geometry.
- Added complementary closed-side start-index-rotated role-flip symmetry
  counterpart where the non-zero overlap segment index is carried by `pline1`:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - verifies stable three-basic-intersect behavior, overlap-endpoint dedup
  rules, and AB/BA role-inversion semantics when rotation is applied on the
  first input polyline instead of the second.
- Added zero-length-lead counterparts for the explicit closed-pline1/2 rotated
  role-flip probes in the same branch:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - verifies stable three-basic-intersect behavior without reintroducing the
    overlap-endpoint basic at `(3, 1)`, plus AB/BA role-inversion start-index
    mapping and overlap endpoint-order reversal under non-zero index shift.
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
- Added role-flip symmetry counterpart for the same bounded
  `arc1_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_role_flip_symmetry`
  - verifies AB/BA index-role inversion with bounded basic+overlap behavior and
    overlap endpoint-order swap semantics.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  same bounded `arc1_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies bounded basic+overlap behavior, AB/BA index-role inversion, and
    overlap endpoint-order swap semantics after closed-side start-vertex
    rotation.
- Added canonical-name alias counterpart for direct closed-pline1 rotated
  counterpart tracing of the same branch:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-pline1
    naming while reusing identical rotated geometry.
- Added complementary closed-side start-index-rotated role-flip symmetry
  counterpart where the non-zero overlap segment index is carried by `pline2`:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies bounded basic+overlap behavior, overlap endpoint-order swap
    semantics, and AB/BA role inversion remain stable when rotation is applied
    on the second input polyline.
- Added bounded closed-shape collection-level counterpart for `both_reverse_dir`
  partial overlap:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed`
- Added parameter-role flipped counterpart for the same bounded
  `both_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_flipped_roles`
- Added role-flip symmetry counterpart for the same bounded
  `both_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_role_flip_symmetry`
  - verifies AB/BA index-role inversion with bounded basic+overlap behavior and
    overlap endpoint-order stability.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  same bounded `both_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies bounded basic+overlap behavior, AB/BA index-role inversion, and
    overlap endpoint-order stability after closed-side start-vertex rotation.
- Added canonical-name alias counterpart for direct closed-pline1 rotated
  counterpart tracing of the same branch:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-pline1
    naming while reusing identical rotated geometry.
- Added complementary closed-side start-index-rotated role-flip symmetry
  counterpart where the non-zero overlap segment index is carried by `pline2`:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies bounded basic+overlap behavior, overlap-endpoint ordering, and
    AB/BA role inversion semantics remain stable when rotation is applied on
    the second input polyline.
- Added bounded closed-shape collection-level counterpart for `arc2_reverse_dir`
  partial overlap:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed`
- Added parameter-role flipped counterpart for the same bounded
  `arc2_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_flipped_roles`
- Added role-flip symmetry counterpart for the same bounded
  `arc2_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_role_flip_symmetry`
  - verifies pure-overlap behavior, AB/BA index-role inversion, and overlap
    endpoint-order swap semantics.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  same bounded `arc2_reverse_dir` + both-closed probe:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies pure-overlap behavior, AB/BA index-role inversion, and overlap
    endpoint-order swap semantics after closed-side start-vertex rotation.
- Added canonical-name alias counterpart for direct closed-pline1 rotated
  counterpart tracing of the same branch:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-pline1
    naming while reusing identical rotated geometry.
- Added complementary closed-side start-index-rotated role-flip symmetry
  counterpart where the non-zero overlap segment index is carried by `pline2`:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies pure-overlap behavior, overlap endpoint-order swap semantics, and
    AB/BA role inversion remain stable when rotation is applied on the second
    input polyline.
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
- Added role-flip symmetry probe for the same bounded closed-`pline1`
  reversed-endpoint-order closure-edge geometry:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_role_flip_symmetry`
  - verifies a stable closure-edge basic intersection and explicit
    `start_index1/start_index2` inversion while overlap endpoint ordering swaps
    under parameter inversion.
- Added closed-side start-index rotation counterpart for the same bounded
  closed-`pline1` reversed-endpoint-order closure-edge role-flip geometry:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation preserves bounded
    overlap/basic-point outcomes and AB/BA index-role inversion semantics.
- Added role-flip symmetry probe for the same bounded closed-`pline2`
  reversed-endpoint-order closure-edge geometry:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_role_flip_symmetry`
  - verifies a stable closure-edge basic intersection and explicit
    `start_index1/start_index2` inversion while overlap endpoint ordering swaps
    under parameter inversion.
- Added closed-side start-index rotation counterpart for the same bounded
  closed-`pline2` reversed-endpoint-order closure-edge role-flip geometry:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation preserves bounded
    overlap/basic-point outcomes and AB/BA index-role inversion semantics.
- Added canonical-name aliases for direct counterpart tracing from
  `...with_closure_basic_intersect(_flipped_roles)` on both closed-side
  assignments:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect_start_index_rotation_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect_start_index_rotation_role_flip_symmetry`
- Clarified bounded parity behavior for these variants: closure edges can
  produce additional real basic intersections that are independent of
  overlap-adjacent duplicate filtering.
- Added bounded wrap-around-adjacency endpoint-dedup probes that exercise
  `next_wrapping_index(last) == 0` without introducing independent closure-edge
  crossings:
  - `wrap_around_overlap_endpoint_deduplication_closed_pline1`
  - `wrap_around_overlap_endpoint_deduplication_closed_pline2`
- Added both-closed counterpart for the same bounded wrap-around endpoint-dedup
  branch:
  - `wrap_around_overlap_endpoint_deduplication_both_closed`
- Added role-flip symmetry counterpart for the same bounded both-closed
  wrap-around endpoint-dedup branch:
  - `wrap_around_overlap_endpoint_deduplication_both_closed_role_flip_symmetry`
  - verifies AB/BA index-role inversion with no-basic dedup behavior and stable
    overlap endpoint ordering.
- Added start-index-rotated role-flip symmetry counterpart for the same bounded
  both-closed wrap-around endpoint-dedup branch:
  - `wrap_around_overlap_endpoint_deduplication_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation keeps no-basic dedup behavior
    while shifting overlap segment indexing away from zero with stable AB/BA
    role symmetry.
- Added canonical-name alias counterparts for direct closed-side rotated
  counterpart tracing of the same branch:
  - `wrap_around_overlap_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-side
    naming while reusing identical rotated geometry.
- Added zero-length-lead counterparts for the explicit closed-pline1/2 rotated
  role-flip probes in the same wrap-around endpoint-dedup branch:
  - `wrap_around_overlap_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA role-inversion start-index mapping,
    and stable overlap endpoint ordering under non-zero index shift.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  bounded closed-`pline2` wrap-around endpoint-dedup probe:
  - `wrap_around_overlap_endpoint_deduplication_closed_pline2_start_index_rotation_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order stability after closed-side start-vertex rotation.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  bounded closed-`pline1` wrap-around endpoint-dedup probe:
  - `wrap_around_overlap_endpoint_deduplication_closed_pline1_start_index_rotation_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order stability after closed-side start-vertex rotation.
- Added bounded mixed line/arc wrap-around-adjacency endpoint-dedup probes:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline1`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline2`
- Added both-closed counterpart for the same bounded mixed line/arc
  wrap-around endpoint-dedup branch:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed`
- Added role-flip symmetry counterpart for the same bounded both-closed mixed
  line/arc wrap-around endpoint-dedup branch:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed_role_flip_symmetry`
  - verifies AB/BA index-role inversion with no-basic dedup behavior and stable
    overlap endpoint ordering.
- Added start-index-rotated role-flip symmetry counterpart for the same bounded
  both-closed mixed line/arc wrap-around endpoint-dedup branch:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation keeps no-basic dedup behavior
    while shifting overlap segment indexing away from zero with stable AB/BA
    role symmetry.
- Added canonical-name alias counterparts for direct closed-side rotated
  counterpart tracing of the same branch:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-side
    naming while reusing identical rotated geometry.
- Added zero-length-lead counterparts for the explicit closed-pline1/2 rotated
  role-flip probes in the same mixed line/arc wrap-around endpoint-dedup branch:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA role-inversion start-index mapping,
    and stable overlap endpoint ordering under non-zero index shift.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  bounded mixed line/arc closed-`pline2` wrap-around endpoint-dedup probe:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline2_start_index_rotation_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order stability after closed-side start-vertex rotation.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  bounded mixed line/arc closed-`pline1` wrap-around endpoint-dedup probe:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline1_start_index_rotation_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order stability after closed-side start-vertex rotation.
- Added bounded non-circle arc/arc wrap-around-adjacency endpoint-dedup probes
  for same and reversed endpoint ordering:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline1`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline1`
- Added exact parameter-role flipped counterpart for the bounded same-order
  closed-`pline1` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline1_flipped_roles`
- Added role-flip symmetry counterpart for the same bounded same-order
  closed-`pline1` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline1_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order stability under parameter inversion.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  same bounded same-order closed-`pline1` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline1_start_index_rotation_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order stability after closed-side start-vertex rotation.
- Added exact parameter-role flipped counterpart for the bounded reversed-order
  closed-`pline1` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline1_flipped_roles`
- Added role-flip symmetry counterpart for the same bounded reversed-order
  closed-`pline1` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline1_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order swap semantics under parameter inversion.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  same bounded reversed-order closed-`pline1` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline1_start_index_rotation_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order swap semantics under parameter inversion after closed-side
    start-vertex rotation.
- Added complementary non-circle arc/arc wrap-around-adjacency probes where
  `pline2` is closed:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline2`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline2`
- Added both-closed counterpart for the bounded same-order non-circle arc/arc
  wrap-around dedup branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed`
- Added role-flip symmetry counterpart for the same bounded same-order
  non-circle both-closed wrap-around dedup branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_role_flip_symmetry`
  - verifies AB/BA index-role inversion with no-basic dedup behavior and stable
    overlap endpoint ordering.
- Added start-index-rotated role-flip symmetry counterpart for the same bounded
  same-order non-circle both-closed wrap-around dedup branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation keeps no-basic dedup behavior
    while shifting overlap segment indexing away from zero with stable AB/BA
    role symmetry.
- Added canonical-name alias counterparts for direct closed-side rotated
  counterpart tracing of the same branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-side
    naming while reusing identical rotated geometry.
- Added zero-length-lead counterparts for the explicit closed-pline1/2 rotated
  role-flip probes in the same same-order both-closed non-circle wrap-around
  dedup branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA role-inversion start-index mapping,
    and stable overlap endpoint ordering under non-zero index shift.
- Added exact parameter-role flipped counterpart for the bounded same-order
  closed-`pline2` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline2_flipped_roles`
- Added role-flip symmetry counterpart for the same bounded same-order
  closed-`pline2` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline2_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order stability under parameter inversion.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  same bounded same-order closed-`pline2` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline2_start_index_rotation_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order stability after closed-side start-vertex rotation.
- Added exact parameter-role flipped counterpart for the bounded reversed-order
  closed-`pline2` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline2_flipped_roles`
- Added role-flip symmetry counterpart for the same bounded reversed-order
  closed-`pline2` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline2_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order swap semantics under parameter inversion.
- Added both-closed counterpart for the bounded reversed-order non-circle arc/arc
  wrap-around dedup branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed`
- Added role-flip symmetry counterpart for the same bounded reversed-order
  non-circle both-closed wrap-around dedup branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_role_flip_symmetry`
  - verifies AB/BA index-role inversion with no-basic dedup behavior and
    overlap endpoint-order swap semantics under parameter inversion.
- Added start-index-rotated role-flip symmetry counterpart for the same bounded
  reversed-order non-circle both-closed wrap-around dedup branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation keeps no-basic dedup behavior
    while shifting overlap segment indexing away from zero with stable AB/BA
    role inversion and endpoint-order swap semantics.
- Added canonical-name alias counterparts for direct closed-side rotated
  counterpart tracing of the same branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - verifies parity evidence is explicitly discoverable under closed-side
    naming while reusing identical rotated geometry.
- Added zero-length-lead counterparts for the explicit closed-pline1/2 rotated
  role-flip probes in the same reversed-order both-closed non-circle wrap-around
  dedup branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA role-inversion start-index mapping,
    and branch-aligned overlap endpoint-order swap semantics under non-zero index shift.
- Added closed-side start-index rotation role-flip symmetry counterpart for the
  same bounded reversed-order closed-`pline2` wrap-around dedup probe:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline2_start_index_rotation_role_flip_symmetry`
  - verifies no-basic dedup behavior, AB/BA index-role inversion, and overlap
    endpoint-order swap semantics under parameter inversion after closed-side
    start-vertex rotation.
- Added non-circle arc/arc wrap-around closure-edge variants where additional
  real basic intersects are expected and asserted:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline1_with_closure_basic_intersect`
  - `wrap_around_non_circle_arc_overlap_reversed_order_closed_pline1_with_closure_basic_intersect`
- Added complementary closure-edge variants where `pline2` is closed and
  independent basics are explicitly asserted:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect`
- Added exact parameter-role flipped counterpart for the bounded open-side-
  reversed closed-`pline2` closure-edge variant:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect_flipped_roles`
- Added role-flip symmetry probe for the same bounded open-side-reversed
  closed-`pline2` closure-edge geometry:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_role_flip_symmetry`
  - verifies bounded basic/overlap outcomes with AB/BA index-role inversion and
    stable overlap endpoint ordering under parameter inversion.
- Added closed-side start-index rotation counterpart for the same bounded
  open-side-reversed closed-`pline2` closure-edge role-flip geometry:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_start_index_rotation_role_flip_symmetry`
  - verifies rotating closed-side start vertex preserves bounded basic/overlap
    outcomes, AB/BA index-role inversion, and stable overlap endpoint ordering.
- Added canonical-name aliases for direct counterpart tracing from
  `...open_side_reversed_closed_pline2_with_closure_basic_intersect(_flipped_roles)`:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect_start_index_rotation_role_flip_symmetry`
- Added exact parameter-role flipped counterpart for the bounded same-order
  closed-`pline2` closure-edge variant:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect_flipped_roles`
- Added role-flip symmetry probe for the same bounded closed-`pline2`
  same-order closure-edge geometry:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_role_flip_symmetry`
  - verifies `start_index1/start_index2` inversion and stable overlap endpoint
    ordering under parameter inversion.
- Added closed-side start-index rotation counterpart for the same bounded
  closed-`pline2` same-order closure-edge role-flip geometry:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation preserves bounded basic/overlap
    outcomes and AB/BA index-role inversion semantics.
- Added canonical-name aliases for direct counterpart tracing from
  `...same_order_closed_pline2_with_closure_basic_intersect(_flipped_roles)`:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect_start_index_rotation_role_flip_symmetry`
- Added closed-`pline2` closure-edge counterpart that isolates open-side
  reversal while keeping closed-side arc orientation non-reversed:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect`
- Added exact parameter-role flipped counterpart for the bounded open-side-
  reversed + normal-closed-side closed-`pline2` closure-edge variant:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_flipped_roles`
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
- Added canonical-name alias for direct counterpart tracing from
  `...with_normal_closed_side_closure_basic_intersect(_flipped_roles)`:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_role_flip_symmetry`
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
- Added closed-side start-index rotation counterpart for the open-side-reversed
  + normal-closed-side role-flip symmetry geometry:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_normal_closed_side_start_index_rotation_role_flip_symmetry`
  - verifies rotating closed-side start vertex preserves bounded overlap/basic
    point outcomes and AB/BA index-role inversion semantics.
- Added non-zero-open-index counterpart for the same open-side-reversed +
  normal-closed-side role-flip symmetry geometry:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_normal_closed_side_role_flip_symmetry_nonzero_open_index`
  - verifies a shifted open-side overlap segment index (off zero) preserves the
    same bounded overlap/basic point outcomes and AB/BA index-role inversion.
- Added canonical-name start-index-rotation alias for direct counterpart
  tracing from `...with_normal_closed_side_closure_basic_intersect(_flipped_roles)`:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_role_flip_symmetry`
- Added canonical-name aliases that retain the explicit `_intersect_` token for
  direct parity-chain tracing from
  `...with_normal_closed_side_closure_basic_intersect(_flipped_roles)`:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_role_flip_symmetry`
- Added closed-side-reversed start-index rotation counterpart for the same
  open-side-reversed closure-edge role-flip geometry:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_side_reversed_start_index_rotation_role_flip_symmetry`
  - verifies rotating closed-side start vertex preserves bounded overlap/basic
    point outcomes and AB/BA index-role inversion semantics.
- Added non-zero-open-index counterpart for the same open-side-reversed +
  closed-side-reversed role-flip symmetry geometry:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_side_reversed_role_flip_symmetry_nonzero_open_index`
  - verifies a shifted open-side overlap segment index (off zero) preserves
    bounded overlap/basic point outcomes and AB/BA index-role inversion while
    keeping overlap endpoint ordering stable under role inversion.
- Added rotated-start counterpart for the original closed-side-reversed
  closure-edge role-flip symmetry probe:
  - `wrap_around_non_circle_arc_overlap_closed_side_reversed_closure_basic_start_index_rotation_role_flip_symmetry`
  - verifies closed-side start-vertex rotation preserves bounded overlap/basic
    point outcomes and AB/BA index-role inversion semantics.
- Added offset-derived circle/rectangle intersection role-flip symmetry matrix
  parity across orientation reversals:
  - `cpp_circle_rectangle_intersection_role_flip_symmetry_matrix_parity`
  - verifies AB/BA role inversion (`start_index1 <-> start_index2`) while
    preserving the expected four C++-aligned intersection points.
- Added offset-derived circle/rectangle start-index rotation parity matrix:
  - `cpp_circle_rectangle_intersection_start_index_rotation_parity`
  - verifies closed-polyline start-vertex rotation keeps the same C++-aligned
    intersection point set while preserving AB/BA `start_index` role inversion.
- Added full orientation+rotation matrix counterpart for the same offset-derived
  circle/rectangle intersection parity:
  - `cpp_circle_rectangle_intersection_start_index_rotation_full_matrix_parity`
  - verifies the same C++-aligned point set and AB/BA `start_index` role
    inversion across all rotated/reversed subject/clip combinations.
- Added full orientation+rotation matrix input-immutability guard for the same
  offset-derived circle/rectangle intersection parity:
  - `cpp_circle_rectangle_intersection_full_matrix_does_not_modify_input`
  - verifies `find_intersects` AB/BA calls do not mutate either input across
    all rotated/reversed subject/clip combinations.
- Added Rust options-path full-matrix counterpart for the same
  offset-derived circle/rectangle intersection parity:
  - `cpp_circle_rectangle_intersection_options_full_matrix_parity`
  - verifies `find_intersects_opt` with explicit `FindIntersectsOptions`
    (including `pline1_aabb_index`) preserves the same C++-aligned point set
    and AB/BA role-flip symmetry across all rotated/reversed subject/clip
    combinations, while keeping both inputs immutable.
- Added Rust options-path overlap+basic counterpart for source-aligned
  non-circle wrap-around overlap-role geometry:
  - `cpp_overlap_and_basic_intersection_options_role_flip_parity_nonzero_open_index`
  - verifies `find_intersects_opt` preserves one basic + one overlapping
    intersect with non-zero open-side segment index, keeps AB/BA role-flip
    start-index mapping and overlap endpoint ordering, and leaves both inputs
    immutable.
- Added Rust options-path overlap-order counterpart for source-aligned
  line-overlap direction ordering:
  - `cpp_overlap_endpoint_order_options_role_flip_parity`
  - verifies `find_intersects_opt` preserves overlap endpoint ordering by
    second-segment direction for same/opposite overlap orientation, keeps AB/BA
    start-index role flip symmetry, matches default-path overlap output, and
    leaves both inputs immutable.
- Added Rust options-path tolerance-control counterpart for near-touch
  intersection detection:
  - `cpp_intersection_options_pos_equal_eps_controls_detection`
  - verifies `find_intersects_opt` with explicit `pos_equal_eps` changes
    near-touch behavior as expected (`1e-6` => no intersects, `1e-3` => one
    basic intersect), preserves AB/BA role-flip symmetry, and leaves both
    inputs immutable.
- Added Rust options-path overlap+basic matrix counterpart for source-aligned
  wrap-around closure-edge geometry:
  - `cpp_overlap_and_basic_intersection_options_matrix_parity`
  - verifies `find_intersects_opt` keeps one basic + one overlap across
    open-side nonzero-index and closed-side start-index-rotation variants,
    preserves AB/BA role-flip mapping and expected overlap endpoints, matches
    default-path index/point outputs, and leaves both inputs immutable.
- Added Rust options-path overlap+basic matrix counterpart for source-aligned
  normal-closed-side wrap-around closure-edge geometry:
  - `cpp_overlap_and_basic_intersection_options_normal_closed_side_matrix_parity`
  - verifies `find_intersects_opt` keeps one basic + one overlap across
    open-side nonzero-index and normal-closed-side start-index-rotation
    variants, preserves AB/BA role-flip mapping with expected swapped endpoint
    order, matches default-path index/point outputs, and leaves both inputs
    immutable.
- Added Rust options-path endpoint-elision matrix counterpart for source-aligned
  `skip_intr_at_end` boundary behavior:
  - `cpp_skip_intr_at_end_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces open/closed `pline1` and
    open/closed `pline2` next-segment start-index attribution at shared-endpoint
    touches, matches default-path index/point outputs, preserves AB/BA role-flip
    mapping, and leaves both inputs immutable.
- Added Rust options-path endpoint-touch matrix counterpart for source-aligned
  open-polyline touch boundaries:
  - `cpp_open_polyline_endpoint_touch_options_parity`
  - verifies `find_intersects_opt` reproduces end-touch-start, flipped, and
    start-touch-start single-basic outputs (no overlaps), matches default-path
    index/point outputs, preserves AB/BA role-flip mapping, and leaves both
    inputs immutable.
- Added Rust options-path circle-touch/overlap matrix counterpart for
  source-aligned closed-circle boundaries:
  - `cpp_circle_touch_and_overlap_options_parity_matrix`
  - verifies `find_intersects_opt` reproduces touching-circle single-basic and
    same/opposing-direction full-overlap outputs, matches default-path
    index/point outputs, preserves AB/BA role-flip mapping, and leaves both
    inputs immutable.
- Added Rust options-path both-closed adjacent-overlap dedup matrix counterpart
  for source-aligned non-circle arc/arc overlap boundaries:
  - `cpp_non_circle_closed_overlap_adjacent_dedup_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces bounded both-closed overlap-only
    output (no basics), including closed-side start-index-rotation variants,
    matches default-path index/point outputs, preserves AB/BA role-flip
    start-index mapping, and leaves both inputs immutable.
- Added Rust options-path both-closed opposing-direction adjacent-overlap dedup
  matrix counterpart for source-aligned arc/arc overlap boundaries:
  - `cpp_opposing_direction_closed_overlap_adjacent_dedup_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces bounded both-closed
    opposing-direction overlap-only output (no basics), including closed-side
    start-index-rotation variants, matches default-path index/point outputs,
    preserves AB/BA role-flip start-index mapping, and leaves both inputs
    immutable.
- Added Rust options-path arc-adjacent overlap-endpoint dedup matrix counterpart
  for source-aligned mixed line/arc overlap boundaries:
  - `cpp_overlap_endpoint_arc_adjacent_dedup_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces bounded both-closed
    overlap-only output (no basics), including closed-side start-index-rotation
    variants, matches default-path index/point outputs, preserves AB/BA
    role-flip start-index mapping with stable endpoint ordering, and leaves
    both inputs immutable.
- Added Rust options-path reversed-endpoint-order closure-basic matrix
  counterpart for source-aligned non-circle arc overlap boundaries:
  - `cpp_reversed_endpoint_closure_basic_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces one-basic + one-overlap closure-edge
    output across closed-pline1 and closed-pline2 variants (including
    closed-side start-index-rotation), matches default-path index/point outputs,
    preserves AB/BA role-flip mapping, and leaves both inputs immutable.
- Added Rust options-path reversed-endpoint-order + adjacent-line-flip
  both-closed matrix counterpart for source-aligned non-circle arc overlap
  boundaries:
  - `cpp_reversed_endpoint_adjacent_line_flip_both_closed_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces three-basic + one-overlap output
    with overlap-endpoint basic dedup at `(3,1)`, including closed-side
    start-index-rotation variants, matches default-path index/point outputs,
    preserves AB/BA role-flip mapping (including swapped overlap endpoint order),
    and leaves both inputs immutable.
- Added Rust options-path `arc1_reverse_dir` + both-closed matrix counterpart
  for source-aligned non-circle arc overlap boundaries:
  - `cpp_arc1_reverse_dir_both_closed_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces one-basic + one-overlap output
    with correct overlap endpoint ordering and AB/BA role-flip behavior
    (including swapped overlap endpoint order), across closed-side
    start-index-rotation variants, while matching default-path index/point
    outputs and preserving input immutability.
- Added Rust options-path `both_reverse_dir` + both-closed matrix counterpart
  for source-aligned non-circle arc overlap boundaries:
  - `cpp_both_reverse_dir_both_closed_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces one-basic + one-overlap output
    with branch-specific overlap endpoint ordering and AB/BA role-flip behavior
    (keeping overlap endpoint ordering stable under inversion), across
    closed-side start-index-rotation variants, while matching default-path
    index/point outputs and preserving input immutability.
- Added Rust options-path `arc2_reverse_dir` + both-closed matrix counterpart
  for source-aligned non-circle arc overlap boundaries:
  - `cpp_arc2_reverse_dir_both_closed_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces overlap-only output (no basics)
    with branch-specific overlap endpoint ordering and AB/BA role-flip behavior
    (including swapped overlap endpoint order), across closed-side
    start-index-rotation variants, while matching default-path index/point
    outputs and preserving input immutability.
- Added Rust options-path wrap-around dedup matrix counterpart for source-
  aligned non-circle arc overlap boundaries with closed `pline1`:
  - `cpp_wrap_around_closed_pline1_dedup_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces overlap-only wrap-around output
    (no basics) across same-order and reversed-order arc direction variants,
    including closed-side start-index-rotation cases, while matching
    default-path index/point outputs, preserving AB/BA role-flip start-index
    mapping, validating branch-specific overlap endpoint-order invariants
    (stable vs swapped), and preserving input immutability.
- Added Rust options-path wrap-around dedup matrix counterpart for source-
  aligned non-circle arc overlap boundaries with closed `pline2`:
  - `cpp_wrap_around_closed_pline2_dedup_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces overlap-only wrap-around output
    (no basics) across same-order and reversed-order arc direction variants,
    including closed-side start-index-rotation cases, while matching
    default-path index/point outputs, preserving AB/BA role-flip start-index
    mapping, validating branch-specific overlap endpoint-order invariants
    (stable vs swapped), and preserving input immutability.
- Added Rust options-path wrap-around same-order closure-edge matrix counterpart
  for source-aligned non-circle arc overlap boundaries with closed `pline2`:
  - `cpp_wrap_around_same_order_closed_pline2_closure_basic_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces one-basic + one-overlap
    closure-edge output for same-order arc direction, including closed-side
    start-index-rotation cases, while matching default-path index/point
    outputs, preserving AB/BA role-flip start-index mapping with stable overlap
    endpoint ordering, and preserving input immutability.
- Added explicit options-path flipped-role counterpart for the same
  closed-`pline2` same-order closure-edge branch:
  - `cpp_wrap_around_same_order_closed_pline2_closure_basic_flipped_roles_options_parity`
  - verifies exact AB start-index attribution from source-aligned
    `..._with_closure_basic_intersect_flipped_roles` geometry (`basic 1/1`,
    `overlap 2/0`), while matching default-path index/point outputs, preserving
    AB/BA role-flip mapping, and preserving input immutability.
- Tightened non-zero-open-index closed-side-reversed probe from non-zero checks
  to exact source-aligned index attribution:
  - `cpp_overlap_and_basic_intersection_options_role_flip_parity_nonzero_open_index`
  - now asserts exact AB start-index values (`basic 1/0`, `overlap 1/1`) and
    default-path index/point parity for the
    `...open_side_reversed_closed_side_reversed...nonzero_open_index` geometry,
    while preserving AB/BA role-flip mapping and input immutability.
- Replaced the normal-closed-side non-zero-open-index alias with an explicit
  options-path semantic probe:
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_nonzero_open_index_options_parity`
  - now asserts exact AB start-index values (`basic 1/0`, `overlap 1/1`) and
    default-path index/point parity for the
    `...open_side_reversed_normal_closed_side...nonzero_open_index` geometry,
    while preserving AB/BA role-flip mapping and input immutability.
- Replaced the closed-side-reversed start-index-rotation alias with an explicit
  options-path semantic probe:
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_start_index_rotation_options_parity`
  - now asserts exact AB start-index values (`basic 0/0`, `overlap 0/1`) and
  default-path index/point parity for the
  `...open_side_reversed_closed_side_reversed...start_index_rotation`
  geometry, while preserving AB/BA role-flip mapping and input immutability.
- Replaced the open-side-reversed + closed-`pline2` closure-basic alias with an
  explicit options-path semantic probe:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_options_matrix_parity`
  - now asserts exact AB start-index values (`basic 0/1`, `overlap 0/2`) and
    default-path index/point parity for the
    `...open_side_reversed_closed_pline2_with_closure_basic...` geometry, while
    preserving AB/BA role-flip mapping and input immutability.
- Replaced the closed-side-reversed non-zero-open-index alias with an explicit
  options-path semantic probe:
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_nonzero_open_index_options_parity`
  - now asserts exact AB start-index values (`basic 1/0`, `overlap 1/1`) and
    default-path index/point parity for the
    `...open_side_reversed_closed_side_reversed...nonzero_open_index` geometry,
    while preserving AB/BA role-flip mapping and input immutability.
- Replaced the closed-side-reversed closure-basic alias with an explicit
  options-path semantic probe:
  - `cpp_wrap_around_closed_side_reversed_closure_basic_options_matrix_parity`
  - now asserts exact AB start-index values (`basic 1/0`, `overlap 2/0`) and
    default-path index/point parity for the
    `...closed_side_reversed...closure_basic` geometry, while preserving AB/BA
    role-flip mapping and input immutability.
- Retargeted three canonical wrap-around aliases to the new explicit semantic
  probes instead of the broad shared matrix:
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_role_flip_options_parity`
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_role_flip_options_matrix_parity`
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_options_matrix_parity`
  - now delegate to branch-specific probes that assert exact start-index
    attribution/default-path parity for the corresponding closure-edge
    geometries.
- Replaced the open-side-reversed + normal-closed-side closure-basic alias with
  an explicit options-path semantic probe:
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_options_matrix_parity`
  - now asserts exact AB start-index values (`basic 0/1`, `overlap 0/2`) and
    default-path index/point parity for the
    `...open_side_reversed_normal_closed_side...closure_basic` geometry, while
    preserving AB/BA role-flip mapping (with reversed overlap endpoint order)
    and input immutability.
- Retargeted the normal-closed-side closure-basic role-flip alias to the new
  explicit semantic probe:
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_role_flip_options_matrix_parity`
- Replaced the normal-closed-side closure-basic start-index-rotation alias with
  an explicit options-path semantic probe:
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_start_index_rotation_options_matrix_parity`
  - now asserts exact AB start-index values (`basic 0/0`, `overlap 0/1`) and
    default-path index/point parity for the
    `...open_side_reversed_normal_closed_side...start_index_rotation` geometry,
    while preserving AB/BA role-flip mapping (with reversed overlap endpoint
    order) and input immutability.
- Replaced three remaining wrap-around role-flip/start-index alias wrappers
  with explicit options-path semantic probes (no test-to-test delegation):
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_role_flip_options_parity`
    and
    `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_role_flip_options_matrix_parity`
    now each assert exact AB start-index values (`basic 1/0`, `overlap 2/0`)
    plus default-path index/point parity for the role-flipped closure-edge
    geometry, while preserving AB/BA start-index mapping and input immutability.
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_role_flip_options_matrix_parity`
    now asserts exact AB start-index values (`basic 1/0`, `overlap 2/0`) plus
    default-path index/point parity for the role-flipped normal-closed-side
    geometry, including expected AB/BA reversed overlap endpoint order and
    input immutability.
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_options_matrix_parity`
    now asserts exact AB start-index values (`basic 0/0`, `overlap 0/1`) plus
    default-path index/point parity for the closed-`pline2`
    start-index-rotation geometry, while preserving AB/BA mapping and input
    immutability.
- Replaced two remaining canonical alias wrappers in collection-level non-circle
  arc/arc-overlap-adjacent branches with explicit branch-local assertions (no
  delegation):
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - both now execute full rotated closed-`pline2` geometry checks inline,
    including AB/BA role-flip start-index mapping, endpoint-order invariants,
    and (for reversed-endpoint-order branch) adjacent-line basic dedup guards.
- Replaced three canonical closed-`pline1` start-index-rotation alias wrappers
  in non-circle both-closed direction-variant branches with explicit
  branch-local assertions (no delegation):
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_closed_pline1_role_flip_symmetry`
  - each now runs full rotated closed-`pline1` geometry checks inline with
    explicit AB/BA index-role mapping and branch-specific overlap endpoint-order
    invariants.
- Replaced two canonical closed-`pline2` start-index-rotation alias wrappers in
  mixed line/arc adjacent-dedup branches with explicit branch-local assertions
  (no delegation):
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_role_flip_symmetry`
  - both now run full rotated closed-`pline2` geometry checks inline, including
    explicit AB/BA index-role mapping and branch-specific overlap endpoint-order
    invariants.
- Added Rust options-path wrap-around closure-edge matrix counterpart for
  source-aligned non-circle arc overlap boundaries with closed `pline1`:
  - `cpp_wrap_around_closed_pline1_closure_basic_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces one-basic + one-overlap
    closure-edge output across same-order and reversed-order arc direction
    variants, including closed-side start-index-rotation cases, while matching
    default-path index/point outputs, preserving AB/BA role-flip start-index
    mapping, validating branch-specific overlap endpoint-order invariants
    (stable vs swapped), and preserving input immutability.
- Added Rust options-path wrap-around both-closed dedup matrix counterpart for
  source-aligned non-circle arc overlap boundaries:
  - `cpp_wrap_around_both_closed_dedup_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces overlap-only wrap-around output
    (no basics) across same-order and reversed-order arc direction variants for
    both-closed geometry, including closed-pline1 and closed-pline2
    start-index-rotation cases, while matching default-path index/point
    outputs, preserving AB/BA role-flip start-index mapping, validating
    branch-specific overlap endpoint-order invariants (stable vs swapped), and
    preserving input immutability.
- Added Rust options-path wrap-around overlap-endpoint dedup matrix counterpart
  for source-aligned closure-vertex duplicate-filter boundaries:
  - `cpp_wrap_around_overlap_endpoint_dedup_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces overlap-only output (no basics)
    across closed-pline1, closed-pline2, both-closed, and start-index-rotation
    variants; matches default-path index/point outputs; preserves AB/BA
    start-index role-flip mapping with stable overlap endpoint ordering; and
    preserves input immutability.
- Added Rust options-path wrap-around arc-adjacent overlap-endpoint dedup
  matrix counterpart for source-aligned closure-vertex duplicate-filter
  boundaries:
  - `cpp_wrap_around_overlap_endpoint_arc_adjacent_dedup_options_matrix_parity`
  - verifies `find_intersects_opt` reproduces overlap-only output (no basics)
    across closed-pline1, closed-pline2, both-closed, and start-index-rotation
    variants for the arc-adjacent branch; matches default-path index/point
    outputs; preserves AB/BA start-index role-flip mapping with stable overlap
    endpoint ordering; and preserves input immutability.
- Added canonical wrap-around closure-edge alias tests to map source branch
  names directly onto existing options-path matrices:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_options_matrix_parity`
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_role_flip_options_matrix_parity`
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_options_matrix_parity`
  - `cpp_wrap_around_closed_side_reversed_closure_basic_options_matrix_parity`
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_role_flip_options_parity`
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_options_matrix_parity`
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_role_flip_options_matrix_parity`
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_start_index_rotation_options_matrix_parity`
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_start_index_rotation_options_parity`
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_nonzero_open_index_options_parity`
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_nonzero_open_index_options_parity`
  - preserves existing assertions and execution semantics by delegating to the
    already-verified matrix implementations while improving source-to-test
    traceability for wrap-around closure-edge variants, including explicit
    non-zero-open-index source branch aliases.
- Strengthened offset input-immutability parity from single sample to full
  C++-mapped offset fixture matrix:
  - `cpp_parallel_offset_does_not_modify_input`
  - verifies `parallel_offset` leaves input vertex data unchanged across all
    simple + specific C++ offset fixtures.
- Added reversed-direction counterpart for the same Rust offset
  input-immutability fixture matrix:
  - `cpp_parallel_offset_reversed_does_not_modify_input`
  - verifies reversed-input `parallel_offset(-delta)` leaves input vertex data
    unchanged across all simple + specific C++ offset fixtures.
- Added Rust options-path counterpart for the same full offset
  input-immutability fixture matrix:
  - `cpp_parallel_offset_options_does_not_modify_input`
  - verifies `parallel_offset_opt` with explicit options leaves input vertex
    data unchanged across all simple + specific C++ offset fixtures.
- Added Rust options-path reversed counterpart for the same full offset
  input-immutability fixture matrix:
  - `cpp_parallel_offset_options_reversed_does_not_modify_input`
  - verifies reversed-input `parallel_offset_opt(-delta)` with explicit options
    leaves input vertex data unchanged across all simple + specific C++ offset
    fixtures.
- Added FFI-side counterpart for the same full offset input-immutability
  fixture matrix:
  - `pline_parallel_offset_does_not_modify_input_cpp_parity`
  - verifies `cavc_pline_parallel_offset` leaves input vertex data unchanged
    across all simple + specific C++ offset fixtures.
- Added FFI-side reversed-direction counterpart for the same full offset
  input-immutability fixture matrix:
  - `pline_parallel_offset_reversed_does_not_modify_input_cpp_parity`
  - verifies reversed-input `cavc_pline_parallel_offset(-delta)` leaves input
    vertex data unchanged across all simple + specific C++ offset fixtures.
- Added FFI options-path reversed counterpart for offset
  input-immutability fixture matrix:
  - `pline_parallel_offset_options_path_reversed_does_not_modify_input_cpp_parity`
  - verifies `cavc_pline_parallel_offset` with `cavc_pline_parallel_offset_o`
    leaves input vertex data unchanged across reversed simple + specific C++
    offset fixtures.
- Added combine-derived circle/rectangle commutative role-flip symmetry matrix
  parity across orientation reversals for C++-mapped operations:
  - `cpp_circle_rectangle_commutative_role_flip_matrix_parity`
  - verifies AB and BA both match C++-mapped geometry for `OR`/`AND`/`XOR`
  and match each other under role inversion.
- Added FFI-side counterpart for circle/rectangle commutative role-flip matrix
  parity:
  - `pline_boolean_circle_rectangle_commutative_role_flip_matrix_parity`
  - verifies `cavc_pline_boolean` remains aligned with C++-mapped
    `OR`/`AND`/`XOR` expectations and AB/BA commutative symmetry across
    orientation role-flip variants.
- Added combine-derived circle/rectangle commutative start-index rotation matrix
  parity:
  - `cpp_circle_rectangle_commutative_start_index_rotation_matrix_parity`
  - verifies closed-polyline start-vertex rotation and orientation reversals do
    not change C++-mapped geometry for `OR`/`AND`/`XOR`, and AB/BA remain
    symmetric.
- Added FFI-side counterpart for the same circle/rectangle commutative
  start-index-rotation matrix:
  - `pline_boolean_circle_rectangle_commutative_start_index_rotation_matrix_parity`
  - verifies `cavc_pline_boolean` remains aligned with C++-mapped
    `OR`/`AND`/`XOR` expectations and AB/BA commutative symmetry across
    rotated/reversed variants.
- Added complementary `NOT` start-index rotation matrix parity for
  circle/rectangle combine:
  - `cpp_circle_rectangle_not_start_index_rotation_matrix_parity`
  - verifies AB remains aligned to C++-mapped `NOT` expected geometry and BA
    remains stable versus the circle/rectangle BA baseline across all
    rotated/reversed subject/clip variants.
- Added complementary `NOT` role-flip matrix parity for circle/rectangle
  combine:
  - `cpp_circle_rectangle_not_complementary_role_flip_matrix_parity`
  - verifies AB stays aligned to C++-mapped `NOT` expected geometry and BA
    stays aligned to the circle/rectangle BA baseline across orientation
    reversals.
- Added FFI-side counterpart for the same circle/rectangle `NOT`
  start-index-rotation matrix:
  - `pline_boolean_circle_rectangle_not_start_index_rotation_matrix_parity`
  - verifies `cavc_pline_boolean` remains aligned with C++-mapped AB `NOT`
    expectations and stable BA baseline across rotated/reversed variants.
- Added FFI-side counterpart for circle/rectangle complementary `NOT`
  role-flip matrix parity:
  - `pline_boolean_circle_rectangle_not_complementary_role_flip_matrix_parity`
  - verifies `cavc_pline_boolean` keeps AB `NOT` expectations and BA baseline
    stable across orientation role-flip variants.
- Added FFI options-path circle/rectangle role-flip matrix parity:
  - `pline_boolean_options_path_circle_rectangle_role_flip_matrix_cpp_parity`
  - verifies `cavc_pline_boolean_o` output remains aligned with default
    `cavc_pline_boolean` output for AB and BA across orientation role-flip
    variants.
- Added FFI options-path circle/rectangle start-index rotation matrix parity:
  - `pline_boolean_options_path_circle_rectangle_start_index_rotation_matrix_cpp_parity`
  - verifies `cavc_pline_boolean_o` output remains aligned with default
    `cavc_pline_boolean` output for AB and BA across start-index rotation and
    orientation-reversal variants.
- Added FFI options-path coincident commutative role-flip matrix parity:
  - `pline_boolean_options_coincident_commutative_role_flip_matrix_cpp_parity`
  - verifies `cavc_pline_boolean_o` output remains aligned with default
    `cavc_pline_boolean` output for AB and BA across coincident orientation
    role-flip variants for `OR`/`AND`/`XOR`.
- Added FFI options-path coincident commutative start-index rotation matrix
  parity:
  - `pline_boolean_options_coincident_commutative_start_index_rotation_matrix_cpp_parity`
  - verifies `cavc_pline_boolean_o` output remains aligned with default
    `cavc_pline_boolean` output for AB and BA across coincident start-index
    rotation and orientation-reversal variants for `OR`/`AND`/`XOR`.
- Added FFI options-path coincident complementary `NOT` role-flip matrix
  parity:
  - `pline_boolean_options_coincident_not_complementary_role_flip_matrix_cpp_parity`
  - verifies `cavc_pline_boolean_o` output remains aligned with default
    `cavc_pline_boolean` output for AB and BA across coincident orientation
    role-flip variants for `A\B` / `B\A`.
- Added FFI options-path coincident complementary `NOT` start-index rotation
  matrix parity:
  - `pline_boolean_options_coincident_not_complementary_start_index_rotation_matrix_cpp_parity`
  - verifies `cavc_pline_boolean_o` output remains aligned with default
    `cavc_pline_boolean` output for AB and BA across coincident start-index
    rotation and orientation-reversal variants for `A\B` / `B\A`.
- Added source-aligned input immutability parity for circle/rectangle combine
  modes (mirrors old C++ no-modify suite coverage):
  - `cpp_circle_rectangle_combine_does_not_modify_input`
  - verifies `OR`/`NOT`/`AND`/`XOR` do not mutate either input polyline.
- Strengthened the same circle/rectangle combine immutability parity to full
  orientation+rotation matrix variants:
  - `cpp_circle_rectangle_combine_matrix_does_not_modify_input`
  - verifies AB/BA `OR`/`NOT`/`AND`/`XOR` do not mutate either input across
    rotated/reversed subject+clip matrix variants.
- Added FFI-side counterpart for full circle/rectangle combine immutability
  matrix coverage:
  - `pline_boolean_circle_rectangle_full_matrix_does_not_modify_input_cpp_parity`
  - verifies `cavc_pline_boolean` AB/BA `OR`/`NOT`/`AND`/`XOR` calls do not
    mutate either input across reversed and rotated circle/rectangle variants.
- Added FFI options-path counterpart for full circle/rectangle combine
  immutability matrix coverage:
  - `pline_boolean_options_path_circle_rectangle_full_matrix_does_not_modify_input_cpp_parity`
  - verifies `cavc_pline_boolean` with `cavc_pline_boolean_o` AB/BA
    `OR`/`NOT`/`AND`/`XOR` calls do not mutate either input across reversed and
    rotated circle/rectangle variants.
- Added source-side Rust options-path counterpart for full circle/rectangle
  combine immutability matrix coverage:
  - `cpp_circle_rectangle_combine_options_full_matrix_does_not_modify_input`
  - verifies `boolean_opt` AB/BA `OR`/`NOT`/`AND`/`XOR` calls do not mutate
    either input across reversed and rotated circle/rectangle variants.
- Added source-aligned self-combine reverse/mixed-direction invariants from old
  C++ combine test coverage:
  - `cpp_combine_with_self_reverse_mix_invariants`
  - verifies self `UNION`/`INTERSECT` returns self for forward and reversed
    orientation while keeping `neg_plines` empty, and self `EXCLUDE`/`XOR`
    keeps both `pos_plines` and `neg_plines` empty for
    `fwd/fwd`, `rev/rev`, `rev/fwd`, and `fwd/rev`.
- Strengthened the same self-combine invariant probe with source-aligned
  exact-vertex sequence assertions for self `UNION`/`INTERSECT`:
  - verifies single-result positive output keeps vertex-by-vertex
    `(x, y, bulge)` parity with input orientation (forward and reversed), in
    addition to existing geometry-set invariants.
- Extended source-aligned self-combine invariants to old C++
  `cavc_plineFunctionTests` closed-case matrix coverage:
  - `cpp_generated_closed_shape_matrix_combine_with_self_invariants_parity`
  - verifies all generated closed circle and half-circle cases preserve strict
    vertex-by-vertex `(x, y, bulge)` parity for self `UNION`/`INTERSECT`, and
    keep both outputs empty for self `EXCLUDE`/`XOR`.
- Added source-aligned `subtracted`-surface parity for combine fixtures:
  - `cpp_combine_expected_subtracted_empty_parity`
  - verifies Rust `neg_plines` remains empty across the C++-mapped
  circle/rectangle and coincident combine matrices where old
  `expectedSubtracted` is empty.
- Added source-anchored commutative role-flip matrix parity across orientation
  reversals for C++ coincident combine fixtures:
  - `cpp_coincident_commutative_role_flip_matrix_parity`
  - verifies AB/BA role inversion and orientation-matrix invariance for
    `OR`/`AND`/`XOR` against the C++-mapped coincident expected geometry sets.
- Added FFI-side counterpart for coincident commutative role-flip matrix
  parity:
  - `pline_boolean_coincident_commutative_role_flip_matrix_parity`
  - verifies `cavc_pline_boolean` keeps coincident `OR`/`AND`/`XOR` AB/BA
    symmetry and baseline geometry stable across orientation role-flip
    variants.
- Added source-anchored commutative start-index rotation matrix parity for C++
  coincident combine fixtures:
  - `cpp_coincident_commutative_start_index_rotation_matrix_parity`
  - verifies closed-polyline start-vertex rotation and orientation reversals do
    not change C++-mapped `OR`/`AND`/`XOR` geometry, and AB/BA remain symmetric.
- Added FFI-side counterpart for coincident commutative start-index-rotation
  matrix parity:
  - `pline_boolean_coincident_commutative_start_index_rotation_matrix_parity`
  - verifies `cavc_pline_boolean` remains aligned with C++-mapped coincident
    `OR`/`AND`/`XOR` expectations and AB/BA commutative symmetry across
    rotated/reversed variants.
- Added source-anchored complementary `NOT` matrix parity across orientation
  reversals for C++ coincident combine fixtures:
  - `cpp_coincident_not_complementary_role_flip_matrix_parity`
  - verifies `A\B` and `B\A` each remain aligned to their respective
  C++-mapped expected geometry sets under orientation reversals.
- Added FFI-side counterpart for coincident complementary `NOT` role-flip
  matrix parity:
  - `pline_boolean_coincident_not_complementary_role_flip_matrix_parity`
  - verifies `cavc_pline_boolean` keeps coincident `A\B` and `B\A` baseline
    geometry stable across orientation role-flip variants.
- Added source-anchored complementary `NOT` start-index rotation matrix parity
  for C++ coincident combine fixtures:
  - `cpp_coincident_not_complementary_start_index_rotation_matrix_parity`
  - verifies closed-polyline start-vertex rotation and orientation reversals do
    not change C++-mapped `A\B` / `B\A` expected geometry sets.
- Added FFI-side counterpart for coincident complementary `NOT`
  start-index-rotation matrix parity:
  - `pline_boolean_coincident_not_complementary_start_index_rotation_matrix_parity`
  - verifies `cavc_pline_boolean` keeps `A\B` and `B\A` baseline geometry
    stable across closed start-vertex rotation and orientation reversals.
- Added source-aligned input immutability parity for C++ coincident combine
  fixtures:
  - `cpp_coincident_combine_does_not_modify_input`
  - verifies all coincident case operations (`OR`/`NOT`/`AND`/`XOR`) preserve
    both input polylines.
- Strengthened the same C++ coincident combine immutability parity to full
  orientation+start-index-rotation matrix variants:
  - `cpp_coincident_combine_matrix_does_not_modify_input`
  - verifies AB/BA `OR`/`NOT`/`AND`/`XOR` calls do not mutate either input
    across reversed and rotated closed-polyline variants for both coincident
    fixture families.
- Added FFI-side counterpart for full coincident combine immutability matrix
  coverage:
  - `pline_boolean_coincident_full_matrix_does_not_modify_input_cpp_parity`
  - verifies `cavc_pline_boolean` AB/BA `OR`/`NOT`/`AND`/`XOR` calls do not
    mutate either input across reversed and rotated closed-polyline variants
    for both coincident fixture families.
- Added FFI options-path counterpart for full coincident combine immutability
  matrix coverage:
  - `pline_boolean_options_coincident_full_matrix_does_not_modify_input_cpp_parity`
  - verifies `cavc_pline_boolean` with `cavc_pline_boolean_o` AB/BA
    `OR`/`NOT`/`AND`/`XOR` calls do not mutate either input across reversed and
    rotated closed-polyline variants for both coincident fixture families.
- Added source-side Rust options-path counterpart for full coincident combine
  immutability matrix coverage:
  - `cpp_coincident_combine_options_full_matrix_does_not_modify_input`
  - verifies `boolean_opt` AB/BA `OR`/`NOT`/`AND`/`XOR` calls do not mutate
    either input across reversed and rotated closed-polyline variants for both
    coincident fixture families.
- Added source-aligned primitive parity probes for coincident-arc single-endpoint
  touch branches in old C++ `intrPlineSegs`:
  - `arc_arc_coincident_touch_only_at_arc1_start`
  - `arc_arc_coincident_touch_only_at_arc2_start`
  - verifies one-intersect endpoint behavior when only one coincident-arc end
    touches (no overlap span), including parameter inversion and reversed
    second-arc direction checks.
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
- Added non-zero-segment-index collection-level counterpart for coincident-arc
  disjoint-sweep no-intersect branch:
  - `coincident_arc_disjoint_sweeps_no_intersects_collection_level_nonzero_index_role_flip`
  - verifies no-intersect behavior remains stable when arc segments are not at
    index 0, including parameter inversion and reversed second-arc direction.
- Added distinct-index counterpart for the same disjoint-sweep no-intersect
  branch:
  - `coincident_arc_disjoint_sweeps_no_intersects_collection_level_distinct_nonzero_indexes_role_flip`
  - verifies no-intersect behavior remains stable with distinct non-zero arc
    indexes under parameter inversion and reversed second-arc direction.
- Added collection-level full-overlap parity guard for coincident arcs where
  one arc is fully overlapped by the other:
  - `coincident_arc_full_overlap_collection_level_ordering`
  - verifies pure overlap output (no basic intersects) plus overlap-endpoint
  ordering behavior under parameter inversion and reversed second-arc
  direction.
- Added non-zero-segment-index collection-level counterpart for coincident
  full-overlap branch:
  - `coincident_arc_full_overlap_collection_level_nonzero_index_role_flip`
  - verifies full-overlap behavior remains stable when arc segments are not at
    index 0, including parameter inversion and reversed second-arc direction.
- Added distinct-index role-inversion counterpart for coincident full-overlap
  branch:
  - `coincident_arc_full_overlap_collection_level_distinct_nonzero_indexes_role_flip`
  - verifies explicit `start_index1/start_index2` role swapping under parameter
    inversion when the overlapping arc segments have different non-zero
    indexes.
- Added collection-level partial-overlap parity guard for coincident arcs where
  overlap exists but neither arc fully contains the other:
  - `coincident_arc_partial_overlap_collection_level_ordering`
  - verifies pure overlap output (no basic intersects) plus overlap-endpoint
    ordering behavior under parameter inversion and reversed second-arc
    direction.
- Added non-zero-segment-index collection-level counterpart for coincident
  partial-overlap branch:
  - `coincident_arc_partial_overlap_collection_level_nonzero_index_role_flip`
  - verifies partial-overlap behavior remains stable when arc segments are not
    at index 0, including parameter inversion and reversed second-arc
    direction.
- Added distinct-index role-inversion counterpart for coincident
  partial-overlap branch:
  - `coincident_arc_partial_overlap_collection_level_distinct_nonzero_indexes_role_flip`
  - verifies explicit `start_index1/start_index2` role swapping under parameter
    inversion when the overlapping arc segments have different non-zero
    indexes.
- Added collection-level one-endpoint parity guards for coincident-arc
  single-touch branches:
  - `coincident_arc_touch_only_at_arc1_start_collection_level`
  - `coincident_arc_touch_only_at_arc2_start_collection_level`
  - verifies `find_intersects` yields exactly one basic endpoint intersect with
    no overlap, including parameter inversion and reversed second-arc direction.
- Added non-zero-segment-index collection-level counterpart for the
  `arc2 end == arc1 start` single-touch branch:
  - `coincident_arc_touch_only_at_arc1_start_collection_level_nonzero_index_role_flip`
  - verifies single-touch behavior remains stable when arc segments are not at
    index 0, including parameter inversion and reversed second-arc direction.
- Added distinct-index role-inversion counterpart for the same single-touch
  branch:
  - `coincident_arc_touch_only_at_arc1_start_collection_level_distinct_nonzero_indexes_role_flip`
  - verifies explicit `start_index1/start_index2` role swapping under parameter
    inversion when the intersecting arc segments have different non-zero
    indexes.
- Added non-zero-segment-index collection-level counterpart for the
  `arc2 start == arc1 end` single-touch branch:
  - `coincident_arc_touch_only_at_arc2_start_collection_level_nonzero_index_role_flip`
  - verifies single-touch behavior remains stable when arc segments are not at
    index 0, including parameter inversion and reversed second-arc direction.
- Added distinct-index role-inversion counterpart for the same single-touch
  branch:
  - `coincident_arc_touch_only_at_arc2_start_collection_level_distinct_nonzero_indexes_role_flip`
  - verifies explicit `start_index1/start_index2` role swapping under parameter
    inversion when the intersecting arc segments have different non-zero
    indexes.
- Added collection-level parity guard for coincident-arc dual-endpoint touch
  (`TwoIntersects`) branch:
  - `coincident_arc_end_points_touch_collection_level`
  - verifies exactly two basic endpoint intersects with no overlap, including
    parameter inversion and reversed second-arc direction while preserving
    second-segment ordering semantics.
- Added non-zero-segment-index collection-level counterpart for coincident
  dual-endpoint touch (`TwoIntersects`) branch:
  - `coincident_arc_end_points_touch_collection_level_nonzero_index_role_flip`
  - verifies stable dual-endpoint behavior when arc segments are not at index
    0, including parameter inversion and reversed second-arc direction.
- Added distinct-index role-inversion counterpart for coincident dual-endpoint
  touch (`TwoIntersects`) branch:
  - `coincident_arc_end_points_touch_collection_level_distinct_nonzero_indexes_role_flip`
  - verifies explicit `start_index1/start_index2` role swapping under parameter
    inversion when the intersecting arc segments have different non-zero
    indexes.
- Added collection-level guard for the opposite-direction endpoint-touch bug
  geometry (issue #42):
  - `opposite_direction_arc_end_touch_collection_level_bug_guard`
  - verifies one-endpoint intersect remains anchored to the shared endpoint
    under parameter inversion and reversed second-arc direction.
- Added non-zero-segment-index collection-level counterpart for the same
  opposite-direction endpoint-touch bug geometry:
  - `opposite_direction_arc_end_touch_collection_level_bug_guard_nonzero_index_role_flip`
  - verifies endpoint anchoring remains stable when arc segments are not at
    index 0, including parameter inversion and reversed second-arc direction.
- Added distinct-index role-inversion counterpart for the same opposite-
  direction endpoint-touch bug geometry:
  - `opposite_direction_arc_end_touch_collection_level_bug_guard_distinct_nonzero_indexes_role_flip`
  - verifies explicit `start_index1/start_index2` role swapping under parameter
    inversion while preserving endpoint anchoring when the intersecting arc
    segments have different non-zero indexes.
- Added generic (non `closed_pline1/2`-specific) zero-length-lead non-zero-index
  role-flip symmetry guards for both-closed start-index-rotated branch
  families:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - verifies AB/BA role-swapped index attribution under non-zero rotated-side
    index shift while preserving branch-specific overlap endpoint semantics
    (same-order, endpoint-set equivalence, or reversed-order swap) and
    branch-expected basic/overlap counts.
- Added the remaining wrap-around generic (non `closed_pline1/2`-specific)
  zero-length-lead non-zero-index role-flip symmetry guards:
  - `wrap_around_overlap_endpoint_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - closes the generic both-closed start-index-rotation role-flip
    zero-length-lead matrix for this branch family set (no missing counterparts
    in `pline_intersects`).
- Added closed-side-specific (non both-closed) zero-length-lead non-zero-index
  role-flip symmetry guards for non-circle wrap-around dedup start-index
  rotation branches:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline1_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline2_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline1_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline2_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - closes this deduplication sub-matrix (`same/reversed` x `closed_pline1/2`)
    for `*_start_index_rotation_role_flip_symmetry` vs
    `*_start_index_rotation_zero_length_lead_role_flip_symmetry`
    counterparts.
- Added zero-length-lead non-zero-index role-flip symmetry guards for the
  remaining non-circle wrap-around closure-basic start-index-rotation families:
  - `wrap_around_non_circle_arc_overlap_closed_side_reversed_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_normal_closed_side_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_side_reversed_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - closes the full `wrap_around_non_circle_arc_overlap*` collection-level
    `*_start_index_rotation_role_flip_symmetry` vs
    `*_start_index_rotation_zero_length_lead_role_flip_symmetry` counterpart
    matrix (`MISSING_COUNT=0` in `pline_intersects` scan).
- Added zero-length-lead non-zero-index role-flip symmetry guards for
  wrap-around overlap-endpoint dedup closed-side start-index-rotation families:
  - `wrap_around_overlap_endpoint_deduplication_closed_pline1_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_deduplication_closed_pline2_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline1_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline2_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - verifies rotated closed-side non-zero index attribution while preserving
    overlap-only endpoint-dedup branch semantics and AB/BA role inversion.
- Added zero-length-lead non-zero-index role-flip symmetry guards for
  non-circle partial-overlap reversed-endpoint-order closure-basic rotated
  closed-side families:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - verifies `1 basic + 1 overlap` closure-basic behavior stays stable while
    rotated closed-side index attribution shifts to non-zero positions.
- Completion checkpoint:
  - collection-level `*_start_index_rotation_role_flip_symmetry` functions now
    all have explicit
    `*_start_index_rotation_zero_length_lead_role_flip_symmetry` counterparts in
    `pline_intersects` (`BASE_COUNT=32`, `MISSING_COUNT=0`).
- Added non-zero-open-index role-flip symmetry guards for non-circle
  reversed-endpoint-order closure-basic branches:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_role_flip_symmetry_nonzero_open_index`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_role_flip_symmetry_nonzero_open_index`
  - verifies open-side zero-length lead index shift does not change
    branch-expected `1 basic + 1 overlap` behavior, and AB/BA role inversion
    continues to preserve basic-point equivalence with reversed overlap endpoint
    ordering.
- Added non-zero-open-index role-flip symmetry guards for non-circle
  reversed-endpoint-order closure-basic-`intersect` branches:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect_role_flip_symmetry_nonzero_open_index`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect_role_flip_symmetry_nonzero_open_index`
  - verifies open-side zero-length lead index shift preserves explicit
    closure-basic-`intersect` branch behavior (`1 basic + 1 overlap`), keeps
    AB/BA role-swapped index mapping stable, and preserves the same
    source-aligned overlap endpoint-order inversion.
- Added non-zero-open-index role-flip symmetry guards for non-circle
  wrap-around same-order closed-`pline2` closure-basic branches:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_role_flip_symmetry_nonzero_open_index`
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect_role_flip_symmetry_nonzero_open_index`
  - verifies open-side zero-length lead index shift preserves explicit
    closure-basic and closure-basic-`intersect` branch behavior (`1 basic + 1 overlap`),
    keeps AB/BA role-swapped index mapping stable, and keeps the same
    source-aligned overlap endpoint ordering under role inversion.
- Added non-zero-open-index role-flip symmetry guards for remaining wrap-around
  open-side-reversed closure-basic naming families:
  - `wrap_around_non_circle_arc_overlap_closed_side_reversed_closure_basic_role_flip_symmetry_nonzero_open_index`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_role_flip_symmetry_nonzero_open_index`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect_role_flip_symmetry_nonzero_open_index`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_role_flip_symmetry_nonzero_open_index`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_role_flip_symmetry_nonzero_open_index`
  - verifies open-side zero-length lead index shift preserves branch-expected
    `1 basic + 1 overlap` behavior, preserves AB/BA role-swapped index
    attribution, and keeps branch-specific overlap endpoint ordering
    semantics (same-order for closed-side-reversed, swapped-order for
    normal-closed-side variants).
- Completion checkpoint:
  - collection-level `*_closure_basic*_role_flip_symmetry` functions now all
    have explicit `*_nonzero_open_index` counterparts in `pline_intersects`
    (`BASE_COUNT=11`, `MISSING_COUNT=0`).
- Added Rust options-path counterpart for non-circle wrap-around same-order
  closed-`pline2` nonzero-open-index closure-basic branch:
  - `cpp_wrap_around_same_order_closed_pline2_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, index attribution, points, AB/BA role
    symmetry, and input immutability) when open-side segment indices are
    shifted off zero via a zero-length lead.
- Added Rust options-path counterpart for non-circle reversed-endpoint-order
  closure-basic branches under nonzero-open-index fixtures:
  - `cpp_reversed_endpoint_closure_basic_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs for both closed-`pline1` and closed-`pline2`
    role variants (counts, AB/BA role inversion, endpoint-set parity,
    nonzero open-side index attribution, and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle
  reversed-endpoint-order closure-basic-`intersect` branches under
  nonzero-open-index fixtures:
  - `cpp_reversed_endpoint_closure_basic_intersect_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs for both closed-`pline1` and closed-`pline2`
    role variants (counts, AB/BA role inversion, endpoint-set parity,
    nonzero open-side index attribution, and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle
  reversed-endpoint-order closure-basic-`intersect` start-index-rotation
  zero-length-lead fixtures:
  - `cpp_reversed_endpoint_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs for both closed-`pline1` and closed-`pline2`
    role variants (counts, AB/BA role inversion, reversed-endpoint overlap
    ordering, zero-length-lead index attribution semantics, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  same-order closed-`pline2` closure-basic-`intersect` start-index-rotation
  zero-length-lead fixture:
  - `cpp_wrap_around_same_order_closed_pline2_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, same-order overlap
    endpoint ordering, rotated zero-length-lead index attribution semantics,
    and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` closure-basic-`intersect`
  start-index-rotation zero-length-lead fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, branch-specific
    overlap endpoint ordering, rotated zero-length-lead index attribution
    semantics, and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side
  closure-basic-`intersect` start-index-rotation zero-length-lead fixture:
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, branch-specific
    overlap endpoint ordering, rotated zero-length-lead index attribution
    semantics, and input immutability).
- Added Rust options-path counterpart for non-circle wrap-around
  open-side-reversed + closed-side-reversed start-index-rotation
  zero-length-lead role-flip fixture:
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, branch-specific
    overlap endpoint ordering, rotated zero-length-lead index attribution
    semantics, and input immutability).
- Added Rust options-path counterpart for non-circle wrap-around same-order
  closed-`pline2` closure-basic start-index-rotation zero-length-lead fixture:
  - `cpp_wrap_around_same_order_closed_pline2_closure_basic_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, same-order overlap
    endpoint ordering, rotated zero-length-lead index attribution semantics,
    and input immutability).
- Added Rust options-path counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` closure-basic start-index-rotation
  zero-length-lead fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, branch-specific
    overlap endpoint ordering, rotated zero-length-lead index attribution
    semantics, and input immutability).
- Added Rust options-path counterpart for non-circle wrap-around
  open-side-reversed normal-closed-side closure-basic start-index-rotation
  zero-length-lead fixture:
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, branch-specific
    overlap endpoint ordering, rotated zero-length-lead index attribution
    semantics, and input immutability).
- Added Rust options-path counterpart for non-circle reversed-endpoint-order
  closure-basic start-index-rotation zero-length-lead fixtures:
  - `cpp_reversed_endpoint_closure_basic_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs for closed-`pline1` and closed-`pline2` role
    variants (counts, AB/BA role inversion, reversed overlap endpoint ordering,
    rotated zero-length-lead closed-side index attribution, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` closure-basic nonzero-open-index fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, nonzero-open-index
    attribution semantics, branch-specific overlap endpoint ordering, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` closure-basic-`intersect`
  nonzero-open-index fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_intersect_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, nonzero-open-index
    attribution semantics, branch-specific overlap endpoint ordering, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side closure-basic
  nonzero-open-index fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, nonzero-open-index
    attribution semantics, branch-specific overlap endpoint ordering, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side
  closure-basic-`intersect` nonzero-open-index fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, nonzero-open-index
    attribution semantics, branch-specific overlap endpoint ordering, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  same-order closed-`pline2` closure-basic-`intersect`
  nonzero-open-index fixture:
  - `cpp_wrap_around_same_order_closed_pline2_closure_basic_intersect_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, nonzero-open-index
    attribution semantics, same-order overlap endpoint ordering, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  same-order closed-`pline2` closure-basic nonzero-open-index fixture:
  - `cpp_wrap_around_same_order_closed_pline2_closure_basic_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, nonzero-open-index
    attribution semantics, same-order overlap endpoint ordering, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side closure-basic
  start-index-rotation zero-length-lead fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated
    zero-length-lead index attribution semantics, branch-specific overlap
    endpoint ordering, and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side closure-basic
  start-index-rotation fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_options_matrix_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated index
    attribution semantics, branch-specific overlap endpoint ordering, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side
  closure-basic-`intersect` start-index-rotation fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_options_matrix_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated index
    attribution semantics, branch-specific overlap endpoint ordering, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side closure-basic
  fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_options_matrix_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, branch-specific
    overlap endpoint ordering, index attribution semantics, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side
  closure-basic-`intersect` fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_options_matrix_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, branch-specific
    overlap endpoint ordering, index attribution semantics, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side closure-basic
  role-flip fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_role_flip_options_matrix_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, role-flip ordering
    semantics, index attribution semantics, and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side
  closure-basic-`intersect` start-index-rotation zero-length-lead fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated
    zero-length-lead index attribution semantics, branch-specific overlap
    endpoint ordering, and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side
  closure-basic-`intersect` role-flip fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_role_flip_options_matrix_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, role-flip ordering
    semantics, index attribution semantics, and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side
  closure-basic-`intersect` role-flip nonzero-open-index fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_role_flip_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, nonzero-open-index
    attribution semantics, role-flip ordering semantics, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side closure-basic
  role-flip nonzero-open-index fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_role_flip_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, nonzero-open-index
    attribution semantics, role-flip ordering semantics, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side closure-basic
  start-index-rotation role-flip fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_role_flip_options_matrix_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated index
    attribution semantics, role-flip ordering semantics, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side
  closure-basic-`intersect` start-index-rotation role-flip fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_role_flip_options_matrix_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated index
    attribution semantics, role-flip ordering semantics, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side closure-basic
  start-index-rotation zero-length-lead role-flip fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_zero_length_lead_role_flip_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated
    zero-length-lead index attribution semantics, role-flip ordering semantics,
    and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side
  closure-basic-`intersect` start-index-rotation zero-length-lead role-flip
  fixture:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated
    zero-length-lead index attribution semantics, role-flip ordering semantics,
    and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-side-reversed start-index-rotation zero-length-lead
  role-flip fixture:
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_start_index_rotation_zero_length_lead_role_flip_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated
    zero-length-lead index attribution semantics, role-flip ordering semantics,
    and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-side-reversed start-index-rotation role-flip
  fixture:
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_start_index_rotation_role_flip_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated
    index-attribution semantics, role-flip ordering semantics, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  open-side-reversed closed-side-reversed role-flip nonzero-open-index fixture:
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_role_flip_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, nonzero-open-index
    attribution semantics, role-flip ordering semantics, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  closed-side-reversed closure-basic role-flip fixture:
  - `cpp_wrap_around_closed_side_reversed_closure_basic_role_flip_options_matrix_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, role-flip ordering
    semantics, index-attribution semantics, and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  closed-side-reversed closure-basic start-index-rotation role-flip fixture:
  - `cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_role_flip_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated
    index-attribution semantics, role-flip ordering semantics, and input
    immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  closed-side-reversed closure-basic start-index-rotation zero-length-lead
  role-flip fixture:
  - `cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_zero_length_lead_role_flip_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, rotated
    zero-length-lead index-attribution semantics, role-flip ordering semantics,
    and input immutability).
- Added Rust options-path canonical-name counterpart for non-circle wrap-around
  closed-side-reversed closure-basic role-flip nonzero-open-index fixture:
  - `cpp_wrap_around_closed_side_reversed_closure_basic_role_flip_nonzero_open_index_options_parity`
  - verifies `find_intersects_opt` remains aligned with default-path
    `find_intersects` outputs (counts, AB/BA role inversion, nonzero-open-index
    attribution semantics, role-flip ordering semantics, and input
    immutability).

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

---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 60
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, start-index-rotation, zero-length-lead, role-flip, closure-basic, wrap-around, endpoint-dedup, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: prior wrap-around non-circle zero-length role-flip matrix coverage
provides:
  - explicit zero-length-lead role-flip counterparts for all collection-level `*_start_index_rotation_role_flip_symmetry` branches in `pline_intersects`
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-60-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Insert zero-length duplicates on rotated closed-side non-overlap vertices, so branch-level overlap/basic expectations stay stable while only start-index attribution shifts."
requirements-completed: [PAR-273]
duration: 22min
completed: 2026-05-15
---

# Plan 99-60 Summary

## Completed

- Added wrap-around overlap-endpoint dedup zero-length-lead role-flip guards
  for rotated closed-side branches:
  - `wrap_around_overlap_endpoint_deduplication_closed_pline1_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_deduplication_closed_pline2_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline1_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline2_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Added non-circle partial-overlap reversed-endpoint-order closure-basic
  zero-length-lead role-flip guards for rotated closed-side branches:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Branch semantics preserved:
  - wrap-around endpoint-dedup branches remain `1 overlap + 0 basic`;
  - closure-basic reversed-endpoint-order branches remain `1 overlap + 1 basic`;
  - AB/BA role inversion and overlap endpoint-order behavior remain branch-aligned.
- Counterpart scan over all collection-level
  `*_start_index_rotation_role_flip_symmetry` names in `pline_intersects`
  reports full closure:
  - `BASE_COUNT=32`
  - `MISSING_COUNT=0`

## Verification

- `cargo test -p cavalier_contours start_index_rotation_zero_length_lead_role_flip_symmetry -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.

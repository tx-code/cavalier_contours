---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 59
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, wrap-around, non-circle, closure-basic, open-side-reversed, closed-side-reversed, start-index-rotation, role-flip, zero-length-lead, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: wrap-around non-circle dedup and partial closure-basic zero-length-lead role-flip guards
provides:
  - zero-length-lead non-zero-index role-flip guards for remaining wrap-around non-circle closure-basic start-index-rotation families
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-59-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "For closure-basic rotated variants, insert zero-length duplicates on rotated closed-side non-overlap vertices so basic+overlap branch semantics stay stable and only start-index attribution shifts."
requirements-completed: [PAR-273]
duration: 18min
completed: 2026-05-15
---

# Plan 99-59 Summary

## Completed

- Added collection-level non-zero-index role-flip guards for closure-basic
  start-index-rotated branches:
  - `wrap_around_non_circle_arc_overlap_closed_side_reversed_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_normal_closed_side_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_side_reversed_start_index_rotation_zero_length_lead_role_flip_symmetry`
- These probes verify AB/BA role inversion under zero-length lead index shift
  keeps branch-expected `1 basic + 1 overlap` behavior, non-zero rotated-side
  index attribution, and branch-specific overlap endpoint-order semantics.
- Counterpart scan over
  `wrap_around_non_circle_arc_overlap*_start_index_rotation_role_flip_symmetry`
  now reports no missing
  `*_start_index_rotation_zero_length_lead_role_flip_symmetry` counterpart
  (`MISSING_COUNT=0`).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this completion.

## Verification

- `cargo test -p cavalier_contours start_index_rotation_zero_length_lead_role_flip_symmetry -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.

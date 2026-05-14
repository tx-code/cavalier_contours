---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 55
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, wrap-around, non-circle, both-closed, same-order, reversed-order, closed-pline1-rotation, closed-pline2-rotation, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: both-closed non-circle wrap-around dedup role-flip symmetry guards for explicit closed-pline1/2 start-index-rotated branches
provides:
  - zero-length-lead non-zero-index role-flip guards for both-closed non-circle wrap-around dedup closed-pline1/2-rotated branches
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-55-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "For zero-length-lead variants, duplicate non-overlap rotated-closed vertices so branch behavior stays one-overlap/zero-basic and only non-zero start-index attribution changes."
requirements-completed: [PAR-277, PAR-278]
duration: 11min
completed: 2026-05-15
---

# Plan 99-55 Summary

## Completed

- Added collection-level non-zero-index role-flip guards for same-order both-closed
  non-circle wrap-around dedup rotated paths:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
- Added collection-level non-zero-index role-flip guards for reversed-order both-closed
  non-circle wrap-around dedup rotated paths:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
- These probes verify AB/BA role inversion under zero-length lead index shift
  still keeps:
  - one overlap + zero basic intersects for all four branches,
  - explicit non-zero start-index attribution on the rotated side,
  - branch-aligned overlap endpoint semantics:
    same-order keeps endpoint ordering;
    reversed-order keeps endpoint-order swap behavior.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with closed-pline1/2-rotated
  zero-length-lead evidence for same-order and reversed-order non-circle
  both-closed wrap-around dedup branches.

## Verification

- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

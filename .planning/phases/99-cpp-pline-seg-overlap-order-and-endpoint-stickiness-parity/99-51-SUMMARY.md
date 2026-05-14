---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 51
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, both-closed, adjacent-dedup, opposing-direction, non-circle, closed-pline1-rotation, closed-pline2-rotation, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: both-closed adjacent-dedup role-flip symmetry guards for explicit closed-pline1/2 start-index-rotated branches
provides:
  - zero-length-lead non-zero-index role-flip symmetry guards for both-closed adjacent-dedup closed-pline1/2-rotated branches
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-51-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "For zero-length-lead variants, duplicate non-overlap vertices on rotated closed paths so overlap-adjacent dedup assertions remain branch-focused and do not introduce unrelated basic intersects."
requirements-completed: [PAR-273]
duration: 14min
completed: 2026-05-15
---

# Plan 99-51 Summary

## Completed

- Added collection-level non-zero-index role-flip guards for opposing-direction
  both-closed adjacent-dedup rotated paths:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
- Added collection-level non-zero-index role-flip guards for non-circle
  both-closed adjacent-dedup rotated paths:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
- These probes verify AB/BA role inversion under zero-length lead index shift
  still keeps:
  - one overlap + zero basic intersects for all four branches,
  - explicit non-zero start-index attribution on the rotated side,
  - branch-aligned overlap endpoint behavior (set-equivalent for opposing-direction,
    stable ordering for non-circle adjacent dedup).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with closed-pline1/2-rotated
  zero-length-lead evidence for both adjacent-dedup branch families.

## Verification

- `cargo test -p cavalier_contours opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo test -p cavalier_contours opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

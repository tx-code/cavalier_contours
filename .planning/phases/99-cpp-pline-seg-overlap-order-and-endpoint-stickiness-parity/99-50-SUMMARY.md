---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 50
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, closed-path, reversed-endpoint-order, closed-pline1-rotation, closed-pline2-rotation, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: non-zero-index role-flip guards for reversed-endpoint-order both-closed start-index-rotated and closed-pline1/2-rotated branches
provides:
  - zero-length-lead non-zero-index role-flip guards for reversed-endpoint-order both-closed closed-pline1/2-rotated branches
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-50-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Apply zero-length lead duplication on non-overlap arc-start vertices in rotated closed-pline1/2 inputs to preserve three-basic-intersect structure while shifting overlap index attribution to non-zero paths."
requirements-completed: [PAR-298]
duration: 13min
completed: 2026-05-15
---

# Plan 99-50 Summary

## Completed

- Added collection-level non-zero-index role-flip guards:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
- These probes verify AB/BA role inversion in reversed-endpoint-order both-closed
  closed-pline1/2-rotated branches under zero-length lead shift still keeps:
  - one overlap + three basic intersects on both sides,
  - AB/BA role-swapped basic and overlap start-index mapping,
  - overlap endpoint-order reversal (`AB point1/point2 == BA point2/point1`),
  - overlap-endpoint basic dedup invariant (no basic at `(3, 1)`).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with closed-pline1/2-rotated
  zero-length-lead evidence for the reversed-endpoint-order both-closed branch.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

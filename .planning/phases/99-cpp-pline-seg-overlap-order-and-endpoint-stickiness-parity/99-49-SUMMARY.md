---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 49
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, closed-path, arc1-reverse, arc2-reverse, closed-pline1-rotation, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: zero-length-lead non-zero-index role-flip guard for bounded both-closed arc1/arc2-reverse closed-pline2-rotated branches
provides:
  - zero-length-lead non-zero-index role-flip guards for bounded both-closed arc1/arc2-reverse closed-pline1-rotated branches
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-49-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Move zero-length lead insertion onto non-overlap arc-start vertices in the closed-pline1-rotated arc2 branch to avoid introducing extra closure-endpoint basic intersects while preserving non-zero overlap start-index attribution."
requirements-completed: [PAR-305, PAR-306]
duration: 11min
completed: 2026-05-15
---

# Plan 99-49 Summary

## Completed

- Added collection-level non-zero-index role-flip guards:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
- These probes verify AB/BA role inversion in closed-pline1-rotated arc1/arc2-reverse
  branches under zero-length lead shift still keeps:
  - arc1: one overlap + one basic intersect on both sides,
  - arc2: one overlap + zero basic intersects on both sides,
  - role-swapped start-index mapping,
  - non-zero start indexes on the closed-pline1 side,
  - source-aligned overlap endpoint-order reversal (`AB point1/point2 == BA point2/point1`).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with closed-pline1 rotated + zero-length-lead
  evidence for both `arc1 reverse` and `arc2 reverse` branch families.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

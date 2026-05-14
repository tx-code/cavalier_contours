---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 48
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, closed-path, both-reverse, closed-pline1-rotation, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: zero-length-lead non-zero-index role-flip guard for bounded both-closed both-reverse closed-pline2-rotated branch
provides:
  - zero-length-lead non-zero-index role-flip guard for bounded both-closed both-reverse closed-pline1-rotated branch
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-48-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use duplicate lead vertices on non-overlap arc-start vertices in both closed polylines so the branch remains one-basic + one-overlap while shifting index attribution to non-zero values in the closed-pline1-rotated counterpart."
requirements-completed: [PAR-307]
duration: 6min
completed: 2026-05-15
---

# Plan 99-48 Summary

## Completed

- Added collection-level non-zero-index role-flip guard:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
- The probe verifies AB/BA role inversion in the closed-pline1-rotated both-reverse
  branch under zero-length lead shift still keeps:
  - one overlap and one basic intersect on both sides,
  - role-swapped basic/overlap start-index mapping,
  - non-zero start indexes for both intersect kinds,
  - source-aligned overlap endpoint-order stability (`AB point1/point2 == BA point1/point2`).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this rotated+shifted
  closed-pline1 evidence for the `both reverse` branch family.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_both_reverse_dir_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

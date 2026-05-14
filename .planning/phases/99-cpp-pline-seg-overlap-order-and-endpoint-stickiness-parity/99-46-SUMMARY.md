---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 46
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, closed-path, arc1-reverse, closed-pline2-rotation, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: zero-length-lead non-zero-index role-flip guard for bounded both-closed arc2-reverse closed-pline2-rotated branch
provides:
  - zero-length-lead non-zero-index role-flip guard for bounded both-closed arc1-reverse closed-pline2-rotated branch
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-46-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Apply zero-length lead shift on non-overlap arc-start vertices to preserve the one-basic + one-overlap branch while exercising closed-pline2-rotation role-flip semantics."
requirements-completed: [PAR-306]
duration: 7min
completed: 2026-05-15
---

# Plan 99-46 Summary

## Completed

- Added collection-level non-zero-index role-flip guard:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
- The probe verifies AB/BA role inversion in the closed-pline2-rotated arc1-reverse
  branch under zero-length lead shift still keeps:
  - one overlap and one basic intersect on both sides,
  - role-swapped basic/overlap start-index mapping,
  - non-zero start indexes for both intersect kinds,
  - source-aligned overlap endpoint reversal (`AB point1/point2 == BA point2/point1`).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this rotated+shifted
  closed-pline2 evidence for the `arc1 reverse` branch family.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_arc1_reverse_dir_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

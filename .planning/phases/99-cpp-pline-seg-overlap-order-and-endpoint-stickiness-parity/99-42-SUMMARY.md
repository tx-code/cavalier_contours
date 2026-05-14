---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 42
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, closed-path, arc2-reverse, start-index-rotation, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: zero-length-lead non-zero-index role-flip guard for bounded both-closed arc2-reverse branch
provides:
  - zero-length-lead non-zero-index role-flip guard for bounded both-closed arc2-reverse start-index-rotated branch
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-42-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use a non-overlap first vertex for the shifted closed side to avoid introducing extra basic intersects while exercising combined start-index rotation and zero-length lead shift."
requirements-completed: [PAR-302]
duration: 9min
completed: 2026-05-15
---

# Plan 99-42 Summary

## Completed

- Added collection-level non-zero-index role-flip guard:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
- The probe combines closed-side start-index rotation with zero-length lead
  shift and verifies AB/BA role inversion still keeps:
  - one overlap and zero basic intersects on both sides,
  - role-swapped overlap start-index mapping,
  - non-zero overlap start indexes,
  - source-aligned overlap endpoint reversal (`AB point1/point2 == BA point2/point1`).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` to include this rotated+shifted
  both-closed evidence for the `arc2 reverse` branch family.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

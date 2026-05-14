---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 36
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, open-path, arc1-reverse, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: zero-length-lead non-zero-index role-flip guard for bounded reversed-endpoint-order open-path branch
provides:
  - zero-length-lead non-zero-index role-flip guard for bounded arc1-reverse open-path branch
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-36-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Preserve original arc geometry by inserting a zero-bulge duplicate lead vertex before the original first arc vertex (which retains the original bulge)."
requirements-completed: [PAR-296]
duration: 12min
completed: 2026-05-15
---

# Plan 99-36 Summary

## Completed

- Added collection-level non-zero-index role-flip guard:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_with_adjacent_line_flip_zero_length_lead_role_flip_symmetry`
- The probe prepends zero-length lead segments while preserving arc bulges and
  verifies AB/BA role inversion still keeps:
  - one overlap and one basic intersect on both sides,
  - role-swapped start-index mapping for both basic and overlap intersects,
  - non-zero start indexes for both intersect kinds,
  - stable overlap endpoint sets (order may reverse).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with non-zero-index open-path
  evidence for the `arc1 reverse` branch family.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_arc1_reverse_dir_with_adjacent_line_flip_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

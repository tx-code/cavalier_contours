---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 33
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, arc-overlap, role-flip]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: collection-level non-zero-index guards for line-line false none true outcomes
provides:
  - collection-level role-flip symmetry guards for bounded mixed arc+adjacent-line open-path branches
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-33-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "For AB/BA role inversion in bounded open-path overlap branches, compare overlap endpoint sets (allowing direction-order reversal) while enforcing role-swapped start-index mapping."
requirements-completed: [PAR-293]
duration: 15min
completed: 2026-05-15
---

# Plan 99-33 Summary

## Completed

- Added two collection-level role-flip symmetry guards in
  `find_intersects_tests`:
  - `non_circle_partial_arc_overlap_arc1_reverse_dir_with_adjacent_line_flip_role_flip_symmetry`
  - `non_circle_partial_arc_overlap_both_reverse_dir_with_adjacent_line_flip_role_flip_symmetry`
- These verify AB/BA role inversion preserves:
  - overlap/basic intersect cardinality for each geometry family,
  - role-swapped start-index attribution,
  - stable overlap endpoint sets (order may reverse with segment direction).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` to include this mixed arc +
  adjacent-line role-flip evidence.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_arc1_reverse_dir_with_adjacent_line_flip_role_flip_symmetry -q` - pass.
- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_both_reverse_dir_with_adjacent_line_flip_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

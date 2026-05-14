---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 31
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, line-line, true, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: collection-level one-basic/no-overlap guards for line-line true outcomes
provides:
  - collection-level non-zero-index guards for line-line true outcomes
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-31-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Pin line-line `True` collection behavior when both intersecting segments are non-zero indexes, plus role inversion."
requirements-completed: [PAR-291]
duration: 14min
completed: 2026-05-15
---

# Plan 99-31 Summary

## Completed

- Added two collection-level non-zero-index line-line `True` guards in
  `find_intersects_tests`:
  - `line_line_true_intersection_collection_level_nonzero_indexes`
  - `line_line_true_intersection_collection_level_nonzero_indexes_flipped_roles`
- Both probes verify one `basic_intersects` entry, zero `overlapping_intersects`,
  `start_index1 == 1`, `start_index2 == 1`, and intersection point
  `(5/3, 4/3)`.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` so line-line collection-level
  `True` coverage now includes non-zero-index and role-flip evidence.

## Verification

- `cargo test -p cavalier_contours line_line_true_intersection_collection_level_nonzero_indexes -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

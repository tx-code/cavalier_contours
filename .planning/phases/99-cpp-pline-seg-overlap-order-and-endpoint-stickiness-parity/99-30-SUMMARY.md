---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 30
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, line-line, true]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: collection-level no-emission guards for line-line false and none outcomes
provides:
  - collection-level one-basic/no-overlap guards for line-line true outcomes
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-30-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Mirror C++ line-line `True` branch expectations at collection level with explicit role-flip coverage."
requirements-completed: [PAR-290]
duration: 12min
completed: 2026-05-15
---

# Plan 99-30 Summary

## Completed

- Added two collection-level line-line `True` guards in `find_intersects_tests`:
  - `line_line_true_intersection_collection_level`
  - `line_line_true_intersection_collection_level_flipped_roles`
- Both probes verify one `basic_intersects` entry, zero `overlapping_intersects`,
  index pair `(0, 0)`, and intersection point `(1.0, 1.0)`.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` to include collection-level line-line
  `True` evidence alongside existing `False`/`None` no-emission guards.

## Verification

- `cargo test -p cavalier_contours line_line_true_intersection_collection_level -q` - pass.
- `cargo test -p cavalier_contours line_line_true_intersection_collection_level_flipped_roles -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace` - pass.
- `gsd-sdk query validate.health` - healthy.

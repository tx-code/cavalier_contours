---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 32
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, line-line, false, none, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: collection-level non-zero-index guards for line-line true outcomes
provides:
  - collection-level non-zero-index guards for line-line false and none outcomes
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-32-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use zero-length lead segments to shift tested line-line candidates to non-zero indexes while preserving branch geometry."
requirements-completed: [PAR-292]
duration: 16min
completed: 2026-05-15
---

# Plan 99-32 Summary

## Completed

- Added four collection-level non-zero-index line-line no-emission guards:
  - `line_line_false_intersection_no_intersects_collection_level_nonzero_indexes`
  - `line_line_false_intersection_no_intersects_collection_level_nonzero_indexes_flipped_roles`
  - `line_line_none_parallel_no_intersects_collection_level_nonzero_indexes`
  - `line_line_none_parallel_no_intersects_collection_level_nonzero_indexes_flipped_roles`
- All four probes use zero-length lead segments to shift the evaluated segment
  to index `1`, verifying `find_intersects` still emits no basic/overlap
  results for segment-level `False` and `None` paths under role inversion.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` so line-line collection-level
  coverage now includes non-zero-index evidence for `False`, `None`, and
  `True` outcomes.

## Verification

- `cargo test -p cavalier_contours line_line_false_intersection_no_intersects_collection_level_nonzero_indexes -q` - pass.
- `cargo test -p cavalier_contours line_line_none_parallel_no_intersects_collection_level_nonzero_indexes -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.

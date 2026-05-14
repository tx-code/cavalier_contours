---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 57
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, wrap-around, both-closed, start-index-rotation, role-flip, zero-length-lead, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: generic both-closed start-index-rotation zero-length-lead role-flip guards for non-wrap-around families
provides:
  - generic wrap-around both-closed start-index-rotation zero-length-lead role-flip guards for overlap-endpoint and non-circle branch families
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-57-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Duplicate non-overlap rotated-side vertices in zero-length-lead variants so only start-index attribution shifts while branch-specific overlap/basic behavior remains unchanged."
requirements-completed: [PAR-273]
duration: 14min
completed: 2026-05-15
---

# Plan 99-57 Summary

## Completed

- Added generic collection-level non-zero-index role-flip guard for both-closed
  wrap-around overlap-endpoint dedup start-index-rotated branch:
  - `wrap_around_overlap_endpoint_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Added generic collection-level non-zero-index role-flip guard for both-closed
  wrap-around mixed line/arc overlap-endpoint-adjacent dedup start-index-rotated branch:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Added generic collection-level non-zero-index role-flip guard for both-closed
  wrap-around non-circle same-order start-index-rotated branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Added generic collection-level non-zero-index role-flip guard for both-closed
  wrap-around non-circle reversed-order start-index-rotated branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
- These probes verify AB/BA role inversion under zero-length lead index shift
  still keeps:
  - branch-expected overlap/basic counts,
  - non-zero start-index attribution on the rotated side,
  - branch-specific overlap endpoint ordering behavior (same-order keep vs reversed-order swap).
- Ran counterpart scan over generic
  `*_both_closed_start_index_rotation_role_flip_symmetry` functions and confirmed
  no missing `*_zero_length_lead_role_flip_symmetry` counterpart remains.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` to record this completion and moved
  next P1 target back to the next unmapped branch family.

## Verification

- `cargo test -p cavalier_contours start_index_rotation_zero_length_lead_role_flip_symmetry -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.

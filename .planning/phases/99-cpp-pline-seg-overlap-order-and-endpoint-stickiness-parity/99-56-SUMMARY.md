---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 56
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, both-closed, start-index-rotation, role-flip, zero-length-lead, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: closed-pline1/2-rotated zero-length-lead role-flip guards for both-closed branch families
provides:
  - generic (non closed-pline1/2-specific) zero-length-lead non-zero-index role-flip guards for both-closed start-index-rotated branch families
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-56-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use zero-length leads on non-overlap rotated-side vertices so only start-index attribution shifts while branch-specific overlap/basic behavior remains unchanged."
requirements-completed: [PAR-273]
duration: 13min
completed: 2026-05-15
---

# Plan 99-56 Summary

## Completed

- Added generic collection-level non-zero-index role-flip guard for both-closed
  mixed line/arc overlap-adjacent dedup branch:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Added generic collection-level non-zero-index role-flip guard for both-closed
  opposing-direction arc-overlap adjacent-endpoint dedup branch:
  - `opposing_direction_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Added generic collection-level non-zero-index role-flip guard for both-closed
  bounded non-circle adjacent-endpoint dedup branch:
  - `non_circle_partial_arc_overlap_adjacent_endpoint_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Added generic collection-level non-zero-index role-flip guard for both-closed
  bounded non-circle reversed-endpoint-order branch:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry`
- These probes verify AB/BA role inversion under zero-length lead index shift
  still keeps:
  - branch-expected overlap/basic counts,
  - explicit non-zero start-index attribution on the rotated side,
  - branch-specific overlap endpoint semantics (same-order, endpoint-set
    equivalence, or reversed-order swap).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this generic zero-length-lead
  coverage and narrowed the next P1 target to remaining wrap-around generic
  branches.

## Verification

- `cargo test -p cavalier_contours zero_length_lead_role_flip_symmetry -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.

---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 58
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, wrap-around, non-circle, dedup, closed-side, start-index-rotation, role-flip, zero-length-lead, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: generic and both-closed zero-length-lead role-flip guards for wrap-around non-circle families
provides:
  - zero-length-lead non-zero-index role-flip guards for non-circle wrap-around dedup closed-side start-index-rotation branches (`same/reversed` x `closed_pline1/2`)
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-58-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Insert zero-length duplicates only on non-overlap rotated closed-side vertices so overlap/basic behavior stays branch-identical and only start-index attribution shifts."
requirements-completed: [PAR-273]
duration: 12min
completed: 2026-05-15
---

# Plan 99-58 Summary

## Completed

- Added collection-level non-zero-index role-flip guard for non-circle wrap-around
  dedup same-order closed-`pline1` start-index-rotated branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline1_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Added collection-level non-zero-index role-flip guard for non-circle wrap-around
  dedup same-order closed-`pline2` start-index-rotated branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline2_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Added collection-level non-zero-index role-flip guard for non-circle wrap-around
  dedup reversed-order closed-`pline1` start-index-rotated branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline1_start_index_rotation_zero_length_lead_role_flip_symmetry`
- Added collection-level non-zero-index role-flip guard for non-circle wrap-around
  dedup reversed-order closed-`pline2` start-index-rotated branch:
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline2_start_index_rotation_zero_length_lead_role_flip_symmetry`
- These probes verify AB/BA role inversion under zero-length lead index shift
  still keeps:
  - one overlap + zero basic intersects for all four branches,
  - non-zero index attribution on the rotated closed side,
  - branch-specific overlap endpoint ordering semantics (same-order keep,
    reversed-order swap).
- Counterpart scan for
  `wrap_around_non_circle_arc_overlap_deduplication_(same|reversed)_order_closed_pline[12]_start_index_rotation_role_flip_symmetry`
  now reports no missing
  `*_start_index_rotation_zero_length_lead_role_flip_symmetry` function.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this completed sub-matrix.

## Verification

- `cargo test -p cavalier_contours start_index_rotation_zero_length_lead_role_flip_symmetry -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.

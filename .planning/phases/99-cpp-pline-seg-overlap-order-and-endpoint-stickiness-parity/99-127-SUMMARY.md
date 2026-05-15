---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 127
subsystem: wrap-around-non-circle-start-index-rotation-options-path-parity
tags: [cpp-parity, find-intersects, wrap-around, non-circle, arc-overlap, dedup, role-flip, options]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: wrap-around non-circle arc-overlap dedup branch baselines
provides:
  - options-path canonical-name parity guards for wrap-around non-circle arc-overlap dedup start-index-rotation role-flip fixture matrices (same-order and reversed-order)
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-127-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Add explicit non-zero-lead role-flip options-path coverage for both same-order and reversed-order wrap-around non-circle dedup branches to keep canonical-name parity and ordering semantics pinned."
requirements-completed: [PAR-317]
duration: 10min
completed: 2026-05-15
---

# Plan 99-127 Summary

## Completed

- Added Rust options-path canonical counterparts for wrap-around non-circle
  arc-overlap dedup start-index-rotation role-flip fixture matrices:
  - `cpp_wrap_around_non_circle_arc_overlap_deduplication_same_order_start_index_rotation_role_flip_options_parity`
  - `cpp_wrap_around_non_circle_arc_overlap_deduplication_reversed_order_start_index_rotation_role_flip_options_parity`
- The new tests verify:
  - `find_intersects_opt` and default `find_intersects` parity for AB/BA outputs,
  - no-basic + one-overlap dedup behavior across:
    - `both_closed_start_index_rotation_role_flip`,
    - `both_closed_start_index_rotation_closed_pline1_role_flip`,
    - `both_closed_start_index_rotation_closed_pline2_role_flip`,
  - same-order branch keeps overlap endpoint ordering under role inversion,
  - reversed-order branch swaps overlap endpoint ordering under role inversion,
  - start-index attribution semantics and input polyline immutability.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this canonical options-path
  evidence.

## Verification

- `cargo test -p cavalier_contours cpp_wrap_around_non_circle_arc_overlap_deduplication_same_order_start_index_rotation_role_flip_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_non_circle_arc_overlap_deduplication_reversed_order_start_index_rotation_role_flip_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_deduplication_same_order_both_closed_start_index_rotation_role_flip_symmetry -- --nocapture` - pass.
- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_deduplication_reversed_order_both_closed_start_index_rotation_role_flip_symmetry -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.

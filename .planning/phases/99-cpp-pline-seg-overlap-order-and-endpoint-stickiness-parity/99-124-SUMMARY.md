---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 124
subsystem: wrap-around-overlap-endpoint-options-path-parity
tags: [cpp-parity, find-intersects, wrap-around, overlap-endpoint, dedup, zero-length-lead, role-flip, options]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: wrap-around overlap-endpoint dedup branch baselines
provides:
  - options-path canonical-name parity guard for wrap-around overlap-endpoint dedup start-index-rotation zero-length-lead role-flip fixture matrix
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-124-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Add explicit options-path coverage for zero-lead role-flip lanes in the wrap-around overlap-endpoint dedup branch to keep canonical-name parity and index semantics pinned."
requirements-completed: [PAR-314]
duration: 8min
completed: 2026-05-15
---

# Plan 99-124 Summary

## Completed

- Added Rust options-path canonical counterpart for wrap-around overlap-endpoint
  dedup start-index-rotation zero-length-lead role-flip fixture matrix:
  - `cpp_wrap_around_overlap_endpoint_dedup_start_index_rotation_zero_length_lead_role_flip_options_parity`
- The new test verifies:
  - `find_intersects_opt` and default `find_intersects` parity for AB/BA outputs,
  - no-basic + one-overlap dedup behavior across:
    - `both_closed_start_index_rotation_zero_length_lead_role_flip`,
    - `both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip`,
    - `both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip`,
  - stable overlap endpoint ordering under role inversion,
  - zero-length-lead index attribution semantics,
  - input polyline immutability.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this canonical options-path
  evidence.

## Verification

- `cargo test -p cavalier_contours cpp_wrap_around_overlap_endpoint_dedup_start_index_rotation_zero_length_lead_role_flip_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours wrap_around_overlap_endpoint_deduplication_both_closed_start_index_rotation_zero_length_lead_role_flip_symmetry -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.

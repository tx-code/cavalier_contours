---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 84
subsystem: options-path-parity
tags: [cpp-parity, find-intersects, non-circle, wrap-around, open-side-reversed, closed-pline2, normal-closed-side, closure-basic, start-index-rotation, options]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: options-path canonical-name parity guard for wrap-around open-side-reversed closed-pline2 with normal-closed-side closure-basic start-index-rotation zero-length-lead fixture
provides:
  - options-path canonical-name parity guard for wrap-around open-side-reversed closed-pline2 with normal-closed-side closure-basic start-index-rotation fixture
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-84-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Add a canonical-name options-matrix parity guard for the rotated normal-closed-side closure-basic fixture to keep options/default-path alignment explicit under closed-pline2 naming."
requirements-completed: [PAR-274]
duration: 8min
completed: 2026-05-15
---

# Plan 99-84 Summary

## Completed

- Added Rust options-path canonical counterpart for wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side closure-basic
  start-index-rotation parity:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_options_matrix_parity`
- The new test verifies:
  - `find_intersects_opt` and default `find_intersects` parity for AB/BA outputs,
  - branch-expected `1 basic + 1 overlap` behavior,
  - rotated start-index attribution semantics,
  - branch-specific overlap endpoint ordering under role inversion,
  - input polyline immutability.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this canonical options-path
  evidence.

## Verification

- `cargo test -p cavalier_contours cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_options_matrix_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours open_side_reversed_normal_closed_side -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.

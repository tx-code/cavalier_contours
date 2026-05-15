---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 74
subsystem: options-path-parity
tags: [cpp-parity, find-intersects, non-circle, wrap-around, open-side-reversed, closed-pline2, closure-basic, start-index-rotation, zero-length-lead, options]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: options-path parity guard for wrap-around same-order closed-pline2 closure-basic start-index-rotation zero-length-lead fixture
provides:
  - options-path parity guard for wrap-around open-side-reversed closed-pline2 closure-basic start-index-rotation zero-length-lead fixture
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-74-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Reuse source-aligned rotated zero-length-lead fixture from internal role-flip tests and assert explicit options/default parity in the offset parity suite."
requirements-completed: [PAR-273]
duration: 8min
completed: 2026-05-15
---

# Plan 99-74 Summary

## Completed

- Added Rust options-path counterpart for wrap-around open-side-reversed
  closed-`pline2` closure-basic start-index-rotation zero-length-lead parity:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_zero_length_lead_options_parity`
- The new test verifies:
  - `find_intersects_opt` and default `find_intersects` parity for AB/BA outputs,
  - branch-expected `1 basic + 1 overlap` behavior,
  - branch-specific overlap endpoint ordering under role inversion,
  - rotated zero-length-lead index attribution semantics on the closed side,
  - input polyline immutability.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this options-path evidence.

## Verification

- `cargo test -p cavalier_contours cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_zero_length_lead_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours open_side_reversed_closed_pline2 -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.


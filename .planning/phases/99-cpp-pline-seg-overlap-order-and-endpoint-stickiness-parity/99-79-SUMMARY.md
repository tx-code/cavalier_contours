---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 79
subsystem: options-path-parity
tags: [cpp-parity, find-intersects, non-circle, wrap-around, open-side-reversed, closed-pline2, normal-closed-side, closure-basic, nonzero-open-index, options]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: options-path canonical-name parity guard for wrap-around open-side-reversed closed-pline2 closure-basic-intersect nonzero-open-index fixture
provides:
  - options-path canonical-name parity guard for wrap-around open-side-reversed closed-pline2 with normal-closed-side closure-basic nonzero-open-index fixture
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-79-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Reuse source-aligned nonzero-open-index normal-closed-side fixture and assert explicit options/default parity in the offset parity suite under canonical closure-basic naming."
requirements-completed: [PAR-273]
duration: 8min
completed: 2026-05-15
---

# Plan 99-79 Summary

## Completed

- Added Rust options-path canonical counterpart for wrap-around
  open-side-reversed closed-`pline2` with normal-closed-side closure-basic
  nonzero-open-index parity:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_nonzero_open_index_options_parity`
- The new test verifies:
  - `find_intersects_opt` and default `find_intersects` parity for AB/BA outputs,
  - branch-expected `1 basic + 1 overlap` behavior,
  - nonzero-open-index attribution semantics,
  - branch-specific overlap endpoint ordering under role inversion,
  - input polyline immutability.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this canonical options-path
  evidence.

## Verification

- `cargo test -p cavalier_contours cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_nonzero_open_index_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours open_side_reversed_normal_closed_side -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.


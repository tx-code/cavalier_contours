---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 82
subsystem: options-path-parity
tags: [cpp-parity, find-intersects, non-circle, wrap-around, same-order, closed-pline2, closure-basic, nonzero-open-index, options]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: options-path canonical-name parity guard for wrap-around same-order closed-pline2 closure-basic-intersect nonzero-open-index fixture
provides:
  - options-path canonical-name parity guard for wrap-around same-order closed-pline2 closure-basic nonzero-open-index fixture
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-82-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Reuse source-aligned nonzero-open-index same-order fixture and assert explicit options/default parity in the offset parity suite under canonical closure-basic naming."
requirements-completed: [PAR-273]
duration: 8min
completed: 2026-05-15
---

# Plan 99-82 Summary

## Completed

- Added Rust options-path canonical counterpart for wrap-around same-order
  closed-`pline2` closure-basic nonzero-open-index parity:
  - `cpp_wrap_around_same_order_closed_pline2_closure_basic_nonzero_open_index_options_parity`
- The new test verifies:
  - `find_intersects_opt` and default `find_intersects` parity for AB/BA outputs,
  - branch-expected `1 basic + 1 overlap` behavior,
  - nonzero-open-index attribution semantics,
  - same-order overlap endpoint ordering under role inversion,
  - input polyline immutability.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this canonical options-path
  evidence.

## Verification

- `cargo test -p cavalier_contours cpp_wrap_around_same_order_closed_pline2_closure_basic_nonzero_open_index_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours same_order_closed_pline2 -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.


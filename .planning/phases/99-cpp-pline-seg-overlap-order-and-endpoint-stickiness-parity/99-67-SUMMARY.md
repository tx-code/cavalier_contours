---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 67
subsystem: options-path-parity
tags: [cpp-parity, find-intersects, non-circle, reversed-endpoint-order, closure-basic-intersect, nonzero-open-index, options]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: options-path parity guard for reversed-endpoint-order closure-basic nonzero-open-index fixtures
provides:
  - options-path canonical-name parity guard for reversed-endpoint-order closure-basic-intersect nonzero-open-index fixtures
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-67-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Keep fixtures source-aligned with the existing reversed-endpoint nonzero-open-index closure-basic probe, and add an explicit closure-basic-intersect canonical-name counterpart to avoid alias-style coverage gaps."
requirements-completed: [PAR-273]
duration: 11min
completed: 2026-05-15
---

# Plan 99-67 Summary

## Completed

- Added Rust options-path canonical-name counterpart for non-circle
  reversed-endpoint closure-basic-`intersect` nonzero-open-index parity:
  - `cpp_reversed_endpoint_closure_basic_intersect_nonzero_open_index_options_parity`
- The new test covers both closed-`pline1` and closed-`pline2` role variants
  and verifies:
  - `find_intersects_opt` and default `find_intersects` parity for AB/BA
    outputs,
  - branch-expected `1 basic + 1 overlap` behavior,
  - stable basic-point and overlap endpoint-set semantics,
  - nonzero open-side index attribution after zero-length lead insertion,
  - input polyline immutability.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this canonical-name options-path
  evidence.

## Verification

- `cargo test -p cavalier_contours cpp_reversed_endpoint_closure_basic_intersect_nonzero_open_index_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours nonzero_open_index -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.

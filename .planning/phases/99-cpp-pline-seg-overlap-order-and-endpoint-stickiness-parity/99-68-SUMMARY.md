---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 68
subsystem: options-path-parity
tags: [cpp-parity, find-intersects, non-circle, reversed-endpoint-order, closure-basic-intersect, start-index-rotation, zero-length-lead, options]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: options-path canonical-name parity guard for reversed-endpoint-order closure-basic-intersect nonzero-open-index fixtures
provides:
  - options-path canonical-name parity guard for reversed-endpoint-order closure-basic-intersect start-index-rotation zero-length-lead fixtures
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-68-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Reuse source-aligned zero-length-lead rotated fixtures from the internal canonical branch tests and assert explicit options/default parity without introducing alias wrappers."
requirements-completed: [PAR-273]
duration: 12min
completed: 2026-05-15
---

# Plan 99-68 Summary

## Completed

- Added Rust options-path canonical-name counterpart for reversed-endpoint
  closure-basic-`intersect` start-index-rotation zero-length-lead parity:
  - `cpp_reversed_endpoint_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity`
- The new test covers both closed-`pline1` and closed-`pline2` role variants
  and verifies:
  - `find_intersects_opt` and default `find_intersects` parity for AB/BA
    outputs,
  - branch-expected `1 basic + 1 overlap` behavior,
  - reversed-endpoint overlap ordering under role inversion,
  - zero-length-lead start-index attribution semantics on the rotated closed side,
  - input polyline immutability.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this canonical-name
  start-index-rotation options-path evidence.

## Verification

- `cargo test -p cavalier_contours cpp_reversed_endpoint_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours nonzero_open_index -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.

---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 65
subsystem: options-path-parity
tags: [cpp-parity, find-intersects, non-circle, wrap-around, same-order, closed-pline2, closure-basic, nonzero-open-index, options]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: canonical-name nonzero-open-index role-flip guards for remaining wrap-around open-side-reversed closure-basic families
provides:
  - options-path parity guard for wrap-around same-order closed-pline2 closure-basic nonzero-open-index fixture
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-65-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Reuse the existing open-side zero-length-lead fixture and assert default/options parity for both AB and BA direction outputs (counts, indices, points, and input immutability)."
requirements-completed: [PAR-273]
duration: 17min
completed: 2026-05-15
---

# Plan 99-65 Summary

## Completed

- Added Rust options-path counterpart test for wrap-around non-circle
  same-order closed-`pline2` closure-basic nonzero-open-index parity:
  - `cpp_wrap_around_same_order_closed_pline2_nonzero_open_index_options_parity`
- The new test verifies `find_intersects_opt` stays aligned with default-path
  `find_intersects` across AB and BA role orientations, including:
  - `1 basic + 1 overlap` counts,
  - start-index role inversion and nonzero-open-index attribution,
  - basic point equality and same-order overlap endpoint ordering,
  - options-path/default-path output equivalence,
  - input polyline immutability.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` to record this options-path parity
  addition in the deepening outcomes section.

## Verification

- `cargo test -p cavalier_contours cpp_wrap_around_same_order_closed_pline2_nonzero_open_index_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours nonzero_open_index -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.


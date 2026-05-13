---
phase: 37-capi-pline-remove-sequence-range-equivalence-parity
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, pline-suite, remove-sequence]
requires:
  - phase: 37-capi-pline-remove-sequence-range-equivalence-parity
    provides: remove-range equivalence target
provides:
  - executable remove-sequence equivalence C-API parity test
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [remove-range-equivalence]
key-files:
  created:
    - .planning/phases/37-capi-pline-remove-sequence-range-equivalence-parity/37-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Represent old remove-range behavior as deterministic ordered remove calls on current API surface."
requirements-completed: [PAR-85, PAR-86]
duration: 5min
completed: 2026-05-14
---

# Plan 37-01 Summary

## Completed

- Added test:
  - `pline_remove_sequence_equivalent_to_cpp_remove_range_parity`
- Verified vertex-level intermediate states and final empty closure.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

---
phase: 32-capi-function-surface-combine-self-matrix-parity
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, function-surface, combine-self]
requires:
  - phase: 32-capi-function-surface-combine-self-matrix-parity
    provides: function-surface combine-self target
provides:
  - executable function-surface combine-with-self matrix C-API parity test
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [function-surface-matrix-bridge]
key-files:
  created:
    - .planning/phases/32-capi-function-surface-combine-self-matrix-parity/32-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Add matrix-scoped combine-with-self invariants using source-backed circle and closed half-circle case builders."
requirements-completed: [PAR-70, PAR-71]
duration: 12min
completed: 2026-05-14
---

# Plan 32-01 Summary

## Completed

- Added plinelist vertex extraction helper for boolean outputs.
- Added matrix combine-with-self invariant test:
  - `pline_function_surface_closed_matrix_combine_with_self_cpp_parity`

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

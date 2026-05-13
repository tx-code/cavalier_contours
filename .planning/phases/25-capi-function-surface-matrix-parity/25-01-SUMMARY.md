---
phase: 25-capi-function-surface-matrix-parity
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, function-surface]
requires:
  - phase: 25-capi-function-surface-matrix-parity
    provides: function-surface parity target
provides:
  - executable C-API function-surface matrix parity tests
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [matrix-parity-bridge]
key-files:
  created:
    - .planning/phases/25-capi-function-surface-matrix-parity/25-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Use generated circle/half-circle matrices from old C++ formulas for C-API parity."
requirements-completed: [PAR-49]
duration: 9min
completed: 2026-05-14
---

# Plan 25-01 Summary

## Completed

- Added `pline_function_surface_circle_metrics_winding_cpp_matrix_parity`.
- Added `pline_function_surface_half_circle_metrics_winding_cpp_matrix_parity`.
- Added shared matrix helpers for case generation, expected extents, and winding
  probes in `cavalier_contours_ffi/tests/test_pline.rs`.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

---
phase: 34-capi-function-surface-parallel-offset-full-matrix-parity
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, function-surface, parallel-offset]
requires:
  - phase: 34-capi-function-surface-parallel-offset-full-matrix-parity
    provides: function-surface parallel-offset full matrix target
provides:
  - executable function-surface full-matrix parallel-offset C-API parity tests
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [offset-full-matrix-bridge]
key-files:
  created:
    - .planning/phases/34-capi-function-surface-parallel-offset-full-matrix-parity/34-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Apply closed-rotation vertex matching for closed polylines and exact-order matching for open polylines."
requirements-completed: [PAR-76, PAR-77]
duration: 14min
completed: 2026-05-14
---

# Plan 34-01 Summary

## Completed

- Added C-API offset output vertex extraction helper:
  - `run_parallel_offset_vertexes`
- Added closed-rotation/open-exact vertex matching helpers.
- Added full-matrix function-surface offset tests:
  - `pline_function_surface_circle_parallel_offset_cpp_matrix_parity`
  - `pline_function_surface_circle_collapsed_offset_cpp_matrix_parity`
  - `pline_function_surface_half_circle_parallel_offset_cpp_matrix_parity`
  - `pline_function_surface_half_circle_collapsed_offset_cpp_matrix_parity`

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

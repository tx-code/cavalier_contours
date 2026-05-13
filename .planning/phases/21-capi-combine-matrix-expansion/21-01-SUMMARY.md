---
phase: 21-capi-combine-matrix-expansion
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, boolean-matrix]
requires:
  - phase: 21-capi-combine-matrix-expansion
    provides: matrix coverage target
provides:
  - executable C-API combine matrix parity tests
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [property-set-matching]
key-files:
  created:
    - .planning/phases/21-capi-combine-matrix-expansion/21-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Use unordered property matching with absolute-area comparison for C++ parity alignment."
requirements-completed: [PAR-37, PAR-38]
duration: 20min
completed: 2026-05-13
---

# Plan 21-01 Summary

## Completed

- Added shared FFI property extraction/matching helpers:
  - `PlineProps`
  - `pline_props`, `plinelist_props`
  - `props_set_match_ignore_area_sign`
  - `run_boolean_props`
- Added `pline_boolean_circle_rectangle_cpp_matrix_parity` (4 operations).
- Added `pline_boolean_coincident_case2_cpp_matrix_parity` (5 operations,
  includes both exclude directions).

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

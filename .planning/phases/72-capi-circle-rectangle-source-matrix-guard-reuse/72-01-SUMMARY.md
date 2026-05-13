---
phase: 72-capi-circle-rectangle-source-matrix-guard-reuse
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, boolean, circle-rectangle, guard]
requires:
  - phase: 72-capi-circle-rectangle-source-matrix-guard-reuse
    provides: circle-rectangle source guard reuse scope
provides:
  - circle-rectangle default matrix source-mapping guard and operation-sequence reuse
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [source-backed-guard-reuse]
key-files:
  created:
    - .planning/phases/72-capi-circle-rectangle-source-matrix-guard-reuse/72-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Use one canonical circle-rectangle operation sequence constant and one shared source-mapping guard helper to prevent matrix drift."
requirements-completed: [PAR-190, PAR-191]
duration: 8min
completed: 2026-05-15
---

# Plan 72-01 Summary

## Completed

- Added `CPP_CIRCLE_RECT_SOURCE_MATRIX` and `CPP_CIRCLE_RECT_SOURCE_OPS`.
- Applied source-backed mapping guard to:
  - `pline_boolean_circle_rectangle_cpp_matrix_parity`
- Reused canonical operation sequence across:
  - `pline_boolean_does_not_modify_input_cpp_parity`
  - `pline_boolean_options_path_circle_rectangle_cpp_parity`
  - `pline_boolean_options_path_circle_rectangle_vertex_output_cpp_parity`
  - `pline_boolean_options_path_circle_rectangle_pos_equal_eps_matrix_cpp_parity`
  - `pline_boolean_options_path_circle_rectangle_does_not_modify_input_cpp_parity`

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.

---
phase: 15-half-circle-offset-and-collapse-matrix-parity
plan: 01
subsystem: parity-tests
tags: [cpp-parity, half-circle, offset-matrix]
requires:
  - phase: 15-half-circle-offset-and-collapse-matrix-parity
    provides: phase context and C++ source mapping
provides:
  - executable half-circle generated offset/collapse matrix parity tests
affects: [parity-tests]
tech-stack:
  added: []
  patterns: [generated-case matrix execution, vertex-level parity]
key-files:
  created:
    - .planning/phases/15-half-circle-offset-and-collapse-matrix-parity/15-01-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_pline_function_parity.rs
key-decisions:
  - "Mirror old C++ half-circle offset formulas for closed connection arcs and inward intersections."
requirements-completed: [PAR-19, PAR-20]
duration: 28min
completed: 2026-05-13
---

# Plan 15-01 Summary

## Completed

- Added generated half-circle matrix offset parity test:
  - `cpp_generated_half_circle_full_matrix_parallel_offset_parity`.
- Added generated half-circle matrix collapsed-offset parity test:
  - `cpp_generated_half_circle_full_matrix_collapsed_offset_parity`.
- Added helper logic mirroring old C++ formulas for:
  - closed outward connection arcs,
  - inward intersection replacement vertices and bulge values.

## Verification

- `cargo test -p cavalier_contours --test test_cpp_pline_function_parity -- --nocapture` - pass.

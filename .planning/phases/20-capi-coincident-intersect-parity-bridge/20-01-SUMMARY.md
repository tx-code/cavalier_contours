---
phase: 20-capi-coincident-intersect-parity-bridge
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, boolean]
requires:
  - phase: 20-capi-coincident-intersect-parity-bridge
    provides: C-API bridge target
provides:
  - executable FFI coincident intersect parity case
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [api-boundary-parity]
key-files:
  created:
    - .planning/phases/20-capi-coincident-intersect-parity-bridge/20-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Use operation=1 (`BooleanOp::And`) explicit FFI mapping for intersect."
requirements-completed: [PAR-34, PAR-35]
duration: 8min
completed: 2026-05-13
---

# Plan 20-01 Summary

## Completed

- Added `pline_boolean_coincident_case1_intersect_cpp_parity` to
  `cavalier_contours_ffi/tests/test_pline.rs`.
- Test uses old C++ coincident inputs and verifies empty `pos/neg` plinelists
  through `cavc_pline_boolean(..., operation=1, ...)`.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

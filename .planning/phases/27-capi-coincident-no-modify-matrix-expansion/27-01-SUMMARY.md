---
phase: 27-capi-coincident-no-modify-matrix-expansion
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, coincident, no-modify]
requires:
  - phase: 27-capi-coincident-no-modify-matrix-expansion
    provides: coincident no-modify target
provides:
  - executable C-API coincident no-modify matrix parity test
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [immutability-matrix-bridge]
key-files:
  created:
    - .planning/phases/27-capi-coincident-no-modify-matrix-expansion/27-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Use shared coincident input helpers to keep matrix variants source-traceable."
requirements-completed: [PAR-55, PAR-56]
duration: 8min
completed: 2026-05-14
---

# Plan 27-01 Summary

## Completed

- Added `cpp_coincident_case1_inputs` / `cpp_coincident_case2_inputs` helpers.
- Added `pline_boolean_coincident_matrices_do_not_modify_input_cpp_parity`.
- Test covers coincident case1/case2 full op matrix with explicit `A-B` and
  `B-A` exclusion direction checks.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

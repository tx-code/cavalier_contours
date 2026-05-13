---
phase: 17-cpp-coincident-combine-matrix-parity-expansion
plan: 01
subsystem: parity-tests
tags: [cpp-parity, combine, coincident-cases]
requires:
  - phase: 17-cpp-coincident-combine-matrix-parity-expansion
    provides: phase context and C++ source mapping
provides:
  - executable coincident combine matrix parity tests
affects: [parity-tests]
tech-stack:
  added: []
  patterns: [matrix parity import, geometry parity classification]
key-files:
  created:
    - .planning/phases/17-cpp-coincident-combine-matrix-parity-expansion/17-01-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_combine_parity.rs
key-decisions:
  - "Import C++ coincident case matrices and keep one bounded divergence explicit."
requirements-completed: [PAR-25, PAR-26]
duration: 22min
completed: 2026-05-13
---

# Plan 17-01 Summary

## Completed

- Expanded `test_cpp_combine_parity.rs` with source-traceable C++ coincident
  case matrices:
  - `coincident_case1_*`
  - `coincident_case2_*`
- Added executable parity loop over imported cases.
- Recorded bounded divergence handling for `coincident_case1_intersect`:
  Rust returns a tiny zero-area sliver while old C++ expected empty.

## Verification

- `cargo test -p cavalier_contours --test test_cpp_combine_parity -- --nocapture` - pass.


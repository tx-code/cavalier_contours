---
phase: 70-capi-coincident-case1-matrix-parity-expansion
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, boolean, coincident, matrix-expansion]
requires:
  - phase: 70-capi-coincident-case1-matrix-parity-expansion
    provides: explicit case1 matrix parity scope
provides:
  - full source-backed coincident_case1 default-path matrix parity test
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [source-backed-matrix-parity]
key-files:
  created:
    - .planning/phases/70-capi-coincident-case1-matrix-parity-expansion/70-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Use old C++ coincident_case1 expected property tuples directly in an explicit default-path matrix parity test."
requirements-completed: [PAR-184, PAR-185]
duration: 7min
completed: 2026-05-15
---

# Plan 70-01 Summary

## Completed

- Added explicit source-backed `coincident_case1` matrix parity test covering:
  - `union`
  - `excludeAFromB`
  - `excludeBFromA`
  - `intersect`
  - `xor`
- Preserved canonical operation mapping and reversed exclude subject/clip
  direction handling.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.

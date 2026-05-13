---
phase: 67-capi-coincident-exclude-name-canonicalization
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, boolean, coincident, naming]
requires:
  - phase: 67-capi-coincident-exclude-name-canonicalization
    provides: canonical coincident exclude naming scope
provides:
  - canonical old C++ coincident exclude labels in Rust matrix case metadata
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [source-name-canonicalization]
key-files:
  created:
    - .planning/phases/67-capi-coincident-exclude-name-canonicalization/67-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Use old C++ canonical exclude labels for coincident case1/case2 metadata across matrix suites."
requirements-completed: [PAR-175, PAR-176]
duration: 5min
completed: 2026-05-14
---

# Plan 67-01 Summary

## Completed

- Renamed coincident exclude case labels from snake-form to old C++ canonical
  form in matrix case metadata:
  - `coincident_case1_excludeAFromB`
  - `coincident_case1_excludeBFromA`
  - `coincident_case2_excludeAFromB`
  - `coincident_case2_excludeBFromA`
- Applied consistently across default/options/no-modify boolean matrix suites.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.

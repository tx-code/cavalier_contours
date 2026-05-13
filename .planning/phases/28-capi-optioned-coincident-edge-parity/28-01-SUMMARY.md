---
phase: 28-capi-optioned-coincident-edge-parity
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, coincident, options-path]
requires:
  - phase: 28-capi-optioned-coincident-edge-parity
    provides: optioned coincident edge target
provides:
  - executable C-API optioned coincident parity tests
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [optioned-edge-bridge]
key-files:
  created:
    - .planning/phases/28-capi-optioned-coincident-edge-parity/28-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Mirror Rust-core collapsed-area edge threshold (`1e-4`) in C-API optioned coincident intersect parity."
requirements-completed: [PAR-58, PAR-59]
duration: 9min
completed: 2026-05-14
---

# Plan 28-01 Summary

## Completed

- Added `pline_boolean_options_coincident_case1_intersect_collapsed_filter_cpp_parity`.
- Added `pline_boolean_options_coincident_matrices_do_not_modify_input_cpp_parity`.
- Options-path no-modify matrix covers coincident case1/case2 full op matrix,
  including `A-B` and `B-A` exclusion direction variants.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

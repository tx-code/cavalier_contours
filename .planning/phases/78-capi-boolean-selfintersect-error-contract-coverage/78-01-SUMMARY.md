---
phase: 78-capi-boolean-selfintersect-error-contract-coverage
plan: 01
subsystem: ffi-contract
tags: [ffi, error-codes, boolean, self-intersect]
requires:
  - phase: 78-capi-boolean-selfintersect-error-contract-coverage
    provides: direct boolean/self-intersect error contract coverage scope
provides:
  - direct boolean/self-intersect invalid-input contract coverage
affects: [ffi-runtime-docs, ffi-tests]
tech-stack:
  added: []
  patterns: [error-contract-hardening]
key-files:
  created:
    - .planning/phases/78-capi-boolean-selfintersect-error-contract-coverage/78-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/src/lib.rs
    - cavalier_contours_ffi.h
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Lock error-code behavior with direct assertions rather than relying on downstream side effects."
requirements-completed: [PAR-208, PAR-209]
duration: 8min
completed: 2026-05-15
---

# Plan 78-01 Summary

## Completed

- Added tests:
  - `pline_boolean_invalid_operation_error_ffi`
  - `pline_scan_for_self_intersect_invalid_options_error_ffi`
- Aligned self-intersect null-input docs from `pline1` to `pline` in runtime/header comments.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.
- `cargo clippy --all-targets -- -D warnings` - pass.

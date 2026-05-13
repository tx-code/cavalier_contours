---
phase: 80-capi-shape-polyline-accessor-invalid-input-contract-coverage
plan: 01
subsystem: ffi-contract
tags: [ffi, shape, polyline, error-codes]
requires:
  - phase: 80-capi-shape-polyline-accessor-invalid-input-contract-coverage
    provides: shape polyline accessor invalid-input contract scope
provides:
  - direct shape polyline accessor invalid-input contract coverage
affects: [ffi-runtime-docs, ffi-header-docs, ffi-tests]
tech-stack:
  added: []
  patterns: [error-contract-hardening]
key-files:
  created:
    - .planning/phases/80-capi-shape-polyline-accessor-invalid-input-contract-coverage/80-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/src/lib.rs
    - cavalier_contours_ffi.h
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Lock accessor failure-path semantics through direct boundary assertions instead of implicit behavior."
requirements-completed: [PAR-214, PAR-215]
duration: 8min
completed: 2026-05-15
---

# Plan 80-01 Summary

## Completed

- Added direct invalid-input contract test:
  - `shape_polyline_access_error_contracts_ffi`
- Aligned shape polyline accessor docs on parameter naming and ccw/cw wording.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.

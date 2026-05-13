---
phase: 77-capi-userdata-getter-bounds-contract-hardening
plan: 01
subsystem: ffi-contract
tags: [ffi, userdata, bounds, error-codes]
requires:
  - phase: 77-capi-userdata-getter-bounds-contract-hardening
    provides: userdata getter bounds hardening scope
provides:
  - explicit userdata getter bounds contract and aligned docs/tests
affects: [ffi-runtime, ffi-header, ffi-tests]
tech-stack:
  added: []
  patterns: [error-contract-hardening]
key-files:
  created:
    - .planning/phases/77-capi-userdata-getter-bounds-contract-hardening/77-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/src/lib.rs
    - cavalier_contours_ffi.h
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Replace implicit panic path with explicit bounds-return contract on userdata getters."
requirements-completed: [PAR-205, PAR-206]
duration: 8min
completed: 2026-05-15
---

# Plan 77-01 Summary

## Completed

- Added explicit bounds checks (`return 2`) to:
  - `cavc_shape_get_ccw_pline_userdata_values`
  - `cavc_shape_get_cw_pline_userdata_values`
- Updated FFI header error docs for both getter functions.
- Added CCW/CW userdata getter bounds assertions in existing direct setter tests.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.
- `cargo clippy --all-targets -- -D warnings` - pass.

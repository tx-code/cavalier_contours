---
phase: 30-capi-closest-point-parity-bridge
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, closest-point, abi]
requires:
  - phase: 30-capi-closest-point-parity-bridge
    provides: closest-point C-API target
provides:
  - closest-point C-API surface
  - executable closest-point parity checks at C-API boundary
affects: [ffi-api, ffi-tests, c-header]
tech-stack:
  added: []
  patterns: [ffi-surface-bridge]
key-files:
  created:
    - .planning/phases/30-capi-closest-point-parity-bridge/30-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/src/lib.rs
    - cavalier_contours_ffi/tests/test_pline.rs
    - cavalier_contours_ffi.h
key-decisions:
  - "Return error code 2 for empty polyline closest-point evaluation."
requirements-completed: [PAR-64, PAR-65]
duration: 14min
completed: 2026-05-14
---

# Plan 30-01 Summary

## Completed

- Added `cavc_pline_eval_closest_point` to FFI surface.
- Added closest-point error-path test (`null` and `empty polyline`) and
  source-backed circle closest-point matrix parity test.
- Regenerated `cavalier_contours_ffi.h` via `cbindgen`.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

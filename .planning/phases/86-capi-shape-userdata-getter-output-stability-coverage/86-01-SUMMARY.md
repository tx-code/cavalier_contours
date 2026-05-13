---
phase: 86-capi-shape-userdata-getter-output-stability-coverage
plan: 01
subsystem: ffi-contract
tags: [ffi, shape, userdata, getter, failure-path]
requires:
  - phase: 86-capi-shape-userdata-getter-output-stability-coverage
    provides: shape userdata getter failure-path stability scope
provides:
  - direct shape userdata getter output stability coverage
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [error-contract-hardening]
key-files:
  created:
    - .planning/phases/86-capi-shape-userdata-getter-output-stability-coverage/86-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Lock shape userdata getter failure-path contracts through explicit out-parameter sentinel checks to prevent accidental mutation regressions."
requirements-completed: [PAR-232, PAR-233]
duration: 8min
completed: 2026-05-15
---

# Plan 86-01 Summary

## Completed

- Added direct invalid-input/output-stability test:
  - `shape_userdata_getter_failure_path_output_stability_ffi`

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.

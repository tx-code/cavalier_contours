---
phase: 36-capi-pline-suite-buffer-reserve-parity
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, pline-suite, buffer-safety]
requires:
  - phase: 36-capi-pline-suite-buffer-reserve-parity
    provides: pline suite buffer/reserve target
provides:
  - executable pline-suite buffer/reserve C-API parity tests
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [buffer-no-write-reserve-no-modify]
key-files:
  created:
    - .planning/phases/36-capi-pline-suite-buffer-reserve-parity/36-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Pin source-backed pline-suite edge semantics using explicit sentinel-buffer and vertex persistence assertions."
requirements-completed: [PAR-82, PAR-83]
duration: 6min
completed: 2026-05-14
---

# Plan 36-01 Summary

## Completed

- Added test:
  - `pline_get_vertex_data_empty_does_not_modify_buffer_cpp_parity`
- Added test:
  - `pline_reserve_does_not_modify_existing_vertex_data_cpp_parity`

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

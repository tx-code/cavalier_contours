---
phase: 81-capi-shape-root-invalid-input-contract-coverage
plan: 01
subsystem: ffi-contract
tags: [ffi, shape, root, error-codes]
requires:
  - phase: 81-capi-shape-root-invalid-input-contract-coverage
    provides: shape-root invalid-input contract scope
provides:
  - direct shape-root invalid-input contract coverage
affects: [ffi-runtime-docs, ffi-header-docs, ffi-tests]
tech-stack:
  added: []
  patterns: [error-contract-hardening]
key-files:
  created:
    - .planning/phases/81-capi-shape-root-invalid-input-contract-coverage/81-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/src/lib.rs
    - cavalier_contours_ffi.h
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Lock root failure-path semantics through direct boundary assertions to prevent accidental output mutation regressions."
requirements-completed: [PAR-217, PAR-218]
duration: 8min
completed: 2026-05-15
---

# Plan 81-01 Summary

## Completed

- Added direct invalid-input contract test:
  - `shape_root_invalid_input_contracts_ffi`
- Aligned shape-surface runtime/header doc references from `cavc_pline_create` to
  `cavc_shape_create` for covered shape-root and adjacent shape accessors.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.

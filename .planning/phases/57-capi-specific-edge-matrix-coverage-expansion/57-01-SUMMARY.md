---
phase: 57-capi-specific-edge-matrix-coverage-expansion
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, options-path, specific-edge, coverage-expansion]
requires:
  - phase: 57-capi-specific-edge-matrix-coverage-expansion
    provides: matrix coverage expansion scope
provides:
  - expanded source-backed specific-edge matrix coverage
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [source-backed-edge-case-expansion]
key-files:
  created:
    - .planning/phases/57-capi-specific-edge-matrix-coverage-expansion/57-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Keep Phase 56 helper flow and expand source-backed matrix inputs with `closed_rectangle_coincident` plus explicit provenance attribution."
requirements-completed: [PAR-145, PAR-146]
duration: 6min
completed: 2026-05-14
---

# Plan 57-01 Summary

## Completed

- Added `cpp_offset_specific_edge_matrix_cases` to extend specific-edge matrix
  inputs with source-backed `closed_rectangle_coincident` from old C++
  `createSimpleCases`.
- Extended provenance mapping in `cpp_specific_edge_attribution` with:
  - `closed_rectangle_coincident` =>
    "old C++ simple edge case: closed rectangle offset inward into coincident line"
- Updated both helper-driven matrix tests to use
  `cpp_offset_specific_edge_matrix_cases`.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.










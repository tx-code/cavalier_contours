---
phase: 60-capi-specific-edge-matrix-open-diamond-expansion
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, options-path, specific-edge, open-diamond-expansion]
requires:
  - phase: 60-capi-specific-edge-matrix-open-diamond-expansion
    provides: open-diamond matrix expansion scope
provides:
  - expanded source-backed specific-edge matrix coverage with open-diamond case
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [source-backed-open-diamond-case-expansion]
key-files:
  created:
    - .planning/phases/60-capi-specific-edge-matrix-open-diamond-expansion/60-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Extend Phase 59 helper flow with source-backed `open_diamond_inward` and explicit provenance attribution."
requirements-completed: [PAR-154, PAR-155]
duration: 6min
completed: 2026-05-14
---

# Plan 60-01 Summary

## Completed

- Extended `cpp_offset_specific_edge_matrix_cases` with additional
  source-backed open-diamond case:
  - `open_diamond_inward`
- Extended provenance mapping in `cpp_specific_edge_attribution` with:
  - `open_diamond_inward` =>
    "old C++ simple case: open diamond offset inward"
- Updated both helper-driven matrix tests to use
  `cpp_offset_specific_edge_matrix_cases`.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.










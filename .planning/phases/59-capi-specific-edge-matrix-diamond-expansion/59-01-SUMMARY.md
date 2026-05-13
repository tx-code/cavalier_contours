---
phase: 59-capi-specific-edge-matrix-diamond-expansion
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, options-path, specific-edge, diamond-expansion]
requires:
  - phase: 59-capi-specific-edge-matrix-diamond-expansion
    provides: diamond matrix expansion scope
provides:
  - expanded source-backed specific-edge matrix coverage with diamond case
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [source-backed-diamond-case-expansion]
key-files:
  created:
    - .planning/phases/59-capi-specific-edge-matrix-diamond-expansion/59-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Extend Phase 58 helper flow with source-backed `closed_diamond_outward` and explicit provenance attribution."
requirements-completed: [PAR-151, PAR-152]
duration: 6min
completed: 2026-05-14
---

# Plan 59-01 Summary

## Completed

- Extended `cpp_offset_specific_edge_matrix_cases` with additional
  source-backed diamond case:
  - `closed_diamond_outward`
- Extended provenance mapping in `cpp_specific_edge_attribution` with:
  - `closed_diamond_outward` =>
    "old C++ simple case: closed diamond offset outward"
- Updated both helper-driven matrix tests to use
  `cpp_offset_specific_edge_matrix_cases`.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.










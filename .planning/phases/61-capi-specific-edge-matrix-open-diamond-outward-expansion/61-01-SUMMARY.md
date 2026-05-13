---
phase: 61-capi-specific-edge-matrix-open-diamond-outward-expansion
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, options-path, specific-edge, open-diamond-outward-expansion]
requires:
  - phase: 61-capi-specific-edge-matrix-open-diamond-outward-expansion
    provides: open-diamond-outward matrix expansion scope
provides:
  - expanded source-backed specific-edge matrix coverage with open-diamond-outward case
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [source-backed-open-diamond-outward-case-expansion]
key-files:
  created:
    - .planning/phases/61-capi-specific-edge-matrix-open-diamond-outward-expansion/61-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Extend Phase 60 helper flow with source-backed `open_diamond_outward` and explicit provenance attribution."
requirements-completed: [PAR-157, PAR-158]
duration: 6min
completed: 2026-05-14
---

# Plan 61-01 Summary

## Completed

- Extended `cpp_offset_specific_edge_matrix_cases` with additional
  source-backed open-diamond-outward case:
  - `open_diamond_outward`
- Extended provenance mapping in `cpp_specific_edge_attribution` with:
  - `open_diamond_outward` =>
    "old C++ simple case: open diamond offset outward"
- Updated both helper-driven matrix tests to use
  `cpp_offset_specific_edge_matrix_cases`.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.










---
phase: 58-capi-specific-edge-matrix-open-rectangle-expansion
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, options-path, specific-edge, open-path-expansion]
requires:
  - phase: 58-capi-specific-edge-matrix-open-rectangle-expansion
    provides: open-path matrix expansion scope
provides:
  - expanded source-backed specific-edge matrix coverage with open-path case
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [source-backed-open-path-case-expansion]
key-files:
  created:
    - .planning/phases/58-capi-specific-edge-matrix-open-rectangle-expansion/58-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Extend Phase 57 helper flow with source-backed `open_rectangle_outward` and explicit provenance attribution."
requirements-completed: [PAR-148, PAR-149]
duration: 6min
completed: 2026-05-14
---

# Plan 58-01 Summary

## Completed

- Extended `cpp_offset_specific_edge_matrix_cases` with additional
  source-backed open-path case:
  - `open_rectangle_outward`
- Extended provenance mapping in `cpp_specific_edge_attribution` with:
  - `open_rectangle_outward` =>
    "old C++ simple case: open rectangle offset outward"
- Updated both helper-driven matrix tests to use
  `cpp_offset_specific_edge_matrix_cases`.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.










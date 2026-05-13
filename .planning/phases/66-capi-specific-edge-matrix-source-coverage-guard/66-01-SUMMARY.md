---
phase: 66-capi-specific-edge-matrix-source-coverage-guard
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, options-path, specific-edge, source-coverage-guard]
requires:
  - phase: 66-capi-specific-edge-matrix-source-coverage-guard
    provides: specific-edge matrix source-coverage guard scope
provides:
  - source-backed simple-case coverage guard in matrix constructor
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [coverage-guard-assertion]
key-files:
  created:
    - .planning/phases/66-capi-specific-edge-matrix-source-coverage-guard/66-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "After Phase 65 source-backed case expansion, add a guardrail that fails when any source-backed simple case is omitted from specific-edge matrix construction."
requirements-completed: [PAR-172, PAR-173]
duration: 5min
completed: 2026-05-14
---

# Plan 66-01 Summary

## Completed

- Added a constructor-level guard in `cpp_offset_specific_edge_matrix_cases`:
  - asserts `simple_cases` is empty after the source-backed selection loop.
  - emits omitted case names in failure diagnostics.
- Preserved all existing helper-driven matrix execution paths and attribution
  diagnostics.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.

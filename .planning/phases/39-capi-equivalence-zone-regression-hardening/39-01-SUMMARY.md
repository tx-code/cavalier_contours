---
phase: 39-capi-equivalence-zone-regression-hardening
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, reserve, remove-sequence, regression]
requires:
  - phase: 39-capi-equivalence-zone-regression-hardening
    provides: equivalence-zone hardening scope
provides:
  - tighter reserve/remove source-backed regression assertions
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [equivalence-zone-hardening]
key-files:
  created:
    - .planning/phases/39-capi-equivalence-zone-regression-hardening/39-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Strengthen existing remove-sequence parity flow with final empty-buffer no-write assertion."
  - "Use reserve+append regression to guard API-evolution equivalence behavior without capacity introspection."
requirements-completed: [PAR-91, PAR-92]
duration: 6min
completed: 2026-05-14
---

# Plan 39-01 Summary

## Completed

- Added test:
  - `pline_reserve_equivalence_preserves_prefix_across_growth_and_append_cpp_parity`
- Strengthened test:
  - `pline_remove_sequence_equivalent_to_cpp_remove_range_parity`
  - Added final empty-state `cavc_pline_get_vertex_data` no-write buffer assertion.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.

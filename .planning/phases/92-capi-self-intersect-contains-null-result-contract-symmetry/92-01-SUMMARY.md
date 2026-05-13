---
phase: 92-capi-self-intersect-contains-null-result-contract-symmetry
plan: 01
subsystem: ffi-contract
tags: [ffi, self-intersect, contains, invalid-input]
requires:
  - phase: 92-capi-self-intersect-contains-null-result-contract-symmetry
    provides: self-intersect/contains null-result symmetry scope
provides:
  - direct null-result and default/options-path contract symmetry coverage
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [error-contract-hardening]
key-files:
  created:
    - .planning/phases/92-capi-self-intersect-contains-null-result-contract-symmetry/92-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Lock self-intersect default/options null-input symmetry and contains explicit-options null-result symmetry through direct assertions."
requirements-completed: [PAR-250, PAR-251]
duration: 6min
completed: 2026-05-15
---

# Plan 92-01 Summary

## Completed

- Extended `boolean_and_self_intersect_failure_path_output_stability_ffi` with default-options null-input assertions for `cavc_pline_scan_for_self_intersect`.
- Extended `pline_contains_invalid_input_result_contract_ffi` with explicit-options null-`pline2` + null-result-pointer invalid-input assertion.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.

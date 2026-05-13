---
phase: 71-capi-coincident-default-matrix-source-map-guard
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, boolean, coincident, guard]
requires:
  - phase: 71-capi-coincident-default-matrix-source-map-guard
    provides: explicit default matrix source-mapping guard scope
provides:
  - shared source-mapping guard helper reused by helper/default matrix suites
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [source-backed-guard-reuse]
key-files:
  created:
    - .planning/phases/71-capi-coincident-default-matrix-source-map-guard/71-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Unify source-backed case-count/name/operation guard diagnostics in one helper and reuse it across coincident matrix surfaces."
requirements-completed: [PAR-187, PAR-188]
duration: 8min
completed: 2026-05-15
---

# Plan 71-01 Summary

## Completed

- Added shared `assert_boolean_case_source_mapping` helper.
- Reused the helper in:
  - shared coincident helper matrix guard path
  - explicit `coincident_case1` default matrix parity test
  - explicit `coincident_case2` default matrix parity test
- Preserved existing expected property assertions and case execution flow.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.

---
phase: 73-capi-pline-core-suite-source-coverage-parity
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, pline-core, source-coverage]
requires:
  - phase: 73-capi-pline-core-suite-source-coverage-parity
    provides: pline core suite source-coverage parity scope
provides:
  - explicit pline core source-backed parity suite and coverage guard
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [source-backed-suite-parity]
key-files:
  created:
    - .planning/phases/73-capi-pline-core-suite-source-coverage-parity/73-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Represent old pline core suite as explicit parity coverage with one guardable source-case list."
requirements-completed: [PAR-193, PAR-194]
duration: 9min
completed: 2026-05-15
---

# Plan 73-01 Summary

## Completed

- Added source-backed pline core case list:
  - `CPP_PLINE_CORE_SOURCE_CASES`
- Added reusable source-case coverage guard:
  - `assert_source_case_coverage`
- Added explicit source-backed parity suite:
  - `pline_core_suite_cpp_parity`
  - mirrors `new`, `set_capacity`-equivalent reserve, `set_vertex_data`,
    `add_vertex`, `remove_range`-equivalent remove sequence, and `clear`.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -q` - pass.

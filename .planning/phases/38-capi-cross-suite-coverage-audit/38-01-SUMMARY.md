---
phase: 38-capi-cross-suite-coverage-audit
plan: 01
subsystem: coverage-audit
tags: [ffi-parity, audit, cross-suite]
requires:
  - phase: 38-capi-cross-suite-coverage-audit
    provides: cross-suite source mapping target
provides:
  - old-C++ to FFI coverage checklist
affects: [planning-audit]
tech-stack:
  added: []
  patterns: [coverage-checklist]
key-files:
  created:
    - .planning/phases/38-capi-cross-suite-coverage-audit/38-CROSS-SUITE-COVERAGE-CHECKLIST.md
    - .planning/phases/38-capi-cross-suite-coverage-audit/38-01-SUMMARY.md
key-decisions:
  - "Classify each old suite block as covered/equivalent/gap using concrete FFI test references."
requirements-completed: [PAR-88, PAR-89]
duration: 8min
completed: 2026-05-14
---

# Plan 38-01 Summary

## Completed

- Built cross-suite coverage matrix mapping old C++ blocks to current FFI tests.
- Classified residual coverage status and follow-up boundaries.

## Verification

- `Select-String -Path .planning\phases\38-capi-cross-suite-coverage-audit\38-CROSS-SUITE-COVERAGE-CHECKLIST.md -Pattern "TEST_cavc_pline","TEST_cavc_pline_function","TEST_cavc_parallel_offset","TEST_cavc_combine_plines","covered","equivalent","gaps"` - pass.

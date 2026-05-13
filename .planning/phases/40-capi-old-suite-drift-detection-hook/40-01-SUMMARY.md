---
phase: 40-capi-old-suite-drift-detection-hook
plan: 01
subsystem: planning-tooling
tags: [cpp-parity, drift-detection, baseline, hook]
requires:
  - phase: 40-capi-old-suite-drift-detection-hook
    provides: drift detection target
provides:
  - executable old-suite drift hook with canonical baseline
affects: [planning-tools]
tech-stack:
  added: [powershell]
  patterns: [hash-and-test-block-drift-check]
key-files:
  created:
    - .planning/phases/40-capi-old-suite-drift-detection-hook/40-01-SUMMARY.md
    - .planning/tools/cpp_suite_drift_baseline.json
    - .planning/tools/cpp_suite_drift_check.ps1
  modified: []
key-decisions:
  - "Use regex extraction over TEST/TEST_F/TEST_P blocks to track suite structure drift."
requirements-completed: [PAR-94, PAR-95]
duration: 7min
completed: 2026-05-14
---

# Plan 40-01 Summary

## Completed

- Added canonical baseline:
  - `.planning/tools/cpp_suite_drift_baseline.json`
- Added drift hook script:
  - `.planning/tools/cpp_suite_drift_check.ps1`

## Verification

- `powershell -ExecutionPolicy Bypass -File .planning/tools/cpp_suite_drift_check.ps1` - pass.

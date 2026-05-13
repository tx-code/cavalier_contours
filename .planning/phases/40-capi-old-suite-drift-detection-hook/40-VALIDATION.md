---
phase: 40
slug: capi-old-suite-drift-detection-hook
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 40 Validation Strategy

## Core Gates

- `cargo test --workspace -q`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours`
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`

## Task Verification Map

| Task ID | Plan | Requirement | Command | Status |
|---------|------|-------------|---------|--------|
| 40-01-01 | 40-01 | PAR-94,PAR-95 | `powershell -ExecutionPolicy Bypass -File .planning/tools/cpp_suite_drift_check.ps1` | pass |
| 40-02-01 | 40-02 | PAR-96 | `Select-String -Path .planning\phases\40-capi-old-suite-drift-detection-hook\40-CPP-SUITE-DRIFT-HOOK.md -Pattern "Command","Pass/Fail Semantics","Failure Handling"` | pass |
| 40-02-02 | 40-02 | PAR-96 | `Select-String -Path .planning\phases\40-capi-old-suite-drift-detection-hook\40-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Hook Outcome","Next Alignment Targets","Decision Boundary"` | pass |
| 40-03-01 | 40-03 | PAR-94,PAR-95,PAR-96 | `cargo test --workspace -q` | pass |
| 40-03-02 | 40-03 | PAR-94,PAR-95,PAR-96 | `cargo fmt --all --check` | pass |
| 40-03-03 | 40-03 | PAR-94,PAR-95,PAR-96 | `cargo clippy --all-targets -- -D warnings` | pass |
| 40-03-04 | 40-03 | PAR-94,PAR-95,PAR-96 | `git diff --check` | pass |
| 40-03-05 | 40-03 | PAR-94,PAR-95,PAR-96 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 40-03-06 | 40-03 | PAR-94,PAR-95,PAR-96 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |

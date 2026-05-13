---
phase: 43
slug: capi-drift-failure-triage-template
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 43 Validation Strategy

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
| 43-01-01 | 43-01 | PAR-103,PAR-104 | `Select-String -Path .planning\tools\cpp_suite_drift_triage_template.md -Pattern "Drift Snapshot","Old-Suite Block Mapping","Classification","Action Decision","Closure Checklist"` | pass |
| 43-02-01 | 43-02 | PAR-105 | `Select-String -Path .planning\phases\43-capi-drift-failure-triage-template\43-CPP-SUITE-DRIFT-TRIAGE-FLOW.md -Pattern "Trigger","Deterministic Flow","Re-Audit Boundary"` | pass |
| 43-02-02 | 43-02 | PAR-105 | `Select-String -Path .planning\phases\43-capi-drift-failure-triage-template\43-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Triage Outcome","Next Alignment Targets","Decision Boundary"` | pass |
| 43-03-01 | 43-03 | PAR-103,PAR-104,PAR-105 | `cargo test --workspace -q` | pass |
| 43-03-02 | 43-03 | PAR-103,PAR-104,PAR-105 | `cargo fmt --all --check` | pass |
| 43-03-03 | 43-03 | PAR-103,PAR-104,PAR-105 | `cargo clippy --all-targets -- -D warnings` | pass |
| 43-03-04 | 43-03 | PAR-103,PAR-104,PAR-105 | `git diff --check` | pass |
| 43-03-05 | 43-03 | PAR-103,PAR-104,PAR-105 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 43-03-06 | 43-03 | PAR-103,PAR-104,PAR-105 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |

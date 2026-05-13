---
phase: 38
slug: capi-cross-suite-coverage-audit
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 38 Validation Strategy

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
| 38-01-01 | 38-01 | PAR-88,PAR-89 | `Select-String -Path .planning\phases\38-capi-cross-suite-coverage-audit\38-CROSS-SUITE-COVERAGE-CHECKLIST.md -Pattern "TEST_cavc_pline","TEST_cavc_pline_function","TEST_cavc_parallel_offset","TEST_cavc_combine_plines","covered","equivalent","gaps"` | pass |
| 38-02-01 | 38-02 | PAR-90 | `Select-String -Path .planning\phases\38-capi-cross-suite-coverage-audit\38-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Audit Outcome","Priority","Decision","File-Level"` | pass |
| 38-03-01 | 38-03 | PAR-88,PAR-89,PAR-90 | `cargo test --workspace -q` | pass |
| 38-03-02 | 38-03 | PAR-88,PAR-89,PAR-90 | `cargo fmt --all --check` | pass |
| 38-03-03 | 38-03 | PAR-88,PAR-89,PAR-90 | `cargo clippy --all-targets -- -D warnings` | pass |
| 38-03-04 | 38-03 | PAR-88,PAR-89,PAR-90 | `git diff --check` | pass |
| 38-03-05 | 38-03 | PAR-88,PAR-89,PAR-90 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 38-03-06 | 38-03 | PAR-88,PAR-89,PAR-90 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
